#![doc = include_str!("../README.md")]
// Add other crate-level attributes if needed, e.g.:
// #![deny(missing_docs)] // Enforce documentation for all public items

// Module declarations
pub mod applicative;
pub mod apply;
pub mod function;
pub mod functor;
pub mod identity; // Added
pub mod kind_based; // No longer cfg-gated
pub mod monad;
pub mod profunctor;
pub mod transformers;
pub mod utils;
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
// So, `use fp_rs::fn0;` etc., should work without explicit re-export here.
// If they were not `#[macro_export]`, they would need to be re-exported like:
// pub use utils::fn0; // (if fn0 was not #[macro_export])

// Example of how to conditionally compile and export:
// #[cfg(feature = "experimental")]
// pub use experimental_apply::ExperimentalApply;
