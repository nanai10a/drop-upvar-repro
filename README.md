# async_drop unsoundness (reproduces on latest nightly, `c1070d693` 2026-09-28)

`#![feature(async_drop)]` miscompiles an async fn: the future it creates
does not store the caller's `&mut` argument, and polling it misbehaves
(`resumed after completion`, wild writes, abort).

## Toolchain

`rustc 1.101.0-nightly (c1070d693 2026-09-28)` (latest at time of
writing). `rust-toolchain.toml` pins `channel = "nightly"` (floating).

History: first observed on `1.100.0-nightly (5ceaf6608 2026-09-25)`;
`nightly-2026-09-25` from rustup resolved to a *different* revision
(`f7575a9da 2026-09-24`) which does **not** reproduce — the bug is
still present on latest, so it is not a closed window.

## Layout (single package, no workspace, no dependencies)

- `src/lib.rs` — the feature gate + `Link::tick` (20 lines).
- `src/main.rs` — `check()` (byte-scan assertion), `main()`, and the test
  in `#[cfg(test)] mod tests` at the crate root.

The lib/bin split is load-bearing, not organization: the bug reproduces
only when the future is created across a crate boundary (bin using the
rlib). A self-contained single-crate binary yields a correct 248-byte
future and passes. See below.

## Run

```
cargo run
cargo test
```

Expected (bug present): `tick future size = 160`,
`link address 0x... not found in 160-byte future`, assertion FAILED.
Control: comment out the feature gate in `src/lib.rs` and both pass
(future is 40 bytes and contains the address).

## What the data shows

- With the gate, the tick future is 160 bytes and a full-byte scan finds
  no trace of the `&mut Link` upvar — even though MIR builds the coroutine
  as `{ self: move _1 }`.
- Without the gate, the same source yields a 40-byte future whose first
  8 bytes are the upvar.
- MIR (`-Zunpretty=mir`) of the broken build shows a coroutine layout with
  6 phantom `impl Future<Output = ()>` fields, an extra `ResumeTy` field,
  and 11 suspend variants (vs 1 without the gate) — the async-drop
  transform's drop-scope bookkeeping. Upvar storage appears to be lost
  when that layout is built.
- Trigger (all required simultaneously, found by bisection from a
  1008-byte real-world future):
  - a shared borrow of a pre-await local (`let _p = &_s;`), and
  - an `OsString` local (`OsString::from`, `PathBuf`, `Cow`, `current_exe()`
    chains all trigger; `String`/`Vec` do not).
- The same source used inside the defining crate yields a correct future
  (`cargo run` on a self-contained bin: 248 bytes, ok), but used across
  the rlib boundary it loses the upvar (160 bytes, FAILED) — with
  identical MIR layout dumps on both sides. The rlib metadata layout (or
  the downstream evaluation of it) disagrees with the defining crate's.

## Origin

Bisected from `epoche`'s `Link::tick` (tokio `AsyncFd`), where the lost
upvar surfaced as garbage-pointer derefs inside tokio's reactor
(`ScheduledIo::Readiness::poll`), `SIGSEGV`, and `resumed after
completion` panics. tokio is not required to reproduce.

## Upstream context

- MCP: rust-lang/compiler-team#727 "Low level components for async
  drop" (CLOSED).
- Tracking: rust-lang/rust#126482 "Tracking Issue for async drop
  codegen" (OPEN). Implementation history: #121801 (`AsyncDrop` trait),
  #124662, #123948, plus follow-up `F-async_drop` fixes (incl. #156649).
- Codegen implementation: rust-lang/rust#123948 "Async drop codegen"
  (MERGED). Core srctree (current main):
  - `compiler/rustc_mir_transform/src/coroutine/` (`mod.rs`,
    `layout.rs`, `drop.rs`, `by_move_body.rs`)
  - `compiler/rustc_mir_dataflow/src/elaborate_drops.rs`
  - `compiler/rustc_mir_build/src/builder/scope.rs`
  - `library/core/src/future/async_drop.rs`
