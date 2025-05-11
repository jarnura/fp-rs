pub mod hkt {
    //! # Higher-Kinded Type (HKT) Apply
    //!
    //! This module defines the `Apply` trait for HKTs, which extends `Functor`.
    //! `Apply` provides the `apply` method (often denoted as `<*>`), allowing sequential
    //! application of a wrapped function to a wrapped value.
    //!
    //! If you have `F<A>` (a wrapped value) and `F<A -> B>` (a wrapped function),
    //! `apply` combines them to produce `F<B>`.
    //!
    //! The HKT `Apply` trait is generic over:
    //! - `Self`: The HKT marker (e.g., [`OptionHKTMarker`]).
    //! - `A`: The input type of the function `A -> B` and the type of value in `Self::Applied<A>`.
    //! - `B`: The output type of the function `A -> B` and the type of value in `Self::Applied<B>`.

    use crate::function::{CFn, CFnOnce};
    use crate::functor::{Functor}; // HKT Functor
    use crate::kind_based::kind::{
        HKT, HKT1, OptionHKTMarker, VecHKTMarker, ResultHKTMarker, CFnHKTMarker, CFnOnceHKTMarker
    };

    /// Represents an HKT that can apply a wrapped function to a wrapped value.
    ///
    /// `Self` refers to the HKT marker type (e.g., [`OptionHKTMarker`]) that implements
    /// [`HKT1`] and [`Functor`].
    ///
    /// The `apply` method takes `Self::Applied<A>` (e.g., `Option<A>`) and
    /// `Self::Applied<CFn<A, B>>` (e.g., `Option<CFn<A, B>>`), and produces
    /// `Self::Applied<B>` (e.g., `Option<B>`).
    ///
    /// ## Apply Laws
    /// (Often defined in terms of `Applicative` which builds on `Apply`)
    /// A key law related to `apply` is compositional:
    /// `apply(apply(v, compose_pure_fn), pure_g_fn) == apply(v, apply(pure_f_fn, pure_compose_fn))`
    /// where `compose_pure_fn` is `pure(.)` or `pure(|f| |g| |x| f(g(x)))`.
    /// More commonly, laws are expressed with `Applicative`.
    pub trait Apply<A, B>: Functor<A, B>
    where
        Self: Sized + HKT1,
        A: 'static,
        B: 'static,
    {
        /// Applies a function wrapped in an HKT structure to a value wrapped in the same HKT structure.
        ///
        /// # Type Parameters
        /// - `Self`: The HKT marker.
        /// - `A`: The input type for the wrapped function `CFn<A, B>`.
        /// - `B`: The result type of the wrapped function and the output HKT structure.
        ///
        /// # Parameters
        /// - `value_container`: The HKT-structured value `Self::Applied<A>`.
        /// - `function_container`: The HKT-structured function `Self::Applied<CFn<A, B>>`.
        ///   Note: The function itself is wrapped in [`CFn`], which handles dynamic dispatch
        ///   and necessary `'static` bounds for the function it wraps.
        ///
        /// # Returns
        /// A new HKT-structured value `Self::Applied<B>`.
        fn apply(
            value_container: Self::Applied<A>,
            function_container: Self::Applied<CFn<A, B>>,
        ) -> Self::Applied<B>;
    }

    impl<A: 'static, B: 'static> Apply<A, B> for OptionHKTMarker {
        fn apply(
            value_container: Self::Applied<A>,
            function_container: Self::Applied<CFn<A, B>>,
        ) -> Self::Applied<B> {
            value_container.and_then(|val_a| function_container.map(|func_ab| func_ab.call(val_a)))
        }
    }

    impl<A: 'static, B: 'static, E: 'static + Clone> Apply<A, B> for ResultHKTMarker<E> {
        fn apply(
            value_container: Self::Applied<A>,
            function_container: Self::Applied<CFn<A, B>>,
        ) -> Self::Applied<B> {
            value_container.and_then(|val_a| function_container.map(|func_ab| func_ab.call(val_a)))
        }
    }

    impl<A: 'static + Clone, B: 'static> Apply<A, B> for VecHKTMarker {
        fn apply(
            value_container: Self::Applied<A>,
            function_container: Self::Applied<CFn<A, B>>,
        ) -> Self::Applied<B> {
            function_container
                .into_iter()
                .flat_map(|f_fn| {
                    value_container
                        .iter()
                        .map(move |val_a| f_fn.call(val_a.clone()))
                })
                .collect()
        }
    }
    
    // Apply for CFnHKTMarker<X>
    // F::Applied<A> is CFn<X, A>
    // F::Applied<CFn<A, B>> is CFn<X, CFn<A, B>>
    // Result is CFn<X, B>
    // This implements S f g x = (f x) (g x)
    impl<X, A, B> Apply<A, B> for CFnHKTMarker<X>
    where
        X: 'static + Clone, // Clone for x_val in the closure
        A: 'static,
        B: 'static,
        Self: Functor<A, B>, // Ensure Functor constraint is met
        Self: HKT<Applied<A> = CFn<X, A>>,
        Self: HKT<Applied<CFn<A, B>> = CFn<X, CFn<A, B>>>,
        Self: HKT<Applied<B> = CFn<X, B>>,
        // Removed: CFn<X, CFn<A, B>>: Fn(X) -> CFn<A, B>,
        // Removed: CFn<X, A>: Fn(X) -> A,
        // The .call method on CFn struct does not require CFn itself to be Fn.
        // A: 'static is from Apply trait. X: 'static + Clone for closure. B: 'static from Apply trait.
    {
        fn apply(
            value_container: Self::Applied<A>, // This is c_x_a
            function_container: Self::Applied<CFn<A, B>>, // This is c_x_fab
        ) -> Self::Applied<B> {
            // Self::Applied<A> is CFn<X, A>
            // Self::Applied<CFn<A, B>> is CFn<X, CFn<A, B>>
            CFn::new(move |x_val: X| {
                let func_ab = function_container.call(x_val.clone()); 
                let val_a = value_container.call(x_val);
                func_ab.call(val_a)
            })
        }
    }

    // Apply for CFnOnceHKTMarker<X>
    // Similar to CFnHKTMarker, but uses call_once and produces CFnOnce
    impl<X, A, B> Apply<A, B> for CFnOnceHKTMarker<X>
    where
        X: 'static + Clone, // Clone for x_val in the closure
        A: 'static,
        B: 'static,
        Self: Functor<A, B>,
        Self: HKT<Applied<A> = CFnOnce<X, A>>,
        Self: HKT<Applied<CFn<A, B>> = CFnOnce<X, CFn<A, B>>>, // Assuming the function container holds CFn<A,B>
        Self: HKT<Applied<B> = CFnOnce<X, B>>,
        // Removed FnOnce bounds on GATs. The .call_once method is inherent.
        // <Self as HKT>::Applied<CFn<A, B>>: FnOnce(X) -> CFn<A, B>,
        // <Self as HKT>::Applied<A>: FnOnce(X) -> A,
        // A: 'static from Apply trait. X: 'static + Clone for closure. B: 'static from Apply trait.
        // For now, let's assume CFn<A,B> can be called via call_once if it's the last use.
        // The compiler error E0599 for call_once on CFnOnceHKTMarker::Applied<CFn<A,B>>
        // indicates that the GAT itself needs to satisfy FnOnce(X) -> CFn<A,B>.
        // And for CFnOnceHKTMarker::Applied<A>, it needs FnOnce(X) -> A.
        // The inner func_ab.call_once(val_a) implies CFn<A,B>: FnOnce(A) -> B.
        // CFn already implements Fn, FnMut, and FnOnce if the inner Box<dyn Fn...> does.
        // The issue is that the GAT Self::Applied<CFn<A,B>> is opaque.
    {
        fn apply(
            value_container: Self::Applied<A>, // CFnOnce<X,A>
            function_container: Self::Applied<CFn<A, B>>, // CFnOnce<X, CFn<A,B>>
        ) -> Self::Applied<B> { // CFnOnce<X,B>
            CFnOnce::new(move |x_val: X| {
                // Self::Applied<CFn<A,B>> is CFnOnce<X, CFn<A,B>>
                // Self::Applied<A> is CFnOnce<X,A>
                let func_ab = function_container.call_once(x_val.clone()); // func_ab is CFn<A,B>
                let val_a = value_container.call_once(x_val);             // val_a is A
                func_ab.call(val_a) // Changed to .call(val_a) from .call_once((val_a,))
            })
        }
    }


    /// Lifts a binary curried function to operate on HKT contexts.
    ///
    /// Given `func: A -> (B -> C)` (represented as `A -> CFn<B, C>`),
    /// `fa: F<A>`, and `fb: F<B>`, `lift2` produces `F<C>`.
    /// It's equivalent to `apply(map(fa, func), fb)` but expressed using the `Apply` trait's `apply` method.
    pub fn lift2<F, A, B, C, FuncImpl>(
        func: FuncImpl, // A -> CFn<B, C>
        fa: F::Applied<A>,
        fb: F::Applied<B>,
    ) -> F::Applied<C>
    where
        F: Apply<B, C> + Functor<A, CFn<B, C>> + HKT1, 
        FuncImpl: Fn(A) -> CFn<B, C> + Clone + 'static, 
        A: 'static, B: 'static, C: 'static,
    {
        let f_b_to_c_in_f = F::map(fa, func); 
        F::apply(fb, f_b_to_c_in_f)           
    }

    /// Lifts a ternary curried function to operate on HKT contexts.
    ///
    /// Given `func: A -> (B -> (C -> D))` (represented as `A -> CFn<B, CFn<C, D>>`),
    /// `fa: F<A>`, `fb: F<B>`, and `fc: F<C>`, `lift3` produces `F<D>`.
    pub fn lift3<F, A, B, C, D, FuncImpl>(
        func: FuncImpl, // A -> CFn<B, CFn<C, D>>
        fa: F::Applied<A>,
        fb: F::Applied<B>,
        fc: F::Applied<C>,
    ) -> F::Applied<D>
    where
        F: Apply<C, D>             
           + Apply<B, CFn<C,D>>    
           + Functor<A, CFn<B, CFn<C,D>>>
           + HKT1,
        FuncImpl: Fn(A) -> CFn<B, CFn<C, D>> + Clone + 'static,
        A: 'static, B: 'static, C: 'static, D: 'static,
    {
        let f_b_to_c_to_d_in_f = F::map(fa, func);
        let f_c_to_d_in_f = <F as Apply<B, CFn<C,D>>>::apply(fb, f_b_to_c_to_d_in_f);
        <F as Apply<C,D>>::apply(fc, f_c_to_d_in_f)
    }

    /// Combines two HKT actions, keeping only the result of the first.
    /// Often denoted as `<*`.
    pub fn apply_first<F, A, B>(
        fa: F::Applied<A>,
        fb: F::Applied<B>,
    ) -> F::Applied<A>
    where
        F: Apply<B, A> + Functor<A, CFn<B, A>> + HKT1,
        A: Copy + 'static, 
        B: 'static,        
    {
        let map_fn = |x: A| CFn::new(move |_y: B| x); 
        lift2::<F, A, B, A, _>(map_fn, fa, fb)
    }

    /// Combines two HKT actions, keeping only the result of the second.
    /// Often denoted as `*>`.
    pub fn apply_second<F, A, B>(
        fa: F::Applied<A>,
        fb: F::Applied<B>,
    ) -> F::Applied<B>
    where
        F: Apply<B, B> + Functor<A, CFn<B, B>> + HKT1,
        A: 'static, 
        B: Copy + 'static, 
    {
        let map_fn = |_: A| CFn::new(|y: B| y); 
        lift2::<F, A, B, B, _>(map_fn, fa, fb)
    }
}

// Directly export HKT Apply and related functions
pub use hkt::*;
