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
- **Higher-Kinded Types (HKTs):** HKTs are the default abstraction. The implementation uses:
    - **Marker Traits and GATs:** `HKT`, `HKT1`, and `type Applied<T>` as defined in `src/kind_based/kind.rs`. This system forms the core of the library's functional abstractions. (See also: [HKT Implementation in System Patterns](./systemPatterns.md#higher-kinded-types-hkt-implementation)).
    - **`'static` and `Clone` Bounds:** These bounds remain a significant aspect of the HKT system, particularly for `CFnHKTMarker`, `CFnOnceHKTMarker`, and `ReaderTHKTMarker`. The `Applicative::pure` method's requirement for `T: Clone` makes some applicative laws untestable for types like `CFn` and `CFnOnce` when the "value" being `pure`d is itself a function wrapper that isn't `Clone`.
    - **Feature Flagging:** The `legacy` feature flag provides access to classic (non-HKT, associated type-based) implementations, which reside in `src/legacy/`. The `kind` feature has been removed.
    - **Monad Trait:** The HKT `Monad` trait (and `Bind`) is the default and has been successfully defined and implemented.
- **Ownership and Borrowing:** All implementations adhere to Rust's ownership and borrowing rules.
- **Type System:** The HKT implementation leverages Rust's strong, static type system.
- **Performance:** HKT system is functional and tested for correctness. Performance implications (HKT vs. Legacy) are yet to be benchmarked.
- **Monad Transformers (`ReaderT`):**
    - **Default (HKT):** `ReaderT` uses the HKT system by default (`ReaderTHKTMarker`). It uses `Rc<dyn Fn(R) -> MMarker::Applied<A> + 'static>` internally and implements HKT `Functor`, `Apply`, `Applicative`, `Bind`, `Monad`, and `MonadReader`.
    - **Legacy:** The classic version of `ReaderT` is available via the `legacy` feature.

## Dependencies
- **Standard Library:** Primarily rely on Rust's standard library (`std`).
- **External Crates:** Minimal external dependencies. `Cargo.toml` is the source of truth.

## Tool Usage Patterns
- **`cargo build`:** For compiling the library (builds HKT by default).
    - **`cargo build --features legacy`:** To build with legacy code included.
- **`cargo test`:** For running unit and integration tests (runs HKT tests by default).
    - **`cargo test --features legacy`:** To run legacy tests.
- **`cargo bench`:** For running benchmarks.
- **`cargo fmt`:** For code formatting.
- **`cargo clippy`:** For linting (checks HKT code by default).
    - **`cargo clippy --features legacy`:** To lint legacy code.
- **`rustdoc` (via `cargo doc`):** For generating documentation (generates for HKT by default).
    - **`cargo doc --features legacy`:** To generate documentation including legacy code.
- **`cargo check`:** For quick compilation checks (checks HKT code by default).
    - **`cargo check --features legacy`:** To check legacy code.
