#[cfg(not(feature = "kind"))]
mod classic {
    use crate::function::{CFn, CFnOnce};

    /// Represents a type constructor that can be mapped over.
    ///
    /// The `Functor` trait provides a `map` operation that allows a function `A -> B`
    /// to be applied to a value within a context `F<A>`, producing a value in a
    /// context `F<B>`. This is a fundamental concept in functional programming
    /// for working with "boxed" or "contextual" values.
    ///
    /// Implementors of `Functor` must satisfy two laws:
    /// 1. **Identity**: `map(x, |a| a) == x` (more precisely, `<Type<A> as Functor<A>>::map(x, |a| a) == x`)
    /// 2. **Composition**: `<Type<B> as Functor<B>>::map(<Type<A> as Functor<A>>::map(x, f), g) == <Type<A> as Functor<A>>::map(x, |a| g(f(a)))`
    ///
    /// The associated type `Functor<T>` represents the generic structure of the functor
    /// itself, allowing `map` to transform the inner type while preserving the structure.
    pub trait Functor<A> {
        /// The associated type representing the structure of the Functor.
        type Functor<T>;

        /// Applies a function to a value within a context.
        fn map<B, Func>(self, f: Func) -> <Self as Functor<A>>::Functor<B>
        where
            Func: Fn(A) -> B + Clone + 'static;
    }

    impl<A: 'static> Functor<A> for Option<A> {
        type Functor<T> = Option<T>;

        fn map<B, Func>(self, f: Func) -> <Self as Functor<A>>::Functor<B>
        where
            Func: FnMut(A) -> B + 'static,
        {
            self.map(f)
        }
    }

    impl<A: 'static, E: 'static> Functor<A> for Result<A, E> {
        type Functor<T> = Result<T, E>;

        fn map<B, Func>(self, f: Func) -> <Self as Functor<A>>::Functor<B>
        where
            Func: FnMut(A) -> B + 'static,
        {
            self.map(f)
        }
    }

    impl<A: 'static> Functor<A> for Vec<A> {
        type Functor<T> = Vec<T>;

        fn map<B, Func>(self, f: Func) -> <Self as Functor<A>>::Functor<B>
        where
            Func: FnMut(A) -> B + 'static,
        {
            self.into_iter().map(f).collect()
        }
    }

    impl<X: 'static, A: 'static> Functor<A> for CFn<X, A> {
        type Functor<T> = CFnOnce<X, T>;

        fn map<B, Func>(self, mut f: Func) -> Self::Functor<B>
        where
            Func: FnMut(A) -> B + 'static,
        {
            CFnOnce::new(move |x: X| f(self.call(x)))
        }
    }

    impl<X: 'static, A: 'static> Functor<A> for CFnOnce<X, A> {
        type Functor<T> = CFnOnce<X, T>;

        fn map<B, Func>(self, mut f: Func) -> Self::Functor<B>
        where
            Func: FnMut(A) -> B + 'static,
        {
            CFnOnce::new(move |x: X| f(self.call_once(x)))
        }
    } // Closing for impl Functor for CFnOnce
}

#[cfg(feature = "kind")]
pub mod hkt {
    //! # Higher-Kinded Type (HKT) Functor
    //!
    //! This module defines the `Functor` trait for types that implement the HKT pattern.
    //! A Functor is a type constructor `F` that supports a `map` operation, allowing
    //! a function `A -> B` to be applied to a value(s) within a structure `F<A>`,
    //! resulting in a structure `F<B>`.
    //!
    //! The HKT `Functor` trait is generic over:
    //! - `Self`: The HKT marker (e.g., [`OptionHKTMarker`]).
    //! - `A`: The input type of the value(s) within the HKT structure.
    //! - `B`: The output type after applying the mapping function.
    //!
    //! It relies on the [`HKT1`] trait from `crate::kind_based::kind` to relate the
    //! marker `Self` to its concrete type application `Self::Applied<T>`.

    use crate::kind_based::kind::{HKT1, OptionHKTMarker, ResultHKTMarker, VecHKTMarker, CFnHKTMarker, CFnOnceHKTMarker};
    use crate::function::{CFn, CFnOnce};

