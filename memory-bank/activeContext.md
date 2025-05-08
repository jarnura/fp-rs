# Active Context

## Current Work Focus
- **Phase 3, Step 3: Comprehensive Documentation.** Focus on writing `rustdoc` comments and updating `README.md`.

## Recent Changes
- **Phase 3, Step 4 (Final Review & Cleanup) Completed:**
    - Ran `cargo check` and `cargo clippy -- -D warnings`.
    - Resolved E0210 warnings in `src/function.rs` by removing the problematic `impl BitOr for BindType`.
    - Silenced dead code warning for `Monad` trait in `src/monad.rs` using `#[allow(dead_code)]`.
    - Removed dead code (`identity` function) in `tests/profunctor.rs`.
    - Fixed/suppressed all `unused_imports` and `unused_variables` warnings across test files. Necessary `Functor` imports were kept but warnings suppressed with `#[allow(unused_imports)]`.
    - Ran `cargo fmt` to format the codebase.
    - Reviewed `Cargo.toml`, removed inaccurate "no-std" category. Metadata TODOs remain.
    - Verified all changes with `cargo test` (83 unit tests + 3 doc tests passed without warnings).
- **Phase 3, Step 2 (Profunctor & Optics Review and Testing) Completed:**
    - Reviewed `src/profunctor.rs`.
    - Fixed failing `Strong` and `Choice` law tests in `tests/profunctor.rs` by correcting assertion values. The issue was a miscalculation of string length in manual traces.
    - All Profunctor, Strong, and Choice law tests are now passing.
    - Reviewed `lens_` and `lens` implementations in `src/profunctor.rs`.
    - Commented out unused code: `dimap_` method in `Profunctor` trait and its implementations, and `Optic_`, `Lens_` structs in `src/profunctor.rs`.
    - Added a new `strong_associativity_law` test to `tests/profunctor.rs`, which is passing.
    - Verified all changes with `cargo test` (83 unit tests + 3 doc tests passed).
- **Phase 3, Step 1 (Vec<T> Implementation & Tests) Completed:**
    - Implemented `Functor` for `Vec<T>` and added Functor law tests.
    - Implemented `Apply` for `Vec<T>` and added Apply law tests.
    - Implemented `Applicative` for `Vec<T>` and added Applicative law tests.
    - Implemented `Bind` (and `Monad`) for `Vec<T>` and added Monad law tests.
    - Verified changes with `cargo test`.
- **Phase 2 Completed:**
    - Implemented `Functor` for `Result<T, E>` and added Functor law tests.
    - Implemented `Apply` for `Result<T, E>`.
    - Implemented `Applicative` for `Result<T, E>` and added Applicative law tests.
    - Implemented `Bind` (and `Monad`) for `Result<T, E>` and added Monad law tests.
    - Resolved compilation issues related to `Clone` bounds by reverting `CFn` to use `Box` and adjusting tests to recreate `CFn` instances.
    - Verified changes with `cargo test` (56 tests passed at that time).
- Switched project from nightly to stable Rust toolchain.
- Added public re-exports for core traits to `src/lib.rs`.
- Completed Phase 1 Refinements & `Option<A>` law tests.
- Completed initial setup of Memory Bank core documents & code review.
- **Refactored Tests:** Moved all `#[cfg(test)] mod tests { ... }` blocks from `src/` files (`functor.rs`, `apply.rs`, `applicative.rs`, `monad.rs`, `profunctor.rs`) into separate files within a new top-level `tests/` directory. Adjusted `use` statements accordingly. Verified with `cargo test` (56 tests + 3 doc tests passed at that time).


## Next Steps
- **Phase 3, Step 3: Comprehensive Documentation.** This includes:
    - Writing `rustdoc` comments for all public items in the library.
    - Updating `README.md` with project status, usage examples, and API overview.
- **Post-Phase 3 Considerations (Lower Priority):**
    - **Benchmarking:** Set up and use `criterion.rs`.
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
- Cloning boxed trait objects (`Box<dyn Fn>`) is non-trivial. Using `Arc` introduced other complexities (`Send`/`Sync`). Reverting to `Box` and adjusting tests was the chosen path for earlier `Clone` issues.
- The `Profunctor`, `Strong`, and `Choice` implementations for `CFn` are correct; test failures were due to incorrect assertion values based on manual trace miscalculations.
- Phase 1 (`Option<T>`), Phase 2 (`Result<T, E>`), Phase 3 Step 1 (`Vec<T>`), and Phase 3 Step 2 (Profunctor/Optics) implementations are complete and verified by law tests.
- Phase 3 Step 4 (Cleanup) is complete. Codebase is formatted, linted, and passes all tests without warnings (other than suppressed ones).
- Project is stable and ready for Phase 3, Step 3 (Documentation).
- Compiler warnings for `unused_imports` related to traits needed for method resolution (e.g., `Functor` for `.map()`) in integration tests (`tests/` dir) can sometimes be misleading. Imports were confirmed necessary via compilation errors when removed, so warnings were suppressed locally.
