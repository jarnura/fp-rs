# Project Progress

## What Works
- **Classic Monadic Implementations (Pre-HKT refactor):**
    - `Option<A>`: Functor, Apply, Applicative, Monad traits and laws implemented and tested.
    - `Result<T, E>`: Functor, Apply, Applicative, Monad traits and laws implemented and tested.
    - `Vec<T>`: Functor, Apply, Applicative, Monad traits and laws implemented and tested.
- **Profunctor Implementation:**
    - `Profunctor`, `Strong`, `Choice` traits and laws implemented and tested for `CFn`.
- **Identity Monad (Classic):**
    - `Identity<A>` struct implemented in `src/identity.rs`.
    - `Functor`, `Apply`, `Applicative`, `Monad` traits and laws implemented and tested for `Identity<A>`.
- **ReaderT Monad Transformer (Classic):**
    - `ReaderT<R, M, A>` struct implemented in `src/transformers/reader.rs`.
    - `Functor`, `Apply`, `Applicative`, `Monad` traits implemented for `ReaderT` (conditional on inner monad `M`).
    - `MonadReader<R, A>` trait defined and implemented for `ReaderT`.
    - All `ReaderT` related compilation errors and test failures resolved (for classic version).
- **Codebase Health (Pre-HKT refactor):**
    - All (7 lib unit tests + 91 integration tests + 28 doc tests = 126 total) tests passing for the classic implementations.
    - Code formatted with `cargo fmt`.
    - Linted with `cargo clippy` (no warnings beyond intentionally suppressed ones).
    - All compiler warnings addressed.
- **Memory Bank:**
    - Core documents established. `activeContext.md` updated with HKT refactor details.
    - Archive for historical project data created: [Project History](./archive/project_history_pre_aug_2025.md).
- **Documentation (Largely Complete for Classic System):**
    - Comprehensive `rustdoc` comments for public items in classic modules.
    - `README.md` significantly updated.

## What's Left to Build / Planned (Key Priorities)
- **Implement Kind-based HKT System (feature: `kind`) (In Progress)**
  - **Phase 1: Project Setup & Core `Kind` Trait**
    - **Micro-Task 1.1: Add `kind` feature to `Cargo.toml` (Completed).**
    - **Micro-Task 1.2: Create `src/kind_based/` directory, modify `src/lib.rs`, create `src/kind_based/mod.rs` (Completed).**
    - **Micro-Task 1.3: Define `HKT`, `HKT1` traits and initial HKT markers in `src/kind_based/kind.rs` (Completed).**
  - **Phase 2: Refactor `Functor` Trait and Implementations (Completed)**
    - `src/functor.rs` created with `classic::Functor` and `hkt::Functor`.
    - `hkt::Functor` implemented for `OptionHKTMarker`, `ResultHKTMarker`, `VecHKTMarker`, `CFnHKTMarker`, `CFnOnceHKTMarker`.
    - Lifetime and trait bound issues addressed. All tests pass with `--features kind --no-default-features` and no warnings.
  - **Phase 3: Refactor `Apply` Trait and Implementations (Completed)**
    - `src/apply.rs` created with `classic::Apply` and `hkt::Apply`.
    - `hkt::Apply` implemented for `OptionHKTMarker`, `ResultHKTMarker`, `VecHKTMarker`, `CFnHKTMarker`, `CFnOnceHKTMarker`.
    - Issues with `Self::Applied<T>` and method calls (E0308/E0599) addressed. All tests pass with `--features kind --no-default-features` and no warnings.
  - **Phase 4: Refactor `Applicative` Trait and Implementations (Completed)**
    - `src/applicative.rs` created with `classic::Applicative` and `hkt::Applicative`.
    - `hkt::Applicative` implemented for `OptionHKTMarker`, `ResultHKTMarker`, `VecHKTMarker`, `CFnHKTMarker`, `CFnOnceHKTMarker`.
    - Issues with `Self::Applied<T>` and method calls addressed. All tests pass with `--features kind --no-default-features` and no warnings.
  - **Phase 5: Refactor `Bind` Trait and Implementations (Completed)**
    - Created `src/monad.rs` with `hkt::Bind`.
    - Implemented `hkt::Bind` for relevant HKT markers (direct implementations).
    - `hkt::Bind` trait defined with `Apply<A,B>` as supertrait.
  - **Phase 6: Refactor `Monad` Trait (Completed)**
    - Defined `hkt::Monad` in `src/monad.rs` with `Applicative<A>` as supertrait.
    - `Monad::join` is a required method, implemented directly for all HKT markers.
    - Resolved cyclic dependency issues between `Bind` and `Monad`.
  - **Phase 7: Update `Identity` Monad for HKT (Completed)**
    - `hkt::IdentityHKTMarker` created and `hkt::Functor`, `hkt::Apply`, `hkt::Applicative`, `hkt::Monad` (including `join`) implemented in `src/identity.rs`.
    - All tests for `Identity` HKT implementations, including monad laws and `join`, pass with `cargo test --features kind --no-default-features`.
    - Doc tests in `README.md` updated for HKT `Option` examples.
    - Obsolete `bfn!` macros and doc tests in `src/utils.rs` commented out.
  - **Phase 8: Update `ReaderT` Monad Transformer for HKT (Completed)**
    - `hkt::ReaderTHKTMarker` exists.
    - `hkt::Functor`, `hkt::Apply`, `hkt::Applicative`, `hkt::Bind`, and `hkt::MonadReader` were already implemented and tested for `ReaderTHKTMarker`.
    - `hkt::Monad` (including `join`) successfully implemented for `ReaderTHKTMarker`. All tests pass with `--features kind --no-default-features` and no warnings.
  - **Phase 9: Testing (Kind-based) (Current Focus)**
    - **Completed Sub-tasks:**
        - HKT tests for `Identity` Monad laws are complete and passing.
        - Comprehensive tests for `ReaderT` HKT `Monad` (including `join` and monad laws) added and passing.
        - HKT tests for `Option` Monad laws added and passing.
        - Monad law tests for `ResultHKTMarker`, `VecHKTMarker`, `CFnHKTMarker`, and `CFnOnceHKTMarker` are implemented and passing.
        - Applicative laws for `ReaderT` (as part of `applicative::hkt_laws_tests`) are passing.
        - Functor laws for all HKT-enabled types (`Option`, `Result`, `Vec`, `CFn`, `CFnOnce`, `Identity`, `ReaderT`) are comprehensively covered in `tests/hkt/functor.rs` and passing.
        - Applicative laws for `Option`, `Result`, `Identity`, and `Vec` are covered in `tests/hkt/applicative.rs` and passing. For `CFn`, `CFnOnce`, and `ReaderT`, laws involving `pure` with function types are noted as untestable due to `Clone` constraints.
    - **Phase 9 Completed.**
  - **Phase 10: Documentation (Memory Bank Update & Rustdoc for Kind System) (Current Focus)**
    - Memory Bank files (`activeContext.md`, `progress.md`, `systemPatterns.md`, `techContext.md`) updated to reflect HKT refactor completion and testing.
    - `rustdoc` for `src/applicative.rs` (HKT module) improved.
    - `rustdoc` for `src/monad.rs` (HKT module) improved.
    - `rustdoc` for `src/identity.rs` (HKT module) improved.
    - `rustdoc` for `src/transformers/reader.rs` (HKT module, including `MonadReader` HKT) improved.
    - General review of all generated `rustdoc` for clarity and completeness once HKT refactor stabilizes.

