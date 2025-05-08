#![doc = include_str!("../README.md")]
// Add other crate-level attributes if needed, e.g.:
// #![deny(missing_docs)] // Enforce documentation for all public items

// Module declarations
pub mod applicative;
pub mod apply;
pub mod experimental_apply; // Consider if this should be public or feature-gated
pub mod function;
pub mod functor;
pub mod monad;
pub mod profunctor;
pub mod utils;

// Public re-exports of core traits
pub use applicative::Applicative;
pub use apply::Apply;
pub use functor::Functor;
pub use monad::{Bind, Monad}; // Re-export Monad as well
pub use profunctor::{Choice, Profunctor, Strong};

// Public re-exports of key structs/types (optional, but can be convenient)
pub use function::{CFn, CFnOnce};

// Note on macros:
// Macros defined with `#[macro_export]` in submodules (like `utils.rs`) are
// automatically available at the crate root.
// So, `use fp_rs::fn0;` etc., should work without explicit re-export here.
// If they were not `#[macro_export]`, they would need to be re-exported like:
// pub use utils::fn0; // (if fn0 was not #[macro_export])

// Example of how to conditionally compile and export:
// #[cfg(feature = "experimental")]
// pub use experimental_apply::ExperimentalApply;
