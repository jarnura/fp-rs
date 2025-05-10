#[cfg(not(feature = "kind"))]
mod classic {
    use crate::{apply::Apply, function::CFn};

    pub trait Applicative<A>: Apply<A> {
        type Applicative<T>;
        fn pure(v: A) -> Self::Applicative<A>;
    }

    impl<A: 'static> Applicative<A> for Option<A> {
        type Applicative<T> = Option<T>;
        fn pure(v: A) -> Self::Applicative<A> {
            Some(v)
        }
    }

    impl<A: 'static, E: 'static + Clone> Applicative<A> for Result<A, E> {
        type Applicative<T> = Result<T, E>;
        fn pure(v: A) -> Self::Applicative<A> {
            Ok(v)
        }
    }

    impl<A: 'static + Clone> Applicative<A> for Vec<A> {
        type Applicative<T> = Vec<T>;
        fn pure(v: A) -> Self::Applicative<A> {
            vec![v]
        }
    }

    #[allow(clippy::module_name_repetitions)]
    pub fn lift_a1<AppCtx, A, B: 'static, FnHook, AppFnCtx>(
        f: FnHook,
        fa: AppCtx,
    ) -> <AppCtx as Apply<A>>::Apply<B>
    where
        FnHook: Fn(A) -> B + 'static,
        AppFnCtx: Applicative<CFn<A, B>, Applicative<CFn<A, B>> = AppFnCtx> + 'static,
        AppCtx: Apply<A, Functor<<AppCtx as Apply<A>>::Fnn<A, B>> = AppFnCtx>,
    {
        let f_in_context: AppFnCtx = AppFnCtx::pure(CFn::new(f));
        <AppCtx as Apply<A>>::apply(fa, f_in_context)
    }
}

#[cfg(feature = "kind")]
pub mod hkt {
    //! # Higher-Kinded Type (HKT) Applicative Functor
    //!
    //! This module defines the `Applicative` trait for HKTs, which extends `Apply`.
    //! An Applicative Functor allows lifting a normal value into the HKT context
    //! (via `pure`) and applying a wrapped function to a wrapped value (via `apply`
    //! from the `Apply` supertrait).
    //!
    //! The HKT `Applicative` trait is generic over:
    //! - `Self`: The HKT marker (e.g., [`OptionHKTMarker`]).
    //! - `T`: The type of the value being lifted by `pure`.
    //!
    //! ## Example
    //!
    //! ```
    //! use fp_rs::applicative::hkt::{Applicative, lift_a1}; // Applicative for ::pure
    //! use fp_rs::apply::hkt::Apply; // for .apply() method
    //! use fp_rs::kind_based::kind::OptionHKTMarker;
    //! use fp_rs::function::CFn;
    //!
    //! // Using pure and apply directly
    //! let val_opt: Option<i32> = OptionHKTMarker::pure(10); // Some(10)
    //! let fn_opt: Option<CFn<i32, i32>> = OptionHKTMarker::pure(CFn::new(|x| x + 1)); // Some(CFn)
    //!
    //! // Need to specify the marker for apply
    //! let result_opt: Option<i32> = OptionHKTMarker::apply(val_opt, fn_opt);
    //! assert_eq!(result_opt, Some(11));
    //!
    //! // Using lift_a1 (which uses pure and apply internally)
    //! let val_opt2: Option<i32> = Some(20);
    //! let result_opt2: Option<i32> = lift_a1(|x: i32| x * 2, val_opt2);
    //! assert_eq!(result_opt2, Some(40));
    //! ```
    //!
    //! `Applicative` builds upon `Apply` by adding the `pure` method. This allows
    //! functions and values to be lifted into the context before application.
    //! A common pattern, often called `lift_a1` or similar, is equivalent to
    //! `map` from `Functor` but implemented using `pure` and `apply`:
    //! `map f fa == apply(fa, pure(f))`. Note: The order of arguments in `apply` can vary;
    //! this library's `apply` takes `(value_context, function_context)`.
    //! The `lift_a1` function in this module demonstrates this pattern.

    use crate::apply::hkt::Apply; // HKT Apply
    use crate::function::{CFn, CFnOnce};
    use crate::kind_based::kind::{
        HKT, HKT1, OptionHKTMarker, ResultHKTMarker, VecHKTMarker, CFnHKTMarker, CFnOnceHKTMarker
    };

