# Environment-friendly async (`enfync`)  {INITIAL RELEASE IS WIP}

Ergonomic utilities for async IO work that easily cross-compiles for native and browser.
- Use `enfync::builtin::Handle::spawn() -> enfync::PendingResult<R>` to launch an IO task on your desired runtime (see documentation for details). The `enfync::PendingResult<R>` output can be used as a join handle on the task. Any errors encountered during your async work will be discarded and replaced with `Err(enfync::ResultError::TaskFailure)`.

This crate is designed for projects that want to ergonomically support WASM targets without sacrificing performance on native builds.


## Features

- `default`: `builtin`
- `builtin`: Enables the [`enfync::builtin`] module. The handle [`enfync::builtin::Handle`] is an alias for platform-specific implementations of the [`enfync::Handle`] trait (`tokio` on non-WASM, `wasm-bindgen-futures` on WASM).



## Important notes

- In WASM, only **one task** can run at a time. The first task is always `fn main()`, followed by whatever tasks were spawned during `fn main()`. Any long-running task, including `fn main()`, will block all other tasks. This means you fundamentally cannot benefit from this crate unless you develop your project from the ground up with WASM in mind.
- We do not provide any API dealing with 'web workers', which are a browser feature similar to threads except they have a **huge** overhead to launch and interact with.



## Comparison with [`prokio`](https://crates.io/crates/prokio)

### Pros

- `enfync::PendingResult<R>` can be used as a join handle.
- `enfync::builtin::native::TokioHandle::try_adopt()` can adopt an existing normal `tokio` runtime (no dependency on `prokio`'s LocalSet-specific design).
- The `enfync::ResultReceiver`/`enfync::Handle` abstractions allow users to easily implement their own custom runtimes (you could even implement a `prokio`-backed `Handle`).

### Cons

- `prokio` is more developed, with a runtime builder, pinned task synchronization primitives, etc.
- `prokio` is compatible with `?Send` tasks, whereas `enfync` only allows `Send` tasks. That means `prokio` is more suited for projects that need to pass WebAssembly types between tasks.



## Recommended WASM Build

We provide a custom `release-wasm` profile that enables `panic = "abort"` and optimizes for small binaries. There is a corresponding `dev-wasm` profile that enables `panic = "abort"`. Currently `wasm-pack` [doesn't support](https://github.com/rustwasm/wasm-pack/issues/1111) custom profiles, so we have to settle for a more verbose build script that overwrites the build files.

1. Prep tooling
- `rustup target install wasm32-unknown-unknown`
- `cargo install wasm-pack`
- install [`wasm-opt`](https://github.com/webassembly/binaryen)

2. Build (this builds twice because we want the `wasm-pack` convenience output and the `release-wasm` profile; you can drop the `wasm-pack` piece as needed)
```ignore
wasm-pack build --target no-modules --mode no-install &&
cargo build --profile=release-wasm --target wasm32-unknown-unknown &&
wasm-bindgen --out-dir ./pkg --target no-modules ./target/wasm32-unknown-unknown/release-wasm/enfync.wasm
```

3. Optimize WASM binary
- `wasm-opt -Os pkg/enfync_bg.wasm -o pkg/enfync_bg.wasm`
- see [the reference](https://rustwasm.github.io/book/reference/code-size.html) for further optimizations

4. Compress WASM binary
- TODO: gzip



## Running WASM

- **Tests**: `wasm-pack test --node`
- **Run your program**: [wasm-server-runner](https://github.com/jakobhellermann/wasm-server-runner) tool



## Options

- `TOKIO_WORKER_THREADS` (env variable): Size of default IO task pool (native builds only).



## Usage

```rust
// path shortcuts

```



## Perf Notes

- Default threadpool initialization is deferred to the first time you spawn something on that threadpool.

