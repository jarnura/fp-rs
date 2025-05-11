// Module gated by "legacy" feature in src/lib.rs

//! # Legacy Implementations
//!
//! This module contains the older, "classic" implementations of functional
//! programming traits that were used before the HKT (Higher-Kinded Types)
//! refactor. These are provided for backward compatibility or reference
//! and are only compiled if the `legacy` feature is enabled.
//!
//! To use these, enable the `legacy` feature in your `Cargo.toml`:
//! ```toml
//! # Cargo.toml
//! # [dependencies]
//! # monadify = { version = "0.1.0", features = ["legacy"] }
//! ```
//!
//! Then, you can access them via their respective paths, e.g.:
//! `use monadify::legacy::functor::Functor as LegacyFunctor;`

/// Legacy `Applicative` trait and implementations.
pub mod applicative;
/// Legacy `Apply` trait and implementations.
pub mod apply;
/// Legacy `Functor` trait and implementations.
pub mod functor;
/// Legacy `Identity` monad implementation.
pub mod identity;
/// Legacy `Monad` and `Bind` traits and implementations.
pub mod monad;
/// Legacy monad transformers, e.g., `ReaderT`.
pub mod transformers; // This will contain the legacy reader module

// Optional: Re-export legacy traits/structs with a `Legacy` prefix
// to avoid name clashes if both HKT (default) and legacy items are in scope.
// Example:
// pub use functor::Functor as LegacyFunctor;
// pub use apply::Apply as LegacyApply;
// pub use applicative::Applicative as LegacyApplicative;
// pub use monad::{Bind as LegacyBind, Monad as LegacyMonad};
// pub use identity::Identity as LegacyIdentity;
// pub use transformers::reader::ReaderT as LegacyReaderT;
// pub use transformers::reader::Reader as LegacyReader;
// pub use transformers::reader::MonadReader as LegacyMonadReader;

// For now, users will access them via full path like `monadify::legacy::functor::Functor`.
