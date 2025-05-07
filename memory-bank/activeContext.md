# Active Context

## Current Work Focus
- **Completed Phase 2: Result<T, E> implementations and tests.** Planning next phase.

## Recent Changes
- **Phase 2 Completed:**
    - Implemented `Functor` for `Result<T, E>` and added Functor law tests.
    - Implemented `Apply` for `Result<T, E>`.
    - Implemented `Applicative` for `Result<T, E>` and added Applicative law tests.
    - Implemented `Bind` (and `Monad`) for `Result<T, E>` and added Monad law tests.
    - Resolved compilation issues related to `Clone` bounds by reverting `CFn` to use `Box` and adjusting tests to recreate `CFn` instances.
    - Verified changes with `cargo test` (56 tests passed).
- Switched project from nightly to stable Rust toolchain.
- Added public re-exports for core traits to `src/lib.rs`.
- Completed Phase 1 Refinements & `Option<A>` law tests.
- Completed initial setup of Memory Bank core documents & code review.
- **Refactored Tests:** Moved all `#[cfg(test)] mod tests { ... }` blocks from `src/` files (`functor.rs`, `apply.rs`, `applicative.rs`, `monad.rs`, `profunctor.rs`) into separate files within a new top-level `tests/` directory. Adjusted `use` statements accordingly. Verified with `cargo test` (56 tests + 3 doc tests passed).


## Next Steps
- **Plan Next Phase (Phase 3):** Decide on next steps, likely focusing on:
    - Implementing traits and law tests for `Vec<T>`.
    - Reviewing/testing Profunctor and Optics laws.
    - Documentation.

## Active Decisions and Considerations
- **`CFn` Clonability:** Decided against making `CFn` (holding `Box<dyn Fn>`) easily `Clone` due to complexity. Tests requiring multiple uses of the same function now recreate the `CFn` instance. This might need revisiting if it becomes a major ergonomic issue.
- **Test Module Organization:** Integration tests are now located in the top-level `tests/` directory, mirroring the module structure of `src/`. Law tests remain organized in submodules within these test files.
- **Warnings:** Acknowledged remaining warnings (dead code, E0210, unused imports/variables in lib, unused imports in tests) as lower priority for now.

## Important Patterns and Preferences
- **Documentation First:** Keeping Memory Bank updated.
- **Systematic Refinement & Testing:** Core `Option<T>` and `Result<T, E>` implementations verified with law tests.

## Learnings and Project Insights
- Cloning boxed trait objects (`Box<dyn Fn>`) is non-trivial. Using `Arc` introduced other complexities (`Send`/`Sync`). Reverting to `Box` and adjusting tests was the chosen path.
- Phase 1 (`Option<T>`) and Phase 2 (`Result<T, E>`) implementations are complete and verified.
- Project is stable and ready for Phase 3.
