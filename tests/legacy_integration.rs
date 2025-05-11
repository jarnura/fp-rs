#![cfg(feature = "legacy")] // Only compile and run these tests if "legacy" feature is active

// Declare the legacy test modules.
// The test files themselves (e.g., tests/legacy/applicative.rs)
// should also be gated with `#![cfg(all(test, feature = "legacy"))]`
// or have their inner modules/tests gated.

#[cfg(test)]
#[path = "legacy/applicative.rs"]
mod applicative;

#[cfg(test)]
#[path = "legacy/apply.rs"]
mod apply;

#[cfg(test)]
#[path = "legacy/functor.rs"]
mod functor;

#[cfg(test)]
#[path = "legacy/identity.rs"]
mod identity;

#[cfg(test)]
#[path = "legacy/monad.rs"]
mod monad;

#[cfg(test)]
#[path = "legacy/transformers/mod.rs"] // Points to the mod file in the legacy transformers directory
mod transformers;
// The `mod transformers` above will correctly load `tests/legacy/transformers/mod.rs`,
// which in turn contains `pub mod reader_test;`. This setup should correctly
// find `tests/legacy/transformers/reader_test.rs`.
