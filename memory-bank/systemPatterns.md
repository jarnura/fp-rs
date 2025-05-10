# System Patterns

## System Architecture
- **Library Crate:** The project is structured as a Rust library crate, intended to be used as a dependency in other Rust projects.
- **Modular Design:** Functional programming concepts (Functor, Apply, Applicative, Bind, etc.) are organized into separate modules (e.g., `functor.rs`, `apply.rs`). These modules are further structured to support the HKT refactoring:
    - Each conceptual trait (e.g., Functor) now primarily uses an `hkt` submodule for the Higher-Kinded Type approach (marker traits and GATs). The `classic` submodules (older, associated type-based approach) are retained but gated by `#[cfg(not(feature = "kind"))]`.
    - The HKT system itself is defined in `src/kind_based/`.
    - Monad transformers (e.g., `ReaderT`) in `src/transformers/` have been refactored for HKT.
- **Trait-based Generics:** Core abstractions are defined using Rust traits, allowing various data types to implement these functional behaviors. This promotes polymorphism and code reuse.
- **Testing Structure:**
    - Classic implementation tests are in the top-level `tests/` directory (e.g., `tests/functor.rs`), gated by `#[cfg(not(feature = "kind"))]`.
    - HKT implementation tests are located in the `tests/hkt/` subdirectory (e.g., `tests/hkt/functor.rs`), organized by trait. These are pulled into the test suite via `tests/hkt_integration.rs` and are gated by `#[cfg(feature = "kind")]`.
    - This structure ensures clear separation and conditional compilation of tests based on the active feature set.

## Key Technical Decisions
- **Focus on Idiomatic Rust:** Prioritize solutions that align with Rust's ownership, borrowing, and type system principles. Avoid direct translations from other languages if they result in unidiomatic Rust.
- **HKT as Primary Abstraction (with `kind` feature):** The HKT system is now the primary way of defining and using Functor, Apply, Applicative, Bind, and Monad traits when the `kind` feature is enabled.
- **Leverage Existing Rust Types:** Where appropriate, implement monadic traits for standard Rust types like `Option<T>`, `Result<T, E>`, and `Vec<T>` to showcase their monadic nature and provide immediate utility.
- **Higher-Kinded Types (HKT) Implementation:** The project implements a Higher-Kinded Type system using:
    - **Marker Traits:** `HKT` and `HKT1` defined in `src/kind_based/kind.rs`. `HKT1` is a marker for type constructors that take one type argument (e.g., `Option<_>`).
    - **Generic Associated Types (GATs):** The `HKT1` trait defines an associated type `type Applied<T>`, which represents the concrete type after applying the type argument `T` (e.g., for `OptionHKTMarker`, `Self::Applied<String>` would be `Option<String>`).
    - **Marker Types:** Specific structs (e.g., `OptionHKTMarker`, `VecHKTMarker`, `CFnHKTMarker<X>`, `IdentityHKTMarker`, `ReaderTHKTMarker<R, MMarker>`) implement `HKT1` to act as representatives for their respective type constructors.
This approach allows traits like `hkt::Functor` to be generic over the HKT marker. This system is now stable and implemented through `Monad` for `Option`, `Result`, `Vec`, `CFn`, `CFnOnce`, `Identity`, and `ReaderT`. (See also: [HKT Constraints in Tech Context](./techContext.md#higher-kinded-types-hkts))
- **Minimal Dependencies:** Aim to keep external dependencies to a minimum to ensure the library is lightweight and easy to integrate.

## Design Patterns in Use
- **Trait-based Polymorphism:** As mentioned, traits are central to defining and implementing functional interfaces.
- **Type Classes:** The traits for `Functor`, `Applicative`, `Monad`, etc., act as type classes, defining behavior for types that can instantiate them.
- **Composition over Inheritance:** Functional patterns will be achieved through composition of functions and types, rather than classical inheritance.
- **Monad Transformers:** Patterns like `ReaderT` allow augmenting existing monads with new capabilities (e.g., access to a read-only environment).
    - **`ReaderT<R, M, A>`:** A monad transformer that wraps an inner monad `M` and provides access to a read-only environment of type `R`. Computations are of the form `R -> M<A>`. It implements `Functor`, `Apply`, `Applicative`, and `Monad` if the inner monad `M` does. It also implements `MonadReader` to provide `ask` (to get the environment) and `local` (to run a computation with a modified environment).
    - **`Identity<A>`:** A simple monad that just wraps a value. Often used as the base monad for simpler versions of transformers, e.g., `Reader<R, A>` is `ReaderT<R, Identity<A>, A>`.

## Component Relationships
- `src/kind_based/kind.rs`: Defines the core HKT infrastructure, including the `HKT<A>`, `HKT1` traits, and various HKT marker types (e.g., `OptionHKTMarker`, `CFnHKTMarker`). This is foundational for the new HKT system.
- `src/kind_based/mod.rs`: Module file for the `kind_based` system.
- `functor.rs`: Defines `classic::Functor` (gated) and `hkt::Functor<A, B>` (primary HKT style).
- `apply.rs`: Defines `classic::Apply` (gated) and `hkt::Apply<A, B>` (primary HKT style).
- `applicative.rs`: Defines `classic::Applicative` (gated) and `hkt::Applicative<T>` (primary HKT style).
- `monad.rs`: Defines `classic::Bind` and `classic::Monad` (gated). The `hkt::Bind<A, B>` and `hkt::Monad<A>` traits are the primary HKT versions. The `hkt::Monad` trait is now fully implemented and used.
- `identity.rs`: Defines `Identity<A>` and `IdentityHKTMarker`, with HKT trait implementations.
- `transformers/reader.rs`: Defines `ReaderT<R, MMarker, A>` and `ReaderTHKTMarker<R, MMarker>`, with HKT trait implementations including `MonadReader`.
- `lib.rs`: Main library file, re-exporting HKT traits and types when the `kind` feature is active.
- `utils.rs`: Helper functions and macros. `bfn!` macros are currently commented out.
- `function.rs`: Defines `CFn` and `CFnOnce`.
- `profunctor.rs`: Implements `Profunctor`, `Strong`, `Choice`.
- `transformers/mod.rs`: Module for organizing monad transformers.

## Critical Implementation Paths
- **HKT Refactoring (Completed):**
    - Defining core HKT traits (`HKT`, `HKT1`) and markers.
    - Refactoring `Functor`, `Apply`, `Applicative`, `Bind`, `Monad` to use the HKT pattern.
    - Updating `Option`, `Result`, `Vec`, `CFn`, `CFnOnce`, `Identity`, and `ReaderT` to implement these HKT traits.
- **Testing (Completed for HKT):**
    - Ensuring all HKT laws for Functor, Applicative, and Monad are upheld by the implementations through comprehensive integration tests in `tests/hkt/`. (Note: Some Applicative laws for `CFn`-like types are untestable due to `Clone` constraints).
