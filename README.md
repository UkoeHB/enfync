# Environment-friendly async (`enfync`)  {INITIAL RELEASE IS WIP}

Ergonomic utilities for async work that easily cross-compiles for native and browser.
- Use `PendingResult::new()` to launch a task on your desired runtime (see documentation for details). The `PendingResult` can be used as a join handle on the task. Any errors encountered during your async work will be discarded and replaced with `enfync::Result::Err`.

This crate is designed for projects that want to ergonomically support WASM targets without sacrificing performance on native builds.



## Recommended WASM Build

We provide a custom `release-wasm` profile that enables `panic = "abort"` and optimizes for small binaries. There is a corresponding `dev-wasm` profile that enables `panic = "abort"`.

1. Prep tooling
- `rustup target install wasm32-unknown-unknown`

2. Build
```ignore
cargo build --release-wasm --target wasm32-unknown-unknown &&
wasm-pack build --target no-modules ./target/ --mode no-install --out-name wasm_gen
```

3. Optimize WASM binary
- install [`wasm-opt`](https://github.com/webassembly/binaryen)
- `wasm-opt Os wasm_gen_bg.wasm -o -`



## Options

- `TOKIO_WORKER_THREADS` (env variable): Size of default IO task pool (native builds only).



## Usage

```rust
// path shortcuts

```



## Perf Notes

- Default threadpool initialization is deferred to the first time you spawn something on that threadpool.



## Comments

- This crate does not use `rustfmt`.
