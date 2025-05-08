# System Patterns

## System Architecture
- **Library Crate:** The project is structured as a Rust library crate, intended to be used as a dependency in other Rust projects.
- **Modular Design:** Functional programming concepts (Functor, Applicative, Monad, etc.) are organized into separate modules within the `src/` directory (e.g., `functor.rs`, `applicative.rs`, `monad.rs`).
- **Trait-based Generics:** Core abstractions are defined using Rust traits, allowing various data types to implement these functional behaviors. This promotes polymorphism and code reuse.
- **Testing Structure:** Integration tests are located in the top-level `tests/` directory, with files mirroring the module structure under `src/` (e.g., `tests/functor.rs` tests `src/functor.rs`). This follows standard Rust conventions.

## Key Technical Decisions
- **Focus on Idiomatic Rust:** Prioritize solutions that align with Rust's ownership, borrowing, and type system principles. Avoid direct translations from other languages if they result in unidiomatic Rust.
- **Leverage Existing Rust Types:** Where appropriate, implement monadic traits for standard Rust types like `Option<T>`, `Result<T, E>`, and `Vec<T>` to showcase their monadic nature and provide immediate utility.
- **Higher-Kinded Types (HKT) Emulation:** Since Rust does not natively support Higher-Kinded Types, the project will likely use common workarounds or emulations (e.g., associated types, specific trait patterns) to achieve a degree of generic programming over type constructors. This is a critical area and decisions here will significantly impact the library's design. (See also: [HKT Constraints in Tech Context](./techContext.md#higher-kinded-types-hkts))
- **Minimal Dependencies:** Aim to keep external dependencies to a minimum to ensure the library is lightweight and easy to integrate.

## Design Patterns in Use
- **Trait-based Polymorphism:** As mentioned, traits are central to defining and implementing functional interfaces.
- **Type Classes:** The traits for `Functor`, `Applicative`, `Monad`, etc., act as type classes, defining behavior for types that can instantiate them.
- **Composition over Inheritance:** Functional patterns will be achieved through composition of functions and types, rather than classical inheritance.

## Component Relationships
- `functor.rs`: Defines the `Functor` trait (`map` operation). This is a foundational concept.
- `apply.rs`: Defines the `Apply` trait (`ap` operation), building upon `Functor`.
- `applicative.rs`: Defines the `Applicative` trait (`pure` or `of` operation, along with `ap` from `Apply`), building upon `Apply`.
- `monad.rs`: Defines the `Monad` trait (`flat_map` or `bind`, and `pure`/`return` often inherited from `Applicative`), building upon `Applicative`.
- `lib.rs` / `main.rs`: Entry points for the library/application, re-exporting modules and potentially providing top-level examples or utilities.
- `utils.rs`: May contain helper functions or common type definitions used across different modules.
- `function.rs`: Likely contains utilities or traits related to function manipulation, currying, composition, etc., which are often used in functional programming.
- `profunctor.rs`: Implements the `Profunctor` concept, which is related but distinct from Functors and Monads, dealing with bifunctorial mappings.

## Critical Implementation Paths
- **Defining the core traits:** `Functor`, `Apply`, `Applicative`, `Monad` traits with their associated methods.
- **Implementing these traits for `Option<T>`:** A canonical example for demonstrating monadic behavior.
- **Implementing these traits for `Result<T, E>`:** Another key Rust type that exhibits monadic properties, especially for error handling.
- **Implementing these traits for `Vec<T>`:** Demonstrating how collections can also be monadic (e.g., for non-deterministic computations).
- **Testing:** Ensuring all laws for Functor, Applicative, and Monad are upheld by the implementations through comprehensive integration tests located in the `tests/` directory.
