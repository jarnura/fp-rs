pub mod hkt {
    //! # Higher-Kinded Type (HKT) Monad and Bind
    //!
    //! This module defines the `Monad` and `Bind` traits for HKTs.
    //!
    //! - [`Bind`]: Provides the `bind` method (often called `flatMap` or `>>=`),
    //!   which allows sequencing operations that return a monadic value. It extends [`Apply`].
    //! - [`Monad`]: Extends [`Applicative`] (and thus `Bind` via `Applicative`'s supertrait `Apply`)
    //!   and adds the `join` method, which flattens a nested monadic structure (e.g., `F<F<A>>` to `F<A>`).
    //!   Alternatively, a monad can be defined by `pure` (from `Applicative`) and `bind`.
    //!
    //! ## Example
    //!
    //! ```
    //! use fp_rs::monad::hkt::{Monad, Bind};
    //! use fp_rs::applicative::hkt::Applicative; // For pure
    //! use fp_rs::kind_based::kind::OptionHKTMarker;
    //!
    //! // Using bind
    //! let opt_val: Option<i32> = Some(5);
    //! let bind_fn = |x: i32| if x > 0 { Some(x * 2) } else { None };
    //! let result_bind: Option<i32> = OptionHKTMarker::bind(opt_val, bind_fn);
    //! assert_eq!(result_bind, Some(10));
    //!
    //! let opt_val_none: Option<i32> = Some(-5);
    //! let result_bind_none: Option<i32> = OptionHKTMarker::bind(opt_val_none, bind_fn);
    //! assert_eq!(result_bind_none, None);
    //!
    //! // Using join
    //! let nested_opt: Option<Option<String>> = Some(Some("hello".to_string()));
    //! let joined_opt: Option<String> = OptionHKTMarker::join(nested_opt);
    //! assert_eq!(joined_opt, Some("hello".to_string()));
    //!
    //! let nested_none_inner: Option<Option<String>> = Some(None);
    //! let joined_none_inner: Option<String> = OptionHKTMarker::join(nested_none_inner);
    //! assert_eq!(joined_none_inner, None);
    //!
    //! let nested_none_outer: Option<Option<String>> = None;
    //! let joined_none_outer: Option<String> = OptionHKTMarker::join(nested_none_outer);
    //! assert_eq!(joined_none_outer, None);
    //! ```

    use crate::applicative::hkt::Applicative; // HKT Applicative
    use crate::apply::hkt::Apply;             // HKT Apply
    use crate::function::{CFn, CFnOnce};
    use crate::kind_based::kind::{
        HKT, HKT1, OptionHKTMarker, ResultHKTMarker, VecHKTMarker, CFnHKTMarker, CFnOnceHKTMarker
    };

    /// HKT-based `Monad` trait.
    ///
    /// A `Monad` allows for sequencing computations within a context. It extends
    /// [`Applicative`]. The key additional operation is `join`, which flattens
    /// nested monadic structures.
    ///
    /// `Self` refers to the HKT marker type (e.g., [`OptionHKTMarker`]) that implements
    /// [`HKT1`] and [`Applicative`].
    /// `A` is the type of the value held within the monadic context (e.g., the `T` in `Option<T>`).
    ///
    /// ## Monad Laws
    /// Implementors must satisfy several laws:
    /// 1.  **Left Identity**: `bind(pure(x), f) == f(x)`
    /// 2.  **Right Identity**: `bind(m, pure) == m`
    /// 3.  **Associativity**: `bind(bind(m, f), g) == bind(m, |x| bind(f(x), g))`
    ///
    /// These laws can also be expressed using `join`, `pure`, and `map` (from `Functor`,
    /// which `Applicative` extends):
    /// 1.  `join(pure(m)) == m`
    /// 2.  `join(map(m, pure)) == m`
    /// 3.  `join(map(mma, join)) == join(join(mma))` (if `map` is defined for `F<F<F<A>>>`)
    ///     or more commonly: `join(map(mmma, |mma| join(mma))) == join(join(mmma))`
    pub trait Monad<A>: Applicative<A> // Monad holds type A
    where
        Self: Sized + HKT1, // Self is the HKT Marker
        A: 'static,
    {
        /// Flattens a nested monadic structure.
        ///
        /// For an HKT `F`, `join` takes `F<F<A>>` and returns `F<A>`.
        ///
        /// # Example
        /// ```
        /// use fp_rs::monad::hkt::Monad;
        /// use fp_rs::kind_based::kind::OptionHKTMarker;
        ///
        /// let nested: Option<Option<i32>> = Some(Some(10));
        /// let flat: Option<i32> = OptionHKTMarker::join(nested);
        /// assert_eq!(flat, Some(10));
        ///
        /// let nested_none: Option<Option<i32>> = Some(None);
        /// assert_eq!(OptionHKTMarker::join(nested_none), None);
        /// ```
        fn join(mma: Self::Applied<Self::Applied<A>>) -> Self::Applied<A>;
    }

    /// HKT-based `Bind` trait (equivalent to `flatMap` or Haskell's `>>=`).
    ///
    /// `Bind` allows sequencing operations where each operation takes a normal value
    /// and returns a value wrapped in the HKT context. It extends [`Apply`].
    ///
    /// `Self` refers to the HKT marker type (e.g., [`OptionHKTMarker`]).
    /// `A` is the type of the value within the input HKT context `Self::Applied<A>`.
    /// `B` is the type of the value within the output HKT context `Self::Applied<B>`
    /// that the provided function `func` returns.
    ///
    /// ## Example
    /// ```
    /// use fp_rs::monad::hkt::Bind;
    /// use fp_rs::kind_based::kind::OptionHKTMarker;
    ///
    /// let opt_val: Option<i32> = Some(5);
    ///
    /// // Function that returns an Option
    /// let half = |x: i32| -> Option<f64> {
    ///     if x % 2 == 0 { Some((x as f64) / 2.0) } else { None }
    /// };
    ///
    /// let result: Option<f64> = OptionHKTMarker::bind(opt_val, half); // Fails as 5 is odd
    /// assert_eq!(result, None);
    ///
    /// let result_even: Option<f64> = OptionHKTMarker::bind(Some(10), half); // Succeeds
    /// assert_eq!(result_even, Some(5.0));
    /// ```
    pub trait Bind<A, B>: Apply<A, B>
    where
        Self: Sized + HKT1,
        A: 'static, 
        B: 'static, 
        // Self::Applied<B>: 'static, // This was for the default impl, may not be needed at trait level.
                                   // Apply<A,B> already requires B: 'static.
    {
        /// Sequentially composes computations within the HKT context.
        ///
        /// Takes a value in context (`Self::Applied<A>`) and a function (`A -> Self::Applied<B>`).
        /// It applies the function to the unwrapped value (if present/valid) and returns
        /// the resulting context `Self::Applied<B>`.
        fn bind(input: Self::Applied<A>, func: impl FnMut(A) -> Self::Applied<B> + Clone + 'static) -> Self::Applied<B>;
    }

    // --- Bind Implementations ---

    impl<A: 'static, B: 'static> Bind<A, B> for OptionHKTMarker {
        /// For `Option`, `bind` is equivalent to `Option::and_then`.
        /// If `input` is `Some(a)`, it applies `func` to `a`.
        /// If `input` is `None`, it returns `None`.
        fn bind(input: Self::Applied<A>, func: impl FnMut(A) -> Self::Applied<B> + Clone + 'static) -> Self::Applied<B> {
            input.and_then(func)
        }
    }

    impl<A: 'static, B: 'static, E: 'static + Clone> Bind<A, B> for ResultHKTMarker<E> {
        /// For `Result`, `bind` is equivalent to `Result::and_then`.
        /// If `input` is `Ok(a)`, it applies `func` to `a`.
        /// If `input` is `Err(e)`, it propagates the `Err(e)`.
        fn bind(input: Self::Applied<A>, func: impl FnMut(A) -> Self::Applied<B> + Clone + 'static) -> Self::Applied<B> {
            input.and_then(func)
        }
    }

    impl<A: 'static + Clone, B: 'static> Bind<A, B> for VecHKTMarker {
        /// For `Vec`, `bind` applies `func` to each element and flattens the results.
        /// This is equivalent to `Vec::into_iter().flat_map(func).collect()`.
        fn bind(input: Self::Applied<A>, func: impl FnMut(A) -> Self::Applied<B> + Clone + 'static) -> Self::Applied<B> {
            input.into_iter().flat_map(func).collect()
        }
    }

    // Bind for CFnHKTMarker<R> (Kleisli composition for R -> _)
    // input: Self::Applied<A> which is CFn<R, A>
    // func: A -> Self::Applied<B> which is A -> CFn<R, B> (a function producing a function)
    // result: Self::Applied<B> which is CFn<R, B> (a new function R -> B)
    impl<R, A, B: 'static> Bind<A, B> for CFnHKTMarker<R>
    where
        R: 'static + Clone, // Clone for `r.clone()`
        A: 'static,
        // Self: Monad<B> + Functor<A, Self::Applied<B>>, // Removed these from impl where clause
        // Self::Applied<B>: 'static,                   // as Bind trait no longer requires them as supertraits directly.
                                                      // Bind now only requires Apply<A,B>.
        // Original specific requirements for CFnHKTMarker's direct bind:
        Self: Apply<A,B>, // Ensure Apply is implemented (This is now the supertrait of Bind)
        Self: HKT<Applied<A> = CFn<R, A>>,
        Self: HKT<Applied<B> = CFn<R, B>>,
    {
        /// Implements Kleisli composition for functions `R -> A` and `A -> (R -> B)`.
        ///
        /// Given `input_fn: R -> A` and `func: A -> (R -> B)`,
        /// produces a new function `R -> B`.
        /// The new function, when called with `r: R`:
        /// 1. Calls `input_fn(r)` to get `a: A`.
        /// 2. Calls `func(a)` to get `output_fn: R -> B`.
        /// 3. Calls `output_fn(r)` to get `b: B`.
        fn bind(input: Self::Applied<A>, func: impl FnMut(A) -> Self::Applied<B> + Clone + 'static) -> Self::Applied<B> {
            let concrete_input_fn = input;

            CFn::new(move |r: R| {
                let a_val = concrete_input_fn.call(r.clone());
                let mut func_clone = func.clone(); // Clone for this call, as CFn::new needs Fn
                let cfn_r_b = func_clone(a_val); // cfn_r_b is CFn<R, B>
                cfn_r_b.call(r.clone())
            })
        }
    }

    impl<R, A, B: 'static> Bind<A, B> for CFnOnceHKTMarker<R>
    where
        R: 'static + Clone,
        A: 'static,
        // Self: Monad<B> + Functor<A, Self::Applied<B>>, // Removed
        // Self::Applied<B>: 'static,                   // Removed
        // Original specific requirements
        Self: Apply<A,B>, // This is now the supertrait of Bind
        Self: HKT<Applied<A> = CFnOnce<R, A>>,
        Self: HKT<Applied<B> = CFnOnce<R, B>>,
    {
        /// Implements Kleisli composition for functions `R -> A` (once) and `A -> (R -> B)` (once).
        ///
        /// Similar to `CFnHKTMarker::bind`, but for `CFnOnce`.
        /// The resulting function `R -> B` can also only be called once.
        fn bind(input: Self::Applied<A>, mut func: impl FnMut(A) -> Self::Applied<B> + Clone + 'static) -> Self::Applied<B> {
            let concrete_input = input; // CFnOnce<R,A>
            CFnOnce::new(move |r: R| {
                let a_val = concrete_input.call_once(r.clone());
                let cfn_once_r_b = func(a_val); // CFnOnce<R,B>
                cfn_once_r_b.call_once(r)
            })
        }
    }

    // --- Monad Implementations ---

    impl<A: 'static> Monad<A> for OptionHKTMarker {
        /// Flattens `Option<Option<A>>` to `Option<A>`.
        /// `Some(Some(a))` becomes `Some(a)`.
        /// `Some(None)` becomes `None`.
        /// `None` becomes `None`.
        fn join(mma: Self::Applied<Self::Applied<A>>) -> Self::Applied<A> { // mma is Option<Option<A>>
            mma.and_then(core::convert::identity)
        }
    }

    impl<A: 'static, E: 'static + Clone> Monad<A> for ResultHKTMarker<E> {
        /// Flattens `Result<Result<A, E>, E>` to `Result<A, E>`.
        /// `Ok(Ok(a))` becomes `Ok(a)`.
        /// `Ok(Err(e))` becomes `Err(e)`.
        /// `Err(e)` becomes `Err(e)`.
        fn join(mma: Self::Applied<Self::Applied<A>>) -> Self::Applied<A> { // mma is Result<Result<A,E>, E>
            mma.and_then(core::convert::identity)
        }
    }

    impl<A: 'static + Clone> Monad<A> for VecHKTMarker {
        /// Flattens `Vec<Vec<A>>` to `Vec<A>`.
        /// `vec![vec![1, 2], vec![3]]` becomes `vec![1, 2, 3]`.
        fn join(mma: Self::Applied<Self::Applied<A>>) -> Self::Applied<A> { // mma is Vec<Vec<A>>
            mma.into_iter().flatten().collect()
        }
    }

    impl<R, A> Monad<A> for CFnHKTMarker<R>
    where
        R: 'static + Clone,
        A: 'static + Clone, // From Applicative supertrait for CFnHKTMarker<R>
    {
        /// Flattens `CFn<R, CFn<R, A>>` to `CFn<R, A>`.
        ///
        /// This is achieved by using `bind` with the identity function `|ma: CFn<R,A>| ma`.
        /// Given `mma: R -> (R -> A)`, produces `R -> A`.
        /// The new function, when called with `r: R`:
        /// 1. Calls `mma(r)` to get `ma: R -> A`.
        /// 2. Calls `ma(r)` to get `a: A`.
        fn join(mma: Self::Applied<Self::Applied<A>>) -> Self::Applied<A> { // mma is CFn<R, CFn<R,A>>
            // Bind<Self::Applied<A>, A> means Bind<CFn<R,A>, A>
            <Self as Bind<Self::Applied<A>, A>>::bind(mma, |ma: Self::Applied<A>| ma)
        }
    }

    impl<R, A> Monad<A> for CFnOnceHKTMarker<R>
    where
        R: 'static + Clone,
        A: 'static + Clone, // From Applicative supertrait for CFnOnceHKTMarker<R>
    {
        /// Flattens `CFnOnce<R, CFnOnce<R, A>>` to `CFnOnce<R, A>`.
        ///
        /// Similar to `CFnHKTMarker::join`, but for `CFnOnce`.
        fn join(mma: Self::Applied<Self::Applied<A>>) -> Self::Applied<A> { // mma is CFnOnce<R, CFnOnce<R,A>>
            <Self as Bind<Self::Applied<A>, A>>::bind(mma, |ma: Self::Applied<A>| ma)
        }
    }

    /// Helper function for `Bind::bind`.
    ///
    /// This allows calling `bind(func, ma)` instead of `F::bind(ma, func)`.
    ///
    /// # Example
    /// ```
    /// use fp_rs::monad::hkt::bind; // The helper function
    /// use fp_rs::kind_based::kind::OptionHKTMarker;
    ///
    /// let opt_val: Option<i32> = Some(5);
    /// let half = |x: i32| if x % 2 == 0 { Some((x as f64) / 2.0) } else { None };
    ///
    /// // Note: Type of F (OptionHKTMarker) might need to be inferred or specified
    /// let result: Option<f64> = bind::<OptionHKTMarker, _, _, _>(half, opt_val);
    /// assert_eq!(result, None);
    /// ```
    pub fn bind<F, A, B, FuncImpl>(
        func: FuncImpl,
        ma: F::Applied<A>,
    ) -> F::Applied<B>
    where
        F: Bind<A, B> + HKT1, // F is the HKTMarker
        FuncImpl: FnMut(A) -> F::Applied<B> + Clone + 'static, // Added Clone + 'static
        A: 'static,
        B: 'static, // B needs to be 'static for F::Applied<B>
    {
        F::bind(ma, func)
    }

    // pub fn join<F, A>(mma: F::Applied<F::Applied<A>>) -> F::Applied<A>
    // where
    //     F: HKT1 + Monad<A> + Bind<F::Applied<A>, A>, // F must be able to bind F<A> to A. Here A is the B in Bind<_,B>
    //     A: 'static, // This A is the result type of the inner F::Applied<A>
    //     F::Applied<A>: 'static, // The inner M<A> must be 'static for the closure
    // {
    //     // The function for bind is `id: F::Applied<A> -> F::Applied<A>`
    //     // F::bind(mma, |ma: F::Applied<A>| ma)
    //     F::join(mma) // Call the trait method
    // }
}

// Directly export HKT Bind, Monad, and helper bind
pub use hkt::{Bind, Monad, bind};
// Note: join is a method on the Monad trait in the hkt module.
