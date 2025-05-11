#![doc = include_str!("../README.md")]
// Add other crate-level attributes if needed, e.g.:
#![deny(missing_docs)] // Enforce documentation for all public items

// Module declarations


/// Provides the Kind-based `Applicative` trait and its implementations for the `monadify` library.
pub mod applicative;
/// Provides the Kind-based `Apply` trait (an extension of `Functor`) and its implementations.
pub mod apply;
/// Defines `CFn` and `CFnOnce` for heap-allocated, callable function wrappers.
pub mod function;
/// Provides the Kind-based `Functor` trait and its implementations.
pub mod functor;
/// Defines the `Identity` monad and its Kind marker.
pub mod identity;
/// Core infrastructure for Kind-based programming (Higher-Kinded Types), including `Kind` and `Kind1` traits,
/// and various Kind marker types (e.g., `OptionKind`).
pub mod kind_based;
/// Provides the Kind-based `Monad` and `Bind` traits and their implementations.
pub mod monad;
/// Implements `Profunctor`, `Strong`, and `Choice` traits, primarily for function types.
pub mod profunctor;
/// Contains monad transformers like `ReaderT`.
pub mod transformers;
/// Utility functions and macros, including `fn0!`, `fn1!`, etc.
pub mod utils;

/// Contains legacy (non-Kind-based, associated type-based) implementations of functional traits.
/// This module is only available when the `legacy` feature is enabled.
#[cfg(feature = "legacy")]
pub mod legacy;

// Public re-exports of core traits (now default to Kind-based versions)
pub use applicative::Applicative; // Points to applicative::kind::Applicative
pub use apply::Apply;             // Points to apply::kind::Apply
pub use functor::Functor;         // Points to functor::kind::Functor
pub use monad::{Bind, Monad};     // Points to monad::kind::Bind and monad::kind::Monad
pub use profunctor::{Choice, Profunctor, Strong};
pub use transformers::reader::MonadReader; // Points to transformers::reader::kind::MonadReader

// Public re-exports of key structs/types (optional, but can be convenient)
pub use function::{CFn, CFnOnce};
pub use identity::Identity; // Points to identity::kind::Identity
pub use transformers::reader::{ReaderT, Reader}; // Points to transformers::reader::kind::ReaderT etc.

// Re-export Kind markers and core Kind traits by default
pub use kind_based::kind::{
    Kind, Kind1, // Core Kind traits
    OptionKind, ResultKind, VecKind,
    CFnKind, CFnOnceKind
};
pub use crate::identity::IdentityKind; // Changed from IdentityHKTMarker
pub use crate::transformers::reader::ReaderTKind; // Changed from ReaderTHKTMarker
// Reader alias is re-exported above.

// Note on macros:
// Macros defined with `#[macro_export]` in submodules (like `utils.rs`) are
// automatically available at the crate root.
// So, `use monadify::fn0;` etc., should work without explicit re-export here.
// If they were not `#[macro_export]`, they would need to be re-exported like:
// pub use utils::fn0; // (if fn0 was not #[macro_export])

// Example of how to conditionally compile and export:
// #[cfg(feature = "experimental")]
// pub use experimental_apply::ExperimentalApply;
