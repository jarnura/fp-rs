// src/kind_based/mod.rs
// This file will declare all submodules within kind_based.

// Conditionally compiled with the 'kind' feature.
// The parent module (kind_based in lib.rs) is already cfg'd,
// but being explicit here for submodules is good practice
// if they could potentially be used elsewhere or if the top-level cfg changes.
#[cfg(feature = "kind")]
pub mod kind;
