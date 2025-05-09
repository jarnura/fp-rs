# Tech Context

## Technologies Used
- **Primary Language:** Rust (stable, as defined by `rust-toolchain.toml`).
- **Build System & Package Manager:** Cargo (Rust's default).
- **Testing Framework:** Rust's built-in testing framework (`#[test]`), potentially with `criterion.rs` for benchmarking (as suggested by `criterion.toml` and `benches/compare.rs`).

## Development Setup
- **IDE/Editor:** Any Rust-compatible editor (e.g., VSCode with rust-analyzer, IntelliJ Rust).
- **Compiler:** `rustc`.
- **Version Control:** Git (implied by `.gitignore`).
- **Toolchain Management:** `rustup` (standard Rust toolchain manager). The `rust-toolchain.toml` file pins the toolchain to `stable` for consistency.

## Technical Constraints
- **Higher-Kinded Types (HKTs):** Rust's lack of native HKTs is a primary constraint. Implementations will need to use established patterns (e.g., associated types, helper traits) to emulate HKTs, which can add complexity. (See also: [HKT Emulation in System Patterns](./systemPatterns.md#higher-kinded-types-hkt-emulation))
- **Ownership and Borrowing:** All implementations must strictly adhere to Rust's ownership and borrowing rules. This can make direct translation of monadic patterns from languages with garbage collection challenging but also leads to safer code.
- **Type System:** Leveraging Rust's strong, static type system to ensure correctness and provide good compile-time errors.
- **Performance:** While correctness and clarity are primary, implementations should strive to be reasonably performant, avoiding unnecessary allocations or overhead where possible.
- **Monad Transformers (`ReaderT`):**
    - The implementation of `ReaderT` was refactored to use `Rc<dyn Fn(R) -> M + 'static>` (from `Box<dyn FnMut(R) -> M + 'static>`) to store its core function. This allows shared ownership of the function, but enforces `Fn` (immutable borrow) semantics for the closure.
    - This change necessitated stricter adherence to `'static` and `Clone` bounds throughout `ReaderT` and related trait implementations (`Functor`, `Apply`, `Bind`, `MonadReader`).
    - Specifically, type parameters like `B` in `Apply::apply` now require `'static` bounds in the trait definitions themselves to ensure that the types constructed within `ReaderT` (which must be `'static` due to `Rc<dyn Fn ... + 'static>`) are valid.
    - `Clone` bounds are frequently required for function arguments to traits (e.g., the `f` in `Functor::map` or `Bind::bind`) and for captured values in closures, especially when these closures are `'static`. The environment `R` in `ReaderT` also often requires `Clone`.

## Dependencies
- **Standard Library:** Primarily rely on Rust's standard library (`std`).
- **External Crates:** Aim for minimal external dependencies. If any are introduced, they should be well-justified (e.g., for advanced HKT emulation patterns if a community crate provides a good solution, or for specific benchmarking/testing utilities). `Cargo.toml` will be the source of truth for dependencies.

## Tool Usage Patterns
- **`cargo build`:** For compiling the library.
- **`cargo test`:** For running unit and integration tests.
- **`cargo bench`:** For running benchmarks (if `criterion` is set up).
- **`cargo fmt`:** For code formatting, ensuring consistent style.
- **`cargo clippy`:** For linting and catching common mistakes or unidiomatic code.
- **`rustdoc` (via `cargo doc`):** For generating documentation from source code comments.
