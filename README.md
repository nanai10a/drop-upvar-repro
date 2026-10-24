# (no idea for title)

this repository is made for research a unsound behavior which's caused by rustc with `async_drop` feature flag.
these crates require toolchain on nightly channel for compiling programs with enabling `async_drop` feature flag.

in this repository, crates have some feature flags:

- `async-drop` / `be-depended/async-drop` ... enable `async_drop` feature in that crate.
- `aa` / `as` / `sa` / `ss` ... are shorthands to enable `async_drop` feature on root crate and `be-depended` crate, with `(enable, enable)`, `(enable, disable)`, respectively.
<!-- diviner --->
- `extra` / `be-depended/extra` ... add items more higher-recursiveness
- `minimal` ... switches entrypoint to another one, which has no extra output and procedure, and it's also for showing on upstream repository (`rust-lang/rust`).

## Reproduce behavior

the problem, which i research to, is happened on between `sa` and `aa`.
in other words, usage of some types, which are carried across crates from where `async_drop` is enabled, might be wrong.

to observe the standard behavior, run:

```sh
cargo run --features ss
```

then got:

```
[  no drop   ] ok - size `1`
[  no drop   ] begin to execute future ... ... completed.
[ has drop   ] ok - size `1`
[ has drop   ] begin to execute future ... {<IsDrop as Drop>::drop} {<IsAnotherDrop as Drop>::drop} ... completed.
[(has drop)^2] ok - size `1`
[(has drop)^2] begin to execute future ... {<IsDrop as Drop>::drop} {<IsAnotherDrop as Drop>::drop} ... completed.
[(has drop)^3] ok - size `1`
[(has drop)^3] begin to execute future ... {<IsDrop as Drop>::drop} {<IsAnotherDrop as Drop>::drop} ... completed.
```

(and this behavior is same as `as`)

but if enable on `be-depended` side,

```sh
cargo run --features sa
```

then got:

```
[  no drop   ] ok - size `1`
[  no drop   ] begin to execute future ... ... completed.
[ has drop   ] ng - inconsistent size, expects `40` but actually is `24`
[ has drop   ] begin to execute future ... {<IsDrop as Drop>::drop} {<IsAnotherDrop as Drop>::drop} ... completed.
[(has drop)^2] ng - inconsistent size, expects `56` but actually is `24`
[(has drop)^2] begin to execute future ... ... completed.
[(has drop)^3] ng - inconsistent size, expects `72` but actually is `24`
[(has drop)^3] begin to execute future ... {<IsDrop as Drop>::drop} {<IsAnotherDrop as Drop>::drop} ... completed.
```

seems something wrong.

and if enable on both side,

```sh
cargo run --features aa
```

then got:

```
[  no drop   ] ok - size `1`
[  no drop   ] begin to execute future ... ... completed.
[ has drop   ] ok - size `40`
[ has drop   ] begin to execute future ... {<IsDrop as Drop>::drop} {<IsAnotherDrop as Drop>::drop} ... completed.
[(has drop)^2] ok - size `56`
[(has drop)^2] begin to execute future ... {<IsDrop as Drop>::drop} {<IsAnotherDrop as Drop>::drop} ... completed.
[(has drop)^3] ok - size `72`
[(has drop)^3] begin to execute future ... {<IsDrop as Drop>::drop} {<IsAnotherDrop as Drop>::drop} ... completed.
```

seems normal.

if want to observe a more broken behavior, then run with `extra`:

```sh
cargo run --features sa,extra
```

then got:

```
[  no drop   ] ok - size `1`
[  no drop   ] begin to execute future ... ... completed.
[ has drop   ] ng - inconsistent size, expects `40` but actually is `24`
[ has drop   ] begin to execute future ... {<IsDrop as Drop>::drop} {<IsAnotherDrop as Drop>::drop} ... completed.
[(has drop)^2] ng - inconsistent size, expects `56` but actually is `24`
[(has drop)^2] begin to execute future ... {<IsDrop as Drop>::drop} {<IsAnotherDrop as Drop>::drop} ... completed.
[(has drop)^3] ng - inconsistent size, expects `72` but actually is `24`

thread 'main' (919244) panicked at /Users/nanai/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/future/async_drop.rs:55:70:
`async fn` resumed after panicking
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[(has drop)^3] begin to execute future ... %
```

(`%` symbol indicates no newline on EOF) so anyway, `sa` pattern seems something wrong.

## If Miri is used

execution commands described before, could be replace with `cargo miri run`, but Miri doesn't reproduce these behavior.
if run:

```sh
cargo miri run --features sa,extra
```

then got:

```
[  no drop   ] ok - size `1`
[  no drop   ] begin to execute future ... ... completed.
[ has drop   ] ok - size `24`
[ has drop   ] begin to execute future ... {<IsDrop as Drop>::drop} {<IsAnotherDrop as Drop>::drop} ... completed.
[(has drop)^2] ok - size `24`
[(has drop)^2] begin to execute future ... {<IsDrop as Drop>::drop} {<IsAnotherDrop as Drop>::drop} ... completed.
[(has drop)^3] ok - size `24`
[(has drop)^3] begin to execute future ... {<IsDrop as Drop>::drop} {<IsAnotherDrop as Drop>::drop} ... completed.
[(has drop)^4] ok - size `24`
[(has drop)^4] begin to execute future ... {<IsDrop as Drop>::drop} {<IsAnotherDrop as Drop>::drop} ... completed.
```

Miri seems that overwrites shim generated on `be-depended`, to another shim generated on root crate. this time, root crate is disabled `async_drop`, so `be-depended` shim is ignored on runtime and doesn't perform async-drop, alternate to normal drop. this is the reason of future size is small. of course this layout / size calculation is right in some reasons, but also inconsistent to actual behavior.

## Investigate weird behaviors

honestly idk a reason in detailed, but:

- basically ADT (like `IsDrop` and `HasDrop` etc.) is ZST, so these types have no data and may represent as nullptr
- but Coroutine, is desugared form of `async fn`, is not ZST and contains many of data
- and also `&mut dyn Future<...>` is used in this program, that means, rustc generates vtable for it. `dyn ...` fat pointer references vtable and it contains (1.) drop\_glue (2.) actual size of data (3.) alignment size (4.) trait vtable. so this program seems used only ZSTs, but rustc emits many of data for handling async context and dynamic dispatching. so some region, relate for async processing, on memory might be corrupted by some instructions.
- i couldn't explain what's happen in executable, but i can explain only this: rustc emits different layout for same types, so this brokes guarantee of memory safety, and also it's unsound. default entrypoint prints some diagnostics about these types, but also  `-Zprint-type-sizes` and more flags ensure to dump about a compilation from rustc, without printing on inside of program
