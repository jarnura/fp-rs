# System Patterns

## System Architecture
- **Library Crate:** The project is structured as a Rust library crate.
- **Modular Design:**
    - **HKT-by-Default:** Higher-Kinded Type (HKT) implementations are now the default for core functional traits (`Functor`, `Apply`, `Applicative`, `Monad`, etc.). These are located directly within their respective modules (e.g., `src/functor.rs`, `src/identity.rs`).
    - **Legacy Implementations:** Older, "classic" (associated type-based) implementations have been moved to a new `src/legacy/` directory structure (e.g., `src/legacy/functor.rs`, `src/legacy/transformers/reader.rs`). These are accessible via a `legacy` feature flag.
    - **HKT Core:** The HKT system itself (marker traits, GATs) is defined in `src/kind_based/`.
    - **Transformers:** Monad transformers (e.g., `ReaderT` in `src/transformers/reader.rs`) use HKTs by default. Legacy transformer versions are in `src/legacy/transformers/`.
- **Trait-based Generics:** Core abstractions are defined using Rust traits.
- **Testing Structure:**
    - **Default Tests (HKT):** Tests for HKT implementations are located in `tests/hkt/` (e.g., `tests/hkt/functor.rs`) and specific files like `tests/transformers/reader_test.rs` (for HKT `ReaderT`). These are run by default, integrated via `tests/hkt_integration.rs`.
    - **Legacy Tests:** Tests for classic implementations are located in `tests/legacy/` (e.g., `tests/legacy/functor.rs`, `tests/legacy/transformers/reader_test.rs`). These are run when the `legacy` feature is enabled, integrated via `tests/legacy_integration.rs`.
    - This structure ensures clear separation and conditional compilation of tests.

## Key Technical Decisions
- **Focus on Idiomatic Rust:** Prioritize solutions that align with Rust's ownership, borrowing, and type system principles.
- **HKT as Default Abstraction:** The HKT system (marker traits and GATs) is the primary and default way of defining and using `Functor`, `Apply`, `Applicative`, `Monad`, etc. The `kind` feature has been removed.
- **Legacy Support:** Classic implementations are available via the `legacy` feature flag, residing in the `src/legacy/` directory.
- **Leverage Existing Rust Types:** Where appropriate, implement monadic traits for standard Rust types like `Option<T>`, `Result<T, E>`, and `Vec<T>` to showcase their monadic nature and provide immediate utility.
- **Higher-Kinded Types (HKT) Implementation:** The project implements a Higher-Kinded Type system using:
    - **Marker Traits:** `HKT` and `HKT1` defined in `src/kind_based/kind.rs`.
    - **Generic Associated Types (GATs):** The `HKT1` trait defines `type Applied<T>`.
    - **Marker Types:** Specific structs (e.g., `OptionHKTMarker`, `ReaderTHKTMarker`) implement `HKT1`.
This system is stable and implemented for `Option`, `Result`, `Vec`, `CFn`, `CFnOnce`, `Identity`, and `ReaderT`. (See also: [HKT Constraints in Tech Context](./techContext.md#higher-kinded-types-hkts))
- **Minimal Dependencies:** Aim for minimal external dependencies.

## Design Patterns in Use
- **Trait-based Polymorphism:** Central to defining functional interfaces.
- **Type Classes:** Traits like `Functor`, `Applicative`, `Monad` act as type classes.
- **Composition over Inheritance:** Standard functional programming approach.
- **Monad Transformers:** `ReaderT` augments existing monads.
    - **Default `ReaderT<R, MMarker, A>`:** Uses HKT marker `MMarker` for the inner monad.
    - **Legacy `ReaderT<R, M, A>`:** Uses concrete type `M` for the inner monad.
    - **`Identity<A>`:** Simple monad, used with both HKT (`IdentityHKTMarker`) and legacy versions.

## Component Relationships
- **Core HKT:**
    - `src/kind_based/kind.rs`: Defines `HKT`, `HKT1`, and HKT marker types.
    - `src/kind_based/mod.rs`: Module file for `kind_based`.
- **Main Functional Traits (HKT Default):**
    - `src/functor.rs`: Defines HKT `Functor`.
    - `src/apply.rs`: Defines HKT `Apply`.
    - `src/applicative.rs`: Defines HKT `Applicative`.
    - `src/monad.rs`: Defines HKT `Bind` and `Monad`.
    - `src/identity.rs`: Defines `Identity<A>` and `IdentityHKTMarker`, with HKT trait implementations.
    - `src/transformers/reader.rs`: Defines HKT `ReaderT<R, MMarker, A>` and `ReaderTHKTMarker`, with HKT trait implementations including `MonadReader`.
- **Legacy Components (gated by `legacy` feature):**
    - `src/legacy/mod.rs`: Main module for legacy code.
    - `src/legacy/functor.rs`: Defines classic `Functor`.
    - `src/legacy/apply.rs`: Defines classic `Apply`.
    - `src/legacy/applicative.rs`: Defines classic `Applicative`.
    - `src/legacy/monad.rs`: Defines classic `Bind` and `Monad`.
    - `src/legacy/identity.rs`: Defines classic implementations for `Identity<A>`.
    - `src/legacy/transformers/mod.rs`: Module for legacy transformers.
    - `src/legacy/transformers/reader.rs`: Defines classic `ReaderT<R, M, A>` and its implementations.
- **Library Entry Point:**
    - `src/lib.rs`: Re-exports HKT traits and types by default. Exports the `legacy` module if the `legacy` feature is active.
- **Utilities & Other:**
    - `src/utils.rs`: Helper functions. Obsolete `bfn!` macros have been removed.
    - `src/function.rs`: Defines `CFn`, `CFnOnce`. Obsolete `BindableFn` code has been removed.
    - `src/profunctor.rs`: Implements `Profunctor`, `Strong`, `Choice` (currently non-HKT).
    - `src/transformers/mod.rs`: Module for HKT monad transformers.

## Critical Implementation Paths
- **HKT System Implementation (Completed):**
    - Core HKT traits and markers defined.
    - `Functor`, `Apply`, `Applicative`, `Bind`, `Monad` refactored to use HKT.
    - `Option`, `Result`, `Vec`, `CFn`, `CFnOnce`, `Identity`, `ReaderT` updated for HKT.
- **HKT-by-Default Refactor (Completed):**
    - Moved classic code to `src/legacy/`.
    - Updated main source files and `src/lib.rs` for HKT-default.
    - Updated `Cargo.toml` (removing `kind` feature, adding `legacy` feature).
    - Reorganized test suite for HKT-default and legacy tests.
    - Fixed `use` paths in `src/legacy/` (completed) and `tests/legacy/` (completed).
- **Testing (Completed):**
    - HKT laws tested for all relevant types. Default tests (`cargo test`) pass.
    - Legacy tests verified and pass (`cargo test --features legacy`).