- Design note:
  https://github.com/azhogin/posts/blob/main/async-drop-impl.md
  (drop terminators expanded to yield poll-loops in `coroutine.rs`,
  async drop shims, `future_drop_poll` lang item).

## MIR evidence (`-Zunpretty=mir`, dumps in `/tmp/mir-min-{bad,good}.txt`)

Gate on (`/tmp/mir-min-bad.txt`, `tick::{closure#0}`):

- The constructor still passes the upvar:
  `_0 = {coroutine} { self: move _1 }`.
- But the layout field list has **no `&mut Link` slot**:
  `_s0: Result<(), ()>` (return slot), `_s1: ResumeTy`, `_s2: u8`,
  `_s3: OsString`, `_s4: Vec<u8>`, `_s5`, `_s6: pend_res future`,
  `_s7.._s12: impl Future<Output = ()>` x6 (phantoms from the
  async-drop transform).
- `Unresumed(0)` is empty, yet `debug self => ((*_54).0: &mut Link)`
  claims the upvar lives at variant field 0 — a slot that does not
  exist. Codegen trusts this and reads/writes garbage at offset 0.
  This is the garbage pointer seen downstream (tokio reactor abort,
  `resumed after completion`).
- `storage_conflicts` is a near-dense matrix: the return slot `_s0`
  and `ResumeTy` `_s1` conflict with **every** field, and the future
  grows 40 -> 160 bytes.
- 11 suspend variants (`Suspend0..Suspend10`) vs 1 without the gate.

Gate off (`/tmp/mir-min-good.txt`): 3 fields (`u8`, `OsString`,
`pend_res` future), 1 suspend variant, 40-byte future holding the
upvar at offset 0.

## Relation to prior report #142572

`rust-lang/rust#142572` ("segmentation fault when using async_drop
feature", CLOSED) is the same bug family: a two-crate split (gate on
in the lib, gate off in the bin) producing colliding codegen, with a
dependency-free tokio reproducer. Discussion there (petrochenkov,
azhogin, RalfJung, oli-obk) identified the design flaw: codegen
consulting the *current* crate's feature gate instead of a marker on
the type. Fixed by #153274 ("Fix async drop multi crate crash",
merged 2026-03-04, adds `async-drop-run-without-feature` UI test) —
but that fix covers types that **implement** `AsyncDrop`. This repro
shows the same class of breakage with **no `AsyncDrop` impl at all**:
the gate alone is enough to desync the layouts.

## Regression: bisected to #123948

Backward verification (same 2-crate repro, `cargo test`):

| nightly | rustc | result |
|---|---|---|
| 2026-09-28 (latest) | `c1070d693` | broken (160B, upvar lost) |
| 2026-09-20..2025-06-30 | various | broken (160B; 176B on 2025-12-31; 240B on 2025-06-30) |
| 2025-04-29 | `25cdf1f67` (2025-04-28) | broken: 240B FAIL in release; **ICE** in dev (`build_coroutine_di_node`, debuginfo chokes on the broken layout) |
| 2025-04-27 | `10fa3c449` (2025-04-26) | clean PASS |
| 2025-01-01 | `d117b7f21` (2024-12-31) | clean PASS |

Between `10fa3c449` and `25cdf1f67` (118 commits) the only async-related
change is the merge of #123948 itself (`7d65abfe8`, "Auto merge of
#123948 - azhogin:azhogin/async-drop", plus its `c366756a8` async-drop
shim commit). **The bug was introduced on day one by the async-drop
codegen (#123948, merged 2025-04-28)** and never fixed for the
no-`AsyncDrop`-impl case. #153274 (2026-03-04) fixed the multi-crate
crash only for types that implement `AsyncDrop`.

(Note: an early "clean on 09-24 nightly" observation was a measurement
artifact of an older single-crate layout — the current 2-crate repro
fails on every toolchain from 2025-04-28 to latest.)
