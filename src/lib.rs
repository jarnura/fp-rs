#![doc = include_str!("../README.md")]
// Add other crate-level attributes if needed, e.g.:
#![deny(missing_docs)] // Enforce documentation for all public items

// Module declarations

/// Provides the HKT `Applicative` trait and its implementations.
pub mod applicative;
/// Provides the HKT `Apply` trait (an extension of `Functor`) and its implementations.
pub mod apply;
/// Defines `CFn` and `CFnOnce` for heap-allocated, callable function wrappers.
pub mod function;
/// Provides the HKT `Functor` trait and its implementations.
pub mod functor;
/// Defines the `Identity` monad and its HKT marker.
pub mod identity; // Added
/// Core infrastructure for Higher-Kinded Types (HKTs), including `HKT` and `HKT1` traits,
/// and various HKT marker types (e.g., `OptionHKTMarker`).
pub mod kind_based; // No longer cfg-gated
/// Provides the HKT `Monad` and `Bind` traits and their implementations.
pub mod monad;
/// Implements `Profunctor`, `Strong`, and `Choice` traits, primarily for function types.
pub mod profunctor;
/// Contains monad transformers like `ReaderT`.
pub mod transformers;
/// Utility functions and macros, including `fn0!`, `fn1!`, etc.
pub mod utils;

/// Contains legacy (non-HKT, associated type-based) implementations of functional traits.
/// This module is only available when the `legacy` feature is enabled.
#[cfg(feature = "legacy")]
pub mod legacy; // Added legacy module

// Public re-exports of core traits (now default to HKT versions)
pub use applicative::Applicative;
pub use apply::Apply;
pub use functor::Functor;
pub use monad::{Bind, Monad}; // Assuming HKT Monad will be re-exported from monad.rs
pub use profunctor::{Choice, Profunctor, Strong};
pub use transformers::reader::MonadReader; // Added

// Public re-exports of key structs/types (optional, but can be convenient)
pub use function::{CFn, CFnOnce};
pub use identity::Identity; // This now points to HKT Identity
pub use transformers::reader::{ReaderT, Reader}; // This now points to HKT ReaderT and Reader alias

// Re-export HKT markers by default
pub use kind_based::kind::{
    HKT, HKT1, // Core HKT traits
    OptionHKTMarker, ResultHKTMarker, VecHKTMarker, 
    CFnHKTMarker, CFnOnceHKTMarker
};
pub use crate::identity::IdentityHKTMarker;
pub use crate::transformers::reader::ReaderTHKTMarker;
// Note: ReaderTHKTMarker was not previously re-exported, adding it.
// Reader alias is now re-exported above.

// Note on macros:
// Macros defined with `#[macro_export]` in submodules (like `utils.rs`) are
// automatically available at the crate root.
// So, `use monadify::fn0;` etc., should work without explicit re-export here.
// If they were not `#[macro_export]`, they would need to be re-exported like:
// pub use utils::fn0; // (if fn0 was not #[macro_export])

// Example of how to conditionally compile and export:
// #[cfg(feature = "experimental")]
// pub use experimental_apply::ExperimentalApply;
