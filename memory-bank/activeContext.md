# Active Context

## Current Work Focus
The primary effort is refactoring the `fp-rs` Rust library to use a new Higher-Kinded Types (HKT) pattern. This involves:
- Defining `HKT` and `HKT1` traits in `src/kind_based/kind.rs`.
- Creating HKT marker types (e.g., `OptionHKTMarker`, `CFnHKTMarker`).
- Refactoring core functional traits (`Functor`, `Apply`, `Applicative`, `Bind`) into:
    - `classic` submodules (old style with associated types like `Functor<T>`).
    - `hkt` submodules (new style with HKT markers and `Self::Applied<T>`).
- Updating implementations for these traits across various types (`Option`, `Result`, `Vec`, `CFn`, `CFnOnce`, `Identity`, `ReaderT`).
- The `Monad` trait itself is currently commented out to resolve cyclic dependencies.

## Recent Changes & Problem Solving
- **HKT Core (`kind.rs`, `functor.rs`, `apply.rs`, `applicative.rs`):**
    - `src/kind_based/kind.rs`: Defined `HKT`, `HKT1` traits.
    - `src/functor.rs` (`hkt::Functor<A,B>`): Stable with HKT.
    - `src/apply.rs` (`hkt::Apply<A,B>`): Stable with HKT. Resolved E0308/E0599 by removing incorrect `Fn`/`FnOnce` bounds on `CFn`/`CFnOnce` types in `impl Apply for CFn...Marker`.
    - `src/applicative.rs` (`hkt::Applicative<T>`): Stable with HKT.
- **`src/monad.rs` (HKT `Bind` and `Monad` - Phase 6 Completed):**
    - `hkt::Bind<A,B>` trait defined with `Apply<A,B>` as supertrait. `bind` is a required method, implemented directly for all markers.
    - `hkt::Monad<A>` trait defined with `Applicative<A>` as supertrait. `join` is a required method, implemented directly for all markers.
    - Successfully resolved previous E0391 cycle errors related to `Bind`/`Monad` definitions and `join` by:
        - Making `Monad::join` a required method (no default implementation with `Bind` in its `where` clause).
        - Simplifying `Bind` supertraits to `Apply<A,B>`.
        - Ensuring `CFn...Marker` implementations of `Monad::join` call their direct `Bind::bind` methods.
    - Removed incorrect `Fn`/`FnOnce` bounds from `impl Bind for CFn...Marker` where clauses.
- **Compilation Status:** `cargo check --features kind --no-default-features` now passes for the entire HKT system up to and including `Monad`.

## Next Steps
- **Phase 7: Update `Identity` Monad for HKT (Completed)**
    - `IdentityHKTMarker` defined in `src/identity.rs`.
    - `hkt::Functor`, `hkt::Apply`, `hkt::Applicative`, and `hkt::Monad` (including `join`) implemented for `IdentityHKTMarker`.
    - Tests for these implementations, including `join` and monad laws, are passing under `cargo test --features kind --no-default-features`.
    - Doc tests in `README.md` updated to reflect HKT usage for `Option` examples.
    - Obsolete `bfn!` macros and their doc tests in `src/utils.rs` commented out.
- **Phase 8: Update `ReaderT` Monad Transformer for HKT (Completed)**
    - `ReaderTHKTMarker` is defined in `src/transformers/reader.rs`.
    - HKT traits (`Functor`, `Apply`, `Applicative`, `Bind`, `Monad`, `MonadReader`) are implemented for `ReaderTHKTMarker`.
    - `hkt::Monad` for `ReaderTHKTMarker` is implemented and relevant tests (including some applicative laws involving `ReaderT`) are passing.
- **Phase 9: Testing (Kind-based) (Current Focus)**
    - Comprehensive tests for HKT `Monad` for `ReaderT` are complete and passing.
    - HKT tests for `Option` Monad laws are complete and passing.
    - HKT tests for `Identity` Monad laws are complete and passing.
    - Monad law tests for `ResultHKTMarker`, `VecHKTMarker`, `CFnHKTMarker`, and `CFnOnceHKTMarker` are implemented and passing. (Note: For `CFn` and `CFnOnce`, one of the `join` laws related to `pure(m)` is not applicable due to `Clone` constraints or the nature of `FnOnce`.)
    - **Functor laws for all HKT-enabled types (`Option`, `Result`, `Vec`, `CFn`, `CFnOnce`, `Identity`, `ReaderT`) are comprehensively covered in `tests/hkt/functor.rs` and passing.**
    - **Applicative laws for `Option`, `Result`, `Identity`, and `Vec` are covered in `tests/hkt/applicative.rs` and passing. For `CFn`, `CFnOnce`, and `ReaderT`, laws involving `pure` with function types are noted as untestable due to `Clone` constraints on `CFn`/`CFnOnce` and the `pure` implementation requiring `T: Clone`.**
    - Phase 9 (HKT Law Testing) is now considered complete.
- **Phase 10: Documentation (HKT System) (Current Focus)**
    - `rustdoc` for `src/applicative.rs` (HKT module) improved with examples and clarifications.
    - `rustdoc` for `src/monad.rs` (HKT module) improved with examples and clarifications.
    - `rustdoc` for `src/identity.rs` (HKT module) improved with examples and clarifications.
    - `rustdoc` for `src/transformers/reader.rs` (HKT module, including `MonadReader` HKT) improved with examples and clarifications.
    - Update Memory Bank files (`systemPatterns.md`, `techContext.md`) with insights from the HKT refactor and testing (this task).
    - General review of all generated `rustdoc`.

## Active Decisions and Considerations
- **HKT Simulation:** Using marker traits (`HKT`, `HKT1`) and Generic Associated Types (`type Applied<T>`) as the core HKT strategy. This is now stable through `Monad` for `Option`, `Result`, `Vec`, `CFn...`, and `Identity`.
- **Feature Flagging:** Using `#[cfg(feature = "kind")]` to manage classic vs. HKT implementations. Test files for classic implementations are now gated with `#[cfg(not(feature = "kind"))]`.
- **`'static` and `Clone` Bounds:** These remain frequently required.
- **`Monad` Trait:** Successfully refactored and integrated into the HKT system. `IdentityHKTMarker` now implements it.
- **`CFn` Clonability:** The existing decision against making `CFn` easily `Clone` persists.
- **`bfn!` macros:** Commented out due to reliance on obsolete `BindableFn`.

## Important Patterns and Preferences
- **Documentation First:** Keeping Memory Bank updated with the latest context.
- **Systematic Refinement & Testing:** Iteratively fixing compilation errors and verifying changes.
- **HKT Pattern:** The current refactoring revolves around the `HKT`/`HKT1` marker traits and `Self::Applied<T>` GAT.

## Learnings and Project Insights
- The HKT refactoring up to `Monad` (including `Identity`) has been a complex but successful undertaking.
- Ensuring HKT traits are correctly imported into test modules (`use crate::module::hkt::Trait;`) is crucial for E0599 errors.
- Making `hkt` submodules public (`pub mod hkt`) was necessary for tests in other modules to access HKT traits.
- Fully qualifying trait calls with the specific HKT marker (e.g., `<Marker as Trait<...>>::method(...)`) is sometimes needed if type inference fails, especially for complex traits like `MonadReader`.

## Known Issues (Post Phase 7)
- `experimental_apply.rs` and `function.rs` (beyond `CFn`/`CFnOnce`) may still require updates in the context of the HKT refactor.
- Comprehensive HKT law testing for `ReaderT`'s `Monad` implementation is pending (Phase 9).
- Unused import warnings persist but are minor.
