# Active Context

## Current Work Focus
- **Phase 3, Step 3: Comprehensive Documentation (Largely Complete).**
  - `rustdoc` comments added to core modules: `functor.rs`, `apply.rs`, `applicative.rs`, `monad.rs`, `function.rs`, `profunctor.rs`, `utils.rs`, and `lib.rs`.
  - `README.md` updated with project overview, features, usage examples, and build/test instructions.
- **Current Task: Implement `ReaderMonad` (`ReaderT`) (Completed).**
  - Created `src/identity.rs` with `Identity` monad and its `Functor`, `Apply`, `Applicative`, `Monad` implementations.
  - Created `src/transformers/reader.rs` with `ReaderT` struct and implementations for `Functor`, `Apply`, `Applicative`, `Monad`, and `MonadReader`.
  - Integrated new modules into `src/lib.rs` and `src/transformers/mod.rs`.
  - Successfully debugged compilation errors in `ReaderT` related to trait bounds (`'static`, `Clone`) after refactoring to `Rc<dyn Fn>`.

## Recent Changes
- **`ReaderT` Implementation and Debugging:**
    - Refactored `ReaderT` internal function from `Box<dyn FnMut(R) -> M>` to `Rc<dyn Fn(R) -> M + 'static>`.
    - Resolved E0276 (impl has stricter requirements) for `ReaderT::apply` by:
        - Ensuring the `where` clause in `ReaderT::apply` correctly used `Self` for associated types to match the trait: `<Self as Functor<A>>::Functor<<Self as Apply<A>>::Fnn<A, B>>: 'static`.
        - Adding `B: 'static` to the `Apply::apply` trait definition in `src/apply.rs`, as this was a necessary bound for `ReaderT`'s implementation and consistent for other types.
    - Propagated `B: 'static` (or `C: 'static`, `D: 'static` as appropriate) bounds to helper functions `lift_a1` (in `src/applicative.rs`), `lift2`, and `lift3` (in `src/apply.rs`) to fix resulting E0310 lifetime errors.
    - Corrected a failing unit test `test_reader_t_functor_map` by updating an incorrect assertion value.
    - All tests now pass.
- **Test Organization:**
    - Moved inline tests from `src/transformers/reader.rs` to a new integration test file `tests/transformers/reader_test.rs`.
- **Phase 3, Step 3 (Documentation) Progress:**
    - Added comprehensive `rustdoc` comments to all public items in `src/functor.rs`, `src/apply.rs`, `src/applicative.rs`, `src/monad.rs`, `src/function.rs`, `src/profunctor.rs`, `src/utils.rs` (macros), and `src/lib.rs` (crate-level docs and re-exports).
    - Updated `README.md` significantly with detailed project information, feature list, usage examples, and sections on building, testing, and benchmarks.
- **Phase 3, Step 4 (Final Review & Cleanup) Completed (Previously):**
    - Codebase formatted, linted, and all compiler/clippy warnings addressed.
    - E0210 (orphan rule) for `BitOr` on `BindType` resolved by removing the implementation.
    - `cargo test` confirms 83 unit tests + 3 doc tests passing without warnings (prior to `ReaderT` work).
- For a detailed history of previous changes and completed phases, see [Project History](./archive/project_history_pre_aug_2025.md).


## Next Steps
- **Add `rustdoc`:** Comprehensive documentation for `src/transformers/reader.rs` and `src/identity.rs`.
- **Review `src/experimental_apply.rs`:** Determine if this module should be fully documented as part of the public API, feature-gated, or made private.
- **Benchmarking Analysis (Ongoing):**
    - Initial benchmarks for `Functor`, `Apply`, and `Bind` on `Option`, `Result`, and `Vec` have been implemented and run using `criterion.rs`.
    - Performance overhead analysis is underway. See `progress.md` for detailed findings.
- **Post-Phase 3 Considerations (Lower Priority):**
    - **`src/main.rs`:** Define purpose (e.g., examples, CLI).
    - **`Cargo.toml` Metadata:** Update author and URL information when available.
    - **Revisit `BitOr` for `BindType`:** Consider alternative designs if the operator syntax is desired.

## Active Decisions and Considerations
- **`CFn` Clonability:** Decided against making `CFn` (holding `Box<dyn Fn>`) easily `Clone` due to complexity. Tests requiring multiple uses of the same function now recreate the `CFn` instance. This might need revisiting if it becomes a major ergonomic issue.
- **Test Module Organization:** Integration tests are now located in the top-level `tests/` directory, mirroring the module structure of `src/`. Law tests remain organized in submodules within these test files.
- **Warnings:** All compiler and clippy warnings addressed in Phase 3, Step 4. Some potentially incorrect `unused_imports` warnings for necessary trait imports (like `Functor`) in test files were suppressed using `#[allow(unused_imports)]`.
- **E0210 / Orphan Rule:** The `impl BitOr for BindType` in `src/function.rs` violated the orphan rule and caused an E0210 warning (future error). Decided to remove the implementation for now to resolve the warning. Direct `bind` calls must be used instead.

## Important Patterns and Preferences
- **Documentation First:** Keeping Memory Bank updated.
- **Systematic Refinement & Testing:** Core `Option<T>` and `Result<T, E>` implementations verified with law tests.

## Learnings and Project Insights
- Key learnings from completed development phases have been archived. See [Project History](./archive/project_history_pre_aug_2025.md) for details.
- Current focus is on applying best practices for documentation and ensuring the library is easy to understand and use.