    /// Represents a type constructor that can be mapped over, using the HKT pattern.
    ///
    /// `Self` refers to the HKT marker type (e.g., [`OptionHKTMarker`], [`VecHKTMarker`])
    /// that implements [`HKT1`].
    ///
    /// The `map` operation takes an instance of `Self::Applied<A>` (e.g., `Option<A>`)
    /// and a function `A -> B`, producing `Self::Applied<B>` (e.g., `Option<B>`).
    ///
    /// ## Functor Laws
    /// Implementors must satisfy two laws:
    /// 1.  **Identity**: For any HKT marker `F` and value `x: F::Applied<A>`,
    ///     `F::map(x, |a| a) == x`.
    /// 2.  **Composition**: For any HKT marker `F`, value `x: F::Applied<A>`, and functions
    ///     `f: A -> B`, `g: B -> C`,
    ///     `F::map(F::map(x, f), g) == F::map(x, |a| g(f(a)))`.
    pub trait Functor<A, B>: HKT1 { // A is input type, B is output type for map
        /// Applies a function to a value (or values) within an HKT-encoded structure.
        ///
        /// # Type Parameters
        /// - `Self`: The HKT marker (e.g., [`OptionHKTMarker`]).
        /// - `A`: The type of the value(s) inside the input structure `Self::Applied<A>`.
        /// - `B`: The type of the value(s) inside the output structure `Self::Applied<B>`.
        ///
        /// # Parameters
        /// - `input`: The HKT-structured value, e.g., `Option<A>`, `Vec<A>`.
        /// - `func`: A function to apply to the inner value(s).
        ///   The `FnMut(A) -> B + Clone + 'static` bound is chosen for broad compatibility:
        ///   - `FnMut`: Allows mutation of captured state if needed, and covers `Fn` and `FnOnce`
        ///     that don't consume their captures by value on first call. Standard library `map`
        ///     methods (like `Option::map`, `Result::map`) often take `FnOnce`.
        ///   - `Clone`: Necessary for some implementations like [`CFnHKTMarker`], where the
        ///     function might need to be cloned if the resulting structure can be "called"
        ///     multiple times.
        ///   - `'static`: Often required when functions are stored or returned within structures
        ///     like [`CFn`] or [`CFnOnce`], especially if they don't borrow from the local scope.
        ///
        /// # Returns
        /// A new HKT-structured value `Self::Applied<B>` containing the result(s) of
        /// applying `func`.
        fn map(input: Self::Applied<A>, func: impl FnMut(A) -> B + Clone + 'static) -> Self::Applied<B>;
    }

    impl<A, B> Functor<A, B> for OptionHKTMarker {
        fn map(input: Self::Applied<A>, func: impl FnMut(A) -> B + Clone + 'static) -> Self::Applied<B> {
            input.map(func) // Option::map takes FnOnce, FnMut is compatible
        }
    }

    impl<A, B, E> Functor<A, B> for ResultHKTMarker<E> {
        fn map(input: Self::Applied<A>, func: impl FnMut(A) -> B + Clone + 'static) -> Self::Applied<B> {
            input.map(func) // Result::map takes FnOnce, FnMut is compatible
        }
    }

    impl<A, B> Functor<A, B> for VecHKTMarker {
        fn map(input: Self::Applied<A>, func: impl FnMut(A) -> B + Clone + 'static) -> Self::Applied<B> {
            input.into_iter().map(func).collect()
        }
    }

    // CFnHKTMarker and CFnOnceHKTMarker definitions and HKT impls are moved to src/kind_based/kind.rs

    // Functor impl for CFnHKTMarker (maps over the output type of CFn)
    // A is the original output type, B is the new output type
    impl<X, A, B> Functor<A, B> for CFnHKTMarker<X>
    where
        X: 'static, 
        A: 'static,
        B: 'static, // B must be 'static for CFn<X,B> which is Self::Applied<B>
    {
        fn map(input: Self::Applied<A>, func: impl FnMut(A) -> B + Clone + 'static) -> Self::Applied<B>
        {
            // input is CFn<X, A>
            // func is A -> B
            // result is CFn<X, B>
            // To create a new CFn, the captured 'func' needs to be Clone if 'input' is called multiple times.
            // Or, if 'input' is Fn, and 'func' is Fn, the result can be Fn.
            // Let's assume func is Clone for simplicity to create a new CFn.
            CFn::new(move |x: X| func.clone()(input.call(x))) // func.clone() for new CFn
        }
    }

    // Functor impl for CFnOnceHKTMarker (maps over the output type of CFnOnce)
    impl<X, A, B> Functor<A, B> for CFnOnceHKTMarker<X>
    where
        X: 'static, 
        A: 'static,
        B: 'static, // B must be 'static for CFnOnce<X,B> which is Self::Applied<B>
    {
        fn map(input: Self::Applied<A>, mut func: impl FnMut(A) -> B + Clone + 'static) -> Self::Applied<B> // Added mut for func
        {
            CFnOnce::new(move |x: X| func(input.call_once(x)))
        }
    }
}

// Re-export based on feature flag
#[cfg(not(feature = "kind"))]
pub use classic::Functor;

#[cfg(feature = "kind")]
pub use hkt::{Functor}; // CFnHKTMarker and CFnOnceHKTMarker are in kind_based::kind
