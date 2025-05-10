#![cfg(feature = "kind")]

//! # Higher-Kinded Type (HKT) Infrastructure
//!
//! This module provides the core traits and marker types for simulating
//! Higher-Kinded Types in Rust. This allows for abstracting over type
//! constructors like `Option`, `Vec`, `Result<_, E>`, etc., which is
//! fundamental for defining generic functional programming traits like
//! `Functor`, `Applicative`, and `Monad` in a way that they can operate
//! over these different type constructors.
//!
//! The primary mechanism used is a combination of:
//! 1.  A central `HKT` trait with a Generic Associated Type (GAT) `Applied<Arg>`.
//! 2.  Marker structs (e.g., `OptionHKTMarker`) that implement `HKT` and specify
//!     what `Applied<Arg>` resolves to for their respective type constructor.
//!
//! This setup enables traits to be generic over the *marker type*, and through
//! the marker's `Applied<Arg>` GAT, they can refer to the concrete type
//! (e.g., `Option<String>`, `Vec<i32>`).

use crate::function::{CFn, CFnOnce};
use std::marker::PhantomData;

/// Represents a Higher-Kinded Type (HKT) constructor.
///
/// This trait is the cornerstone of the HKT simulation. It allows abstracting
/// over type constructors that take one type argument, such as `Option<_>`,
/// `Vec<_>`, or `Result<_, E>` (where `E` is fixed).
///
/// Implementors of this trait are typically lightweight "marker" structs
/// (e.g., [`OptionHKTMarker`], [`VecHKTMarker`]) that don't hold data themselves
/// but serve to identify the type constructor they represent.
pub trait HKT {
    /// The concrete type resulting from applying this HKT constructor to a type argument `Arg`.
    ///
    /// For example:
    /// - If `Self` is [`OptionHKTMarker`], then `Self::Applied<Arg>` is `Option<Arg>`.
    /// - If `Self` is [`VecHKTMarker`], then `Self::Applied<Arg>` is `Vec<Arg>`.
    /// - If `Self` is [`ResultHKTMarker<MyError>`], then `Self::Applied<Arg>` is `Result<Arg, MyError>`.
    type Applied<Arg>: Sized;
}

// --- Marker Structs for common HKTs ---

/// Marker for the `Option` type constructor.
///
/// Implements [`HKT`] such that `OptionHKTMarker::Applied<T>` resolves to `Option<T>`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct OptionHKTMarker;
impl HKT for OptionHKTMarker {
    type Applied<Arg> = Option<Arg>;
}

/// Marker for the `Vec` type constructor.
///
/// Implements [`HKT`] such that `VecHKTMarker::Applied<T>` resolves to `Vec<T>`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct VecHKTMarker;
impl HKT for VecHKTMarker {
    type Applied<Arg> = Vec<Arg>;
}

/// Marker for the `Result<T, E>` type constructor, where `E` (the error type) is fixed.
///
/// `ResultHKTMarker<E>` acts as the constructor for `Result<_, E>`.
/// Implements [`HKT`] such that `ResultHKTMarker<E>::Applied<T>` resolves to `Result<T, E>`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ResultHKTMarker<E>(std::marker::PhantomData<E>);

impl<E> ResultHKTMarker<E> {
    /// Creates a new marker for `Result<_, E>`.
    /// This is primarily for type inference or explicit construction if needed,
    /// though often `Default::default()` or type inference is sufficient.
    pub fn new() -> Self {
        ResultHKTMarker(std::marker::PhantomData)
    }
}

impl<E> HKT for ResultHKTMarker<E> {
    type Applied<Arg> = Result<Arg, E>;
}

/// HKT Marker for `CFn<X, _>`. `X` is the fixed input type of the function.
///
/// Implements [`HKT`] such that `CFnHKTMarker<X>::Applied<Output>` resolves to `CFn<X, Output>`.
#[derive(Default)] // CFnHKTMarker itself doesn't need Debug, Clone etc. unless used directly in ways that require it.
pub struct CFnHKTMarker<X>(PhantomData<X>);

impl<X> HKT for CFnHKTMarker<X> {
    type Applied<Output> = CFn<X, Output>;
}

/// HKT Marker for `CFnOnce<X, _>`. `X` is the fixed input type of the function.
///
/// Implements [`HKT`] such that `CFnOnceHKTMarker<X>::Applied<Output>` resolves to `CFnOnce<X, Output>`.
#[derive(Default)]
pub struct CFnOnceHKTMarker<X>(PhantomData<X>);

impl<X> HKT for CFnOnceHKTMarker<X> {
    type Applied<Output> = CFnOnce<X, Output>;
}

// --- Arity Markers ---

/// Marks an `HKT` that effectively takes one type argument (e.g., `F<A>`).
///
/// In this library's current HKT simulation, all types implementing [`HKT`]
/// (which defines `type Applied<Arg>`) inherently fit this "arity-1" concept.
/// This trait serves as a blanket implementation for all `T: HKT`, providing
/// a convenient way to specify this arity in trait bounds if needed, although
/// often just `T: HKT` is sufficient.
pub trait HKT1: HKT {}
impl<T: HKT> HKT1 for T {} // Blanket implementation

// If HKTs with more complex arities were needed, e.g., for Bifunctor `F<A, B>`:
// pub trait HKT2 {
//     type Applied<Arg1, Arg2>: Sized;
// }
// For now, `HKT` with a single `Applied<Arg>` GAT covers Functor, Applicative, Monad.

// The `concretize` function from the original sketch could be added here if useful.
// It would simply be an identity function on `Self::Applied<Arg>`.
// pub trait HKTConcretize: HKT {
//     fn concretize<Arg>(value: Self::Applied<Arg>) -> Self::Applied<Arg> {
//         value
//     }
// }
// impl<T: HKT> HKTConcretize for T {}
