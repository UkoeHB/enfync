# Environment-friendly async (`enfync`)

Provides easy-to-use utilities for performing async work regardless of environment (native/browser).
- Use `PendingResult::new()` to launch a task on your desired runtime (see documentation for details). The `PendingResult` can be used as a join handle on the task. Any errors encountered during your async work will be discarded and replaced with `PRResult::Error`.

This crate is designed for projects that want to ergonomically support WASM targets without sacrificing performance on native builds.



## Recommended WASM Build

We provide a custom `release-wasm` profile that enables `panic = "abort"` and optimizes for small binaries. There is a corresponding `dev-wasm` profile that enables `panic = "abort"`.

1. Prep tooling
- `rustup target install wasm32-unknown-unknown`
- install [`wasm-opt`](https://github.com/webassembly/binaryen)

2. Build
```ignore
cargo build --release-wasm --target wasm32-unknown-unknown &&
wasm-pack build --target no-modules ./target/ --mode no-install --out-name wasm_gen
```

3. Optimize WASM binary
- `wasm-opt Os wasm_gen_bg.wasm -o -`



## Options

- `TOKIO_WORKER_THREADS` (env variable): Default size of IO task pool (native builds only).



## Usage

```rust
// path shortcuts

```



## Perf Notes

- Default threadpool initialization is deferred to the first time you spawn something on that threadpool.



## Comments

- This crate does not use `rustfmt`.
