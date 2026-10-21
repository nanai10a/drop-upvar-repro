# (no idea for title)

## Run

Type these commands:

```sh
# they output `1`
cargo run --features ss
cargo run --features as

# some problems'll be happen here
cargo run --features sa
cargo run --features aa
```

and also can use Miri,

```sh
# (no attention needed to "ss" and "as")
cargo miri run --features sa
cargo miri run --features aa
```

### Run minimal edit.

Type a command:

```sh
cargo run --features sa,minimal
cargo miri run --features sa,minimal
```

Then I'd got:

```
repro--rust-lang--rust--163506(55558,0x1ef057f80) malloc: *** error for object 0x16b4628f0: pointer being freed was not allocated
repro--rust-lang--rust--163506(55558,0x1ef057f80) malloc: *** set a breakpoint in malloc_error_break to debug
```

```
# no outputs
```

## Current diagnostics

TL;DR - Currently Miri handles shim\[s\] correctly, so **this problem couldn't reproduce on it**.
This problem only happens on outside of Miri, executing codegen-ed instructions.

On minimal edit., `fn main` polls the same future twice, because if poll only once, someone doesn't validate call-stack soundness so we can't know what happened.

---

On runned natively with `sa`, I'd got:

```
[  no drop   ] ok - size `1`
[ has drop   ] ng - inconsistent size, expects `96` but actually is `48`
[(has drop)^2] ng - inconsistent size, expects `112` but actually is `48`
repro--rust-lang--rust--163506(54948,0x1ef057f80) malloc: *** error for object 0x10700007e4e0: pointer being freed was not allocated
repro--rust-lang--rust--163506(54948,0x1ef057f80) malloc: *** set a breakpoint in malloc_error_break to debug
```

And, on runned natively with `aa`, I'd got:

```
[  no drop   ] ok - size `1`
[ has drop   ] ok - size `96`
[(has drop)^2] ok - size `112`
[(has drop)^3] ok - size `128`
```

But if runned on Miri with `sa`, I'd got:

```
[  no drop   ] ok - size `1`
[ has drop   ] ok - size `48`
[(has drop)^2] ok - size `48`
[(has drop)^3] ok - size `48`
```

And, with `aa`, respectively, I'd got:

```
[  no drop   ] ok - size `1`
[ has drop   ] ok - size `96`
[(has drop)^2] ok - size `112`
[(has drop)^3] ok - size `128`
```

Of course Miri didn't emit any warnings or errors excepts `incomplete_features` warning of rustc.