    /// Represents an HKT that is an Applicative Functor.
    ///
    /// `Self` refers to the HKT marker type (e.g., [`OptionHKTMarker`]) that implements
    /// [`HKT1`] and [`Apply`].
    ///
    /// The primary method provided by `Applicative` is `pure`, which takes a regular
    /// value `T` and lifts it into the HKT context, producing `Self::Applied<T>`
    /// (e.g., `pure(10)` for `OptionHKTMarker` yields `Some(10)`).
    ///
    /// ## Example of `pure`
    ///
    /// ```
    /// use fp_rs::applicative::hkt::Applicative;
    /// use fp_rs::kind_based::kind::{OptionHKTMarker, VecHKTMarker};
    ///
    /// // For Option
    /// let val_opt: Option<i32> = OptionHKTMarker::pure(10);
    /// assert_eq!(val_opt, Some(10));
    ///
    /// // For Vec (requires T: Clone for pure)
    /// let val_vec: Vec<String> = VecHKTMarker::pure("hello".to_string());
    /// assert_eq!(val_vec, vec!["hello".to_string()]);
    /// ```
    ///
    /// ## Applicative Laws
    /// Implementors must satisfy several laws:
    /// 1.  **Identity**: `apply(v, pure(identity_fn)) == v`
    /// 2.  **Homomorphism**: `apply(pure(x), pure(f_fn)) == pure(f(x))`
    /// 3.  **Interchange**: `apply(pure(y), u) == apply(u, pure(|f_fn| f_fn(y)))`
    /// 4.  **Composition (derived)**: `map f x == apply(x, pure(f))` (often shown as `lift_a1`)
    ///     (Note: The exact formulation of composition can vary, often involving `apply` and `pure`.)
    pub trait Applicative<T>: Apply<T, T>
    where
        Self: Sized + HKT1,
        T: 'static, 
    {
        /// Lifts a value into the applicative context.
        ///
        /// # Parameters
        /// - `value`: The value of type `T` to be lifted.
        ///   The `T: 'static` bound is common. Many `pure` implementations also require `T: Clone`
        ///   (e.g., for [`CFnHKTMarker`], [`VecHKTMarker`]) if the `value` needs to be cloned
        ///   into the new context, especially if the context itself might be "called" or
        ///   iterated multiple times. This can make some applicative laws involving `pure`
        ///   of non-`Clone` function types (like `CFn`) untestable.
        ///
        /// # Returns
        /// The value wrapped in the HKT applicative structure, `Self::Applied<T>`.
        fn pure(value: T) -> Self::Applied<T>;
    }

