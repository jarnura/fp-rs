# Project Progress

## What Works
- **Core Monadic Implementations:**
    - `Option<A>`: Functor, Apply, Applicative, Monad traits and laws implemented and tested.
    - `Result<T, E>`: Functor, Apply, Applicative, Monad traits and laws implemented and tested.
    - `Vec<T>`: Functor, Apply, Applicative, Monad traits and laws implemented and tested.
- **Profunctor Implementation:**
    - `Profunctor`, `Strong`, `Choice` traits and laws implemented and tested for `CFn`.
- **Codebase Health:**
    - All (83 unit + 3 doc) tests passing.
    - Code formatted with `cargo fmt`.
    - Linted with `cargo clippy` (no warnings beyond intentionally suppressed ones).
    - All compiler warnings addressed.
- **Memory Bank:**
    - Core documents established.
    - Archive for historical project data created: [Project History](./archive/project_history_pre_aug_2025.md).
- **Documentation (Phase 3, Step 3 - Largely Complete):**
    - Added comprehensive `rustdoc` comments to public items in `functor.rs`, `apply.rs`, `applicative.rs`, `monad.rs`, `function.rs`, `profunctor.rs`, `utils.rs` (macros), and `lib.rs`.
    - `README.md` significantly updated with project overview, features, usage examples, build/test instructions, and benchmark summary.
    - Crate-level documentation linked to `README.md` via `#![doc = include_str!("../README.md")]` in `lib.rs`.
- For detailed history of completed work, see the [Project History](./archive/project_history_pre_aug_2025.md).

## What's Left to Build / Planned (Key Priorities)
- **Phase 3, Step 3: Documentation (Final Review):**
    - Review `rustdoc` for `src/experimental_apply.rs` and decide on its public API status (document, feature-gate, or privatize).
    - General review of all generated `rustdoc` for clarity and completeness.
- **Benchmarking (Initial Analysis Complete, Ongoing):**
    - Benchmarks for `Functor::map`, `Apply::apply`, and `Bind::bind` implemented for `Option`, `Result`, and `Vec` using `criterion.rs`.
    - **`Functor::map` Overhead:**
        - `Option`: Negligible.
        - `Result`: Negligible.
        - `Vec`: Moderate overhead for by-value `map` compared to native iterator-based map (~5-7ns for 100 elements).
    - **`Apply::apply` Overhead** (function involves `Box::new`):
        - `Option`: ~2.2 ns compared to manual `match`.
        - `Result`: ~2.3 ns compared to manual `match`.
        - `Vec`: ~4.2 ns per effective operation (value-function pair) compared to manual loops.
    - **`Bind::bind` Overhead:**
        - `Option`: Negligible.
        - `Result`: Negligible.
        - `Vec`: Moderate overhead for by-value `bind` compared to native `flat_map` (~20ns for 10 input elements producing 20 output elements).
    - **Overall:** `Bind` and `Functor` (for `Option`/`Result`) show excellent performance. `Apply` has a consistent small overhead (a few ns). `Vec` operations show more overhead due to by-value semantics and less optimization potential compared to iterator chains.
- **`src/main.rs`:** Define purpose (e.g., examples, CLI) (lower priority).
- **`Cargo.toml` Metadata:** Update author and URL information (lower priority).
- **Revisit `BitOr` for `BindType`:** Consider alternative designs if operator syntax is desired (lower priority).


## Current Status
- **Phase:** Phase 3, Step 3 (Documentation - Final Review).
- **Focus:** Finalizing documentation, particularly for `experimental_apply.rs`.
- **Overall Test Status:** 83 unit tests + 3 doc tests passing. Code is clean and formatted. Documentation for core modules added.

## Known Issues
- `src/experimental_apply.rs` documentation status pending review.
- `src/main.rs` is currently a placeholder.
- Further refinement of `Vec` benchmarks or implementations could be considered if `Vec` performance is critical.
- `Apply::apply` overhead, while small, is the most notable among the core operations for `Option` and `Result`.
- Placeholder metadata (author, URLs) in `Cargo.toml`.

## Evolution of Project Decisions
- Historical project decisions and their evolution are documented in the [Project History](./archive/project_history_pre_aug_2025.md).
- Current decision-making focuses on effective documentation strategies and API clarity.
