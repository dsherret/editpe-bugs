1. Install [Deno](https://deno.com/)
1. Run `deno run -A build.ts`

Output:

```
> deno run -A build.ts
    Finished `release` profile [optimized] target(s) in 0.06s
   Compiling write v0.1.0 (V:\scratch_rust\write)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.27s
     Running `target\debug\write.exe`

thread 'main' panicked at V:\.cache\cargo\registry\src\index.crates.io-1949cf8c6b5b557f\editpe-0.2.1\src\image.rs:471:29:
attempt to subtract with overflow
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```