- **Documentation (Final Review - Postponed):**
    - General review of all generated `rustdoc` for clarity and completeness once HKT refactor stabilizes.
- **Review `src/experimental_apply.rs` (Lower Priority):**
    - Determine its fate (document, feature-gate, or privatize) after HKT refactor.
- **Benchmarking (Analysis for Classic System Complete, HKT TBD):**
    - Benchmarks for classic `Functor::map`, `Apply::apply`, and `Bind::bind` on `Option`, `Result`, and `Vec` are done.
    - New benchmarks will be needed for HKT versions.

## Current Status
- **Phase:** Kind-based HKT System Implementation (Feature: `kind`) - Phase 8 (ReaderT HKT Monad) completed.
- **Focus:** Phase 10: Documentation (Memory Bank Update & Rustdoc for Kind System).
- **Compilation:** `cargo test --features kind --no-default-features` passes for all implemented HKT functionality and law tests. Doc tests also pass.

## Known Issues (HKT Refactor - Post Phase 9 Completion)
- **`experimental_apply.rs` and `function.rs` (beyond `CFn`/`CFnOnce`):** These files may still require updates in the context of the HKT refactor (Lower Priority).
- **`bfn!` macros:** Commented out in `src/utils.rs` due to reliance on obsolete `BindableFn`. Their future needs to be decided (Lower Priority).
- **Applicative Laws for `CFn`, `CFnOnce`, `ReaderT`:** Some laws involving `pure` with function types are untestable due to `Clone` constraints. This is a known limitation.
- **`'static` and `Clone` bounds:** Continued vigilance is needed.
- **Unused import warnings:** Some may persist but are minor (e.g. in `applicative.rs` after recent fixes, some might have been addressed).

## Evolution of Project Decisions
- The project has pivoted to a major refactoring to implement a more robust HKT system using marker traits and GATs. This is a significant architectural change.
- Historical project decisions are in [Project History](./archive/project_history_pre_aug_2025.md).