    impl<T: 'static> Applicative<T> for OptionHKTMarker {
        /// Lifts a value `T` into [`Some(T)`].
        fn pure(value: T) -> Self::Applied<T> { // Self::Applied<T> is Option<T>
            Some(value)
        }
    }

    impl<T: 'static, E: 'static + Clone> Applicative<T> for ResultHKTMarker<E> {
        /// Lifts a value `T` into [`Ok(T)`].
        fn pure(value: T) -> Self::Applied<T> { // Self::Applied<T> is Result<T, E>
            Ok(value)
        }
    }

    impl<T: 'static + Clone> Applicative<T> for VecHKTMarker {
        /// Lifts a value `T` into `vec![T]`.
        ///
        /// The `T: Clone` bound on this `impl` block is due to `Vec`'s `pure`
        /// creating a new vector with the element.
        fn pure(value: T) -> Self::Applied<T> { // Self::Applied<T> is Vec<T>
            vec![value]
        }
    }

    // Applicative for CFnHKTMarker
    // Lifts a value `T` into `CFn<X, T>` which always returns `value.clone()`
    impl<X, T> Applicative<T> for CFnHKTMarker<X>
    where
        X: 'static,
        T: 'static + Clone, // T needs to be Clone for the closure
        Self: Apply<T,T>, // Ensure Apply<T,T> for CFnHKTMarker<X> is defined
        Self: HKT<Applied<T> = CFn<X, T>>, // Explicitly state the GAT equality
    {
        /// Lifts a value `T` into a `CFn<X, T>` (a function `X -> T`).
        ///
        /// The resulting function, when called with any input of type `X`,
        /// will ignore that input and always return a clone of the original `value`.
        ///
        /// Requires `T: Clone` because the lifted value is cloned by the returned function.
        fn pure(value: T) -> Self::Applied<T> {
            // Self::Applied<T> is CFn<X, T> as per HKT1 impl for CFnHKTMarker
            CFn::new(move |_x: X| value.clone())
        }
    }

    // Applicative for CFnOnceHKTMarker
    // Lifts a value `T` into `CFnOnce<X, T>`
    impl<X, T> Applicative<T> for CFnOnceHKTMarker<X>
    where
        X: 'static,
        T: 'static + Clone,
        Self: Apply<T,T>,
        Self: HKT<Applied<T> = CFnOnce<X, T>>, // Explicitly state the GAT equality
    {
        /// Lifts a value `T` into a `CFnOnce<X, T>` (a function `X -> T` called once).
        ///
        /// The resulting function, when called with any input of type `X`,
        /// will ignore that input and return a clone of the original `value`.
        ///
        /// Requires `T: Clone` as the lifted value is cloned by the returned function.
        fn pure(value: T) -> Self::Applied<T> {
            // Self::Applied<T> is CFnOnce<X, T> as per HKT1 impl for CFnOnceHKTMarker
            CFnOnce::new(move |_x: X| value.clone())
        }
    }

    /// Lifts a unary function `A -> B` to operate on HKT `Applicative` values: `F<A> -> F<B>`.
    /// This is `map` defined via `pure` and `apply`: `map f fa == apply(fa, pure(CFn::new(f)))`.
    ///
    /// # Parameters
    /// - `F`: The HKT marker, must implement `Applicative<CFn<A,B>>` and `Apply<A,B>`.
    /// - `func`: The function `A -> B`.
    /// - `fa`: The applicative value `F::Applied<A>`.
    ///
    /// # Returns
    /// The result `F::Applied<B>`.
    ///
    /// ## Example
    ///
    /// ```
    /// use fp_rs::applicative::hkt::lift_a1;
    /// use fp_rs::kind_based::kind::{OptionHKTMarker, VecHKTMarker}; // For context type
    ///
    /// // Using lift_a1 with Option
    /// let opt_val: Option<i32> = Some(5);
    /// // Provide the HKT marker via turbofish if type inference needs help
    /// let lifted_opt: Option<String> = lift_a1::<OptionHKTMarker, _, _, _>(
    ///     |x: i32| (x * 2).to_string(),
    ///     opt_val
    /// );
    /// assert_eq!(lifted_opt, Some("10".to_string()));
    ///
    /// // Using lift_a1 with Vec
    /// let vec_val: Vec<i32> = vec![1, 2, 3];
    /// let lifted_vec: Vec<bool> = lift_a1::<VecHKTMarker, _, _, _>(
    ///     |x: i32| x % 2 == 0,
    ///     vec_val
    /// );
    /// assert_eq!(lifted_vec, vec![false, true, false]);
    /// ```
    pub fn lift_a1<F, A, B, FuncImpl>(
        func: FuncImpl,
        fa: F::Applied<A>,
    ) -> F::Applied<B>
    where
        F: Applicative<CFn<A, B>> + Apply<A, B> + HKT1,
        FuncImpl: Fn(A) -> B + 'static,
        A: 'static,
        B: 'static,
        CFn<A, B>: 'static, // Ensure the lifted function type is 'static
    {
        // 1. Lift the function `func: A -> B` into the context using `CFn`.
        //    `F::pure(CFn::new(func))` results in `F::Applied<CFn<A, B>>`.
        //    This requires `F` to be `Applicative` for the type `CFn<A, B>`.
        let f_in_context: F::Applied<CFn<A, B>> = F::pure(CFn::new(func));

        // 2. Apply the wrapped function to the wrapped value.
        //    `F::apply(fa, f_in_context)` where `fa` is `F::Applied<A>`.
        //    This requires `F` to be `Apply<A, B>`.
        F::apply(fa, f_in_context)
    }
}

// Re-export based on feature flag
#[cfg(not(feature = "kind"))]
pub use classic::*;

#[cfg(feature = "kind")]
pub use hkt::*;
