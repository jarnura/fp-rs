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
- **Higher-Kinded Types (HKTs):** The primary technical challenge is implementing HKTs in Rust. The current approach uses:
    - **Marker Traits and GATs:** `HKT`, `HKT1`, and `type Applied<T>` as defined in `src/kind_based/kind.rs`. This system is now stable and forms the core of the library's functional abstractions when the `kind` feature is enabled. (See also: [HKT Implementation in System Patterns](./systemPatterns.md#higher-kinded-types-hkt-implementation)).
    - **`'static` and `Clone` Bounds:** These bounds remain a significant aspect of the HKT system, particularly for `CFnHKTMarker`, `CFnOnceHKTMarker`, and `ReaderTHKTMarker`. The `Applicative::pure` method's requirement for `T: Clone` makes some applicative laws untestable for types like `CFn` and `CFnOnce` when the "value" being `pure`d is itself a function wrapper that isn't `Clone`.
    - **Feature Flagging:** The `kind` feature flag successfully separates classic implementations (gated by `not(feature = "kind")`) from the HKT-based implementations.
    - **Monad Trait:** The HKT `Monad` trait (and `Bind`) has been successfully defined and implemented, resolving previous cyclic dependency issues.
- **Ownership and Borrowing:** All HKT implementations adhere to Rust's ownership and borrowing rules.
- **Type System:** The HKT implementation leverages Rust's strong, static type system. Previous compilation challenges have been largely resolved.
- **Performance:** HKT system is functional and tested for correctness. Performance implications are yet to be benchmarked.
- **Monad Transformers (`ReaderT` - HKT Context):**
    - `ReaderT` has been successfully refactored for the HKT system, using `ReaderTHKTMarker`. It uses `Rc<dyn Fn(R) -> MMarker::Applied<A> + 'static>` internally, similar to its classic counterpart, and correctly implements HKT `Functor`, `Apply`, `Applicative`, `Bind`, `Monad`, and `MonadReader`.

## Dependencies
- **Standard Library:** Primarily rely on Rust's standard library (`std`).
- **External Crates:** Aim for minimal external dependencies. If any are introduced, they should be well-justified (e.g., for advanced HKT emulation patterns if a community crate provides a good solution, or for specific benchmarking/testing utilities). `Cargo.toml` will be the source of truth for dependencies.

## Tool Usage Patterns
- **`cargo build`:** For compiling the library.
- **`cargo test`:** For running unit and integration tests. When working with HKTs: `cargo test --features kind --no-default-features`.
- **`cargo bench`:** For running benchmarks (if `criterion` is set up).
- **`cargo fmt`:** For code formatting, ensuring consistent style.
- **`cargo clippy`:** For linting. When working with HKTs: `cargo clippy --features kind --no-default-features`.
- **`rustdoc` (via `cargo doc`):** For generating documentation.
- **`cargo check --features kind --no-default-features`:** Crucial for quick compilation checks during HKT refactoring.
