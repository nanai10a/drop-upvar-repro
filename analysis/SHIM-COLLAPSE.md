# Shim collapse drill-down (t_e36a954c)

Source captures (this repo, V16 repro, `analysis/`): `pts-lib.txt` (gate
ON, 248B), `pts-bin.txt` (gate-off consumer re-layout from rlib, 160B),
`mir-lib.txt` (gate-ON MIR, ground truth for drop scopes),
`offsets.py` (offset-table script). No rustc build. Offsets below are
prefix sums over `-Zprint-type-sizes` listing order (the listing carries
no explicit offsets except `_s0`-style field bases). Machine-checked by
`offsets.py`: on most variants item sums match variant sizes within
+-1B (tail discriminant noise), but on V16 Suspend3/5/7 the items exceed
the variant size by 31B — fields overlap there, so prefix sums are
_claimed_ positions (where the compiler thinks `.self` lives), an upper
bound on the true DWARF offset, not exact addresses. The conclusion does
not rest on them: the lib-side 248B runtime probe FINDS the address and
the bin-side 160B scan does NOT, so the slot is absent from the 160B
object regardless of where the listing claims it.

## 1. Which shim state machines collapse

Three `async_drop_in_place` instantiations exist in the MIR (bb25/bb32/bb38/bb44
assign them to variant fields; V16 numbering: pend-res shim
`coroutine_field6`, Vec shim `coroutine_field7`, OsString shim
`coroutine_field8/9/11`):

- `async_drop_in_place::<OsString>` — COLLAPSES. Lib: 48B with
  `Suspend0`/`Suspend1` (each: upvar `._to_drop` @0 + 32B
  `async_drop_in_place::<Buf>` child) + Unresumed/Returned/Panicked.
  Bin (`analysis/pts-bin.txt:234`): 16B pure leaf — Unresumed/Returned/Panicked only
  (`._to_drop` 8B + 1B discriminant + 7B tail pad). Both suspend states and
  the Buf child are gone. Embedded in tick as `coroutine_field8` (S5/S6),
  `coroutine_field9` (S7/S8), `coroutine_field11` (S10): 48 -> 16 each.
- `async_drop_in_place::<Buf>` (`std::sys::os_str::bytes::Buf`) — VANISHES.
  Lib: 32B with `Suspend0`/`Suspend1` (each: `._to_drop` @0 + 16B
  `async_drop_in_place::<Vec<u8>>` child). Bin: zero entries for the shim
  type (only the plain 24B `Buf` storage type remains at
  `analysis/pts-bin.txt:163`).
  Its sole parent was the OsString shim's suspend states; once those collapse,
  it drops out of the bin-side monomorphization set entirely.
- Unchanged (leaves in BOTH files, no Suspend states anywhere):
  `async_drop_in_place::<Vec<u8>>` 16B (`coroutine_field7`, S3/S4) and
  `async_drop_in_place::<pend_res-future>` 16B (`coroutine_field6`, S0/S1/S2/S9;
  bin S9 renumbers it `coroutine_field10`). Their drops contain no nested async-drop work, so
  there was never anything to collapse.

Chain view: OsString-drop -> Buf-drop -> Vec-drop. The collapse cuts the chain
at the top two links; the leaf (Vec) is untouched.

## 2. Upvar slot offsets before/after

`.self` (`&mut Link`, 8B) prefix-sum offset per variant:

| variant          | lib | bin | delta |
|------------------|-----|-----|-------|
| Unresumed        | 183 | 119 | -64   |
| Suspend0         | 183 | 119 | -64   |
| Suspend1/2       | 184 | 120 | -64   |
| Suspend3/4       | 183 | 119 | -64   |
| Suspend5/6       | 183 | 119 | -64   |
| Suspend7/8       | 183 | 119 | -64   |
| Suspend9/10      | 183 | 119 | -64   |
| Returned/Panicked| 183 | 119 | -64   |

Uniform -64B shift of the _claimed_ positions in every variant. Lib
claimed slot bytes 183..191: `183 + 8 = 191 > 160`, i.e. even the
compiler's own claimed position lies ENTIRELY outside the 160B window
the gate-off consumer computes. A bin-side byte scan over `size_of_val`
(160B) can never observe the address — this is the "off the window"
mechanism. (S1/S2 sit 1B higher in both layouts because the leading 16B
`coroutine_field6` @0 shifts 8-aligned packing; same relative shift.)
Note (V16): in Suspend3/5/7 the listing items overlap (sums exceed sizes
by 31B), so 183/119 there are prefix-sum claims, not DWARF-measured
addresses. This only weakens the illustration, not the mechanism: the
runtime scans prove absence directly (lib 248B FOUND vs bin 160B NOT
FOUND), and §3 shows the listing cannot account for the layouts anyway.

Max-variant switch behind 248 -> 160 (-88): lib max is Suspend10 (247 + 1B
disc = 248); bin max is Suspend2 (159 + 1B = 160). Per-variant decomposition:

- S5/S6: shim -32 (48->16) + padding -32 (pad 31+56+24=111 -> 31+24+24=79).
- S7/S8: shim -32 + padding -32 (pad 79+8+24=111 -> 47+8+24=79).
- S10: shim -32 + padding -64 (135+24+8=167 -> 71+24+8=103); 247 -> 151.
- S1/S2/S3/S4/S9/Unresumed/S0/Returned/Panicked: NO named-field change at
  all (e.g. S1 named multiset {16,24,8,1,1} identical); entire -64 is
  padding shrinkage (S1 interior pad 120 -> 56; Unresumed leading pad
  183 -> 119).

## 3. Caveat: 64B/variant of unattributed state

Only 32B of the -88B is attributable to named shim fields; the remaining -56B
(net of the max-variant switch) is padding. The S1/Unresumed pairs prove the
gate-ON layout carries 64B/variant of state invisible to `-Zprint-type-sizes`:
identical named field sets cannot lay out differently under per-variant
packing, so either there are hidden gate-ON coroutine fields folded into
"padding" lines, or the packing is global (shared field arena compacted by the
shim shrink). Exact nature needs rustc-source or DWARF follow-up — flagged,
not guessed. Either way it does not move the conclusion: `.self` @183 is
outside any 160B prefix.

## 4. Relation to main V16 (ea78c6a)

Captures and offsets above were re-taken natively on the minimized
V16 repro (OsString-only, no borrow): totals (248/160) and the full
`.self` offset table (lib 183/184, bin 119/120, uniform -64) reproduce
exactly, so the collapse set (OsString + Buf shims; Vec/pend leaves)
carries over unchanged.
