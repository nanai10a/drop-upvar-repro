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

## Current diagnostics

TL;DR - Currently Miri handles shim\[s\] correctly, so **this problem couldn't reproduce on it**.
This problem only happens on outside of Miri, executing codegen-ed instructions.

---

On runned natively with `sa`, I'd got:

```
[  no drop   ] ok - size `1`
[ has drop   ] ok - size `24`
[(has drop)^2] ng - inconsistent size, expects `40` but actually is `24`
[(has drop)^3] ng - inconsistent size, expects `56` but actually is `24`
```

And, on runned natively with `aa`, I'd got:

```
[  no drop   ] ok - size `1`
[ has drop   ] ok - size `24`
[(has drop)^2] ok - size `40`
[(has drop)^3] ok - size `56`
```

But if runned on Miri with `sa`, I'd got:

```
[  no drop   ] ok - size `1`
[ has drop   ] ok - size `24`
[(has drop)^2] ok - size `24`
[(has drop)^3] ok - size `24`
```

And, with `aa`, respectively, I'd got:

```
[  no drop   ] ok - size `1`
[ has drop   ] ok - size `24`
[(has drop)^2] ok - size `40`
[(has drop)^3] ok - size `56`
```

Of course Miri didn't emit any warnings or errors excepts `incomplete_features` warning of rustc.
