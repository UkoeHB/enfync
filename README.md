# Environment-friendly async (`enfync`)  {INITIAL RELEASE IS WIP}

Ergonomic utilities for async IO work that easily cross-compiles for native and browser.
- Use `enfync::Handle::spawn() -> enfync::PendingResult<R>` to launch an IO task on your desired runtime (see documentation for details). The `enfync::PendingResult<R>` output can be used as a join handle on the task. Any errors encountered during your async work will be discarded and replaced with `Err(enfync::ResultError::TaskFailure)`.

This crate is designed for projects that want to ergonomically support WASM targets without sacrificing performance on native builds.



## Important notes

- In WASM, only **one task** can run at a time. The first task is always `fn main()`, followed by whatever tasks were spawned during `fn main()`. Any long-running task, including `fn main()`, will block all other tasks. This means you fundamentally cannot use this crate unless you develop your project from the ground up with WASM in mind.
- We do not provide any API dealing with 'web workers', which are a browser feature similar to threads except they have a **huge** overhead to launch and interact with.



## Comparison with [`prokio`](https://crates.io/crates/prokio)

### Pros

- `enfync::PendingResult<R>` can be used as a join handle.
- `enfync::builtin::Handle::try_adopt()` can adopt an existing normal `tokio` runtime (no dependency on `prokio`'s LocalSet-specific design).
- The `enfync::ResultReceiver`/`enfync::Handle` abstractions allow users to easily implement their own custom runtimes (you could even implement a `prokio`-backed `Handle`).

### Cons

- `prokio` is more developed, with a runtime builder, pinned task synchronization primitives, etc.
- `prokio` is compatible with `?Send` tasks, whereas `enfync` only allows `Send` tasks. That means `prokio` is more suited for projects that need to pass WebAssembly types between tasks.



## Recommended WASM Build

We provide a custom `release-wasm` profile that enables `panic = "abort"` and optimizes for small binaries. There is a corresponding `dev-wasm` profile that enables `panic = "abort"`.

1. Prep tooling
- `rustup target install wasm32-unknown-unknown`
- `cargo install wasm-pack`
- install [`wasm-opt`](https://github.com/webassembly/binaryen)

2. Build
```ignore
cargo build --release-wasm --target wasm32-unknown-unknown &&
wasm-pack build --target no-modules ./target/ --mode no-install
```

3. Optimize WASM binary
- `wasm-opt Os wasm_gen_bg.wasm -o -`



## Running WASM tests

Command: `wasm-pack test --node`



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
