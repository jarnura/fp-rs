use std::ops::Deref;


/// Type alias for a boxed, dynamically dispatched, repeatable closure.
/// `BFn<A, B>` is equivalent to `Box<dyn Fn(A) -> B + 'static>`.
/// This represents a heap-allocated closure that can be called multiple times.
type BFn<A, B> = Box<dyn Fn(A) -> B + 'static>;

/// Type alias for a boxed, dynamically dispatched, once-callable closure.
/// `BFnOnce<A, B>` is equivalent to `Box<dyn FnOnce(A) -> B + 'static>`.
/// This represents a heap-allocated closure that can be called at most once.
type BFnOnce<A, B> = Box<dyn FnOnce(A) -> B + 'static>;

/// A wrapper around `BFn<A, B>` (a `Box<dyn Fn(A) -> B + 'static>`).
///
/// This struct provides a concrete type for heap-allocated, repeatable closures,
/// which is useful for storing them in structs or passing them as arguments
/// where a concrete type is needed (e.g., in trait implementations like `Functor` for functions).
///
/// `CFn` stands for "Clonable Function" or "Composable Function", though it's not inherently `Clone`
/// unless the underlying boxed closure captures only `Clone` data (which `Box<dyn Fn>` doesn't guarantee).
/// The primary purpose here is to provide a newtype wrapper.
///
/// # Examples
/// ```
/// use fp_rs::function::CFn;
///
/// let add_one = CFn::new(|x: i32| x + 1);
/// assert_eq!(add_one.call(5), 6);
/// assert_eq!(add_one.call(10), 11); // Can be called multiple times
/// ```
pub struct CFn<A, B>(pub BFn<A, B>);

/// A wrapper around `BFnOnce<A, B>` (a `Box<dyn FnOnce(A) -> B + 'static>`).
///
/// This struct provides a concrete type for heap-allocated, once-callable closures.
/// Similar to `CFn`, it's useful for type concretization.
///
/// # Examples
/// ```
/// use fp_rs::function::CFnOnce;
///
/// let s = "hello".to_string();
/// // This closure captures `s` by move, so it's FnOnce.
/// let append_s_once = CFnOnce::new(move |x: i32| format!("{}-{}", s, x));
/// assert_eq!(append_s_once.call_once(5), "hello-5");
/// // append_s_once.call_once(10); // This would be a compile error (use of moved value) if not for Box
///                               // but logically it's consumed.
/// ```
pub struct CFnOnce<A, B>(pub BFnOnce<A, B>);

impl<A, B> CFn<A, B> {
    /// Creates a new `CFn` by boxing the given closure.
    ///
    /// # Parameters
    /// - `f`: A closure that implements `Fn(A) -> B` and is `'static`.
    ///
    /// # Returns
    /// A new `CFn<A, B>` instance.
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(A) -> B + 'static,
    {
        CFn(Box::new(f))
    }

    /// Calls the wrapped closure.
    ///
    /// This method takes `&self` and the argument `arg` by value,
    /// allowing the closure to be called multiple times.
    ///
    /// # Parameters
    /// - `arg`: The argument of type `A` to pass to the closure.
    ///
    /// # Returns
    /// The result of type `B` from calling the closure.
    pub fn call(&self, arg: A) -> B {
        (self.0)(arg)
    }
}

impl<A, B> CFnOnce<A, B> {
    /// Creates a new `CFnOnce` by boxing the given closure.
    ///
    /// # Parameters
    /// - `f`: A closure that implements `FnOnce(A) -> B` and is `'static`.
    ///
    /// # Returns
    /// A new `CFnOnce<A, B>` instance.
    pub fn new<F>(f: F) -> Self
    where
        F: FnOnce(A) -> B + 'static,
    {
        CFnOnce(Box::new(f))
    }

    /// Calls the wrapped closure once.
    ///
    /// This method takes `self` by value, consuming the `CFnOnce` instance,
    /// reflecting the `FnOnce` nature of the underlying closure.
    ///
    /// # Parameters
    /// - `arg`: The argument of type `A` to pass to the closure.
    ///
    /// # Returns
    /// The result of type `B` from calling the closure.
    pub fn call_once(self, arg: A) -> B {
        (self.0)(arg)
    }
}

/// Allows `CFn<A, B>` to be dereferenced to `&Box<dyn Fn(A) -> B + 'static>`.
/// This enables calling the boxed closure directly using `(*cfn_instance)(arg)` syntax
/// if desired, though `cfn_instance.call(arg)` is generally preferred for clarity.
impl<A, B> Deref for CFn<A, B> {
    type Target = BFn<A, B>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Allows `CFnOnce<A, B>` to be dereferenced to `&Box<dyn FnOnce(A) -> B + 'static>`.
impl<A, B> Deref for CFnOnce<A, B> {
    type Target = BFnOnce<A, B>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Composes two boxed `Fn` closures.
/// Given `f: A -> B` and `g: B -> C`, returns a new boxed closure `h: A -> C`
/// such that `h(x) = g(f(x))`.
fn compose<A: 'static, B: 'static, C: 'static>(f: BFn<A, B>, g: BFn<B, C>) -> BFn<A, C> {
    Box::new(move |x| g(f(x)))
}

/// Composes two boxed `FnOnce` closures.
/// Given `f: A -> B` and `g: B -> C`, returns a new boxed closure `h: A -> C`
/// such that `h(x) = g(f(x))`.
/// The resulting closure is also `FnOnce`.
fn compose_fn_once<A: 'static, B: 'static, C: 'static>(
    f: BFnOnce<A, B>,
    g: BFnOnce<B, C>,
) -> BFnOnce<A, C> {
    Box::new(move |x| g(f(x))) // f and g are moved into the closure
}

/// Implements `f >> g` (forward composition) for `CFn`.
/// `(self >> rhs)(x)` is equivalent to `rhs(self(x))`.
/// `CFn<A,B> >> CFn<B,C>` results in `CFn<A,C>`.
impl<A: 'static, B: 'static, C: 'static> std::ops::Shr<CFn<B, C>> for CFn<A, B> {
    type Output = CFn<A, C>;
    fn shr(self, rhs: CFn<B, C>) -> Self::Output {
        // self is f: A -> B, rhs is g: B -> C
        // Result is g(f(x))
        CFn(compose(self.0, rhs.0))
    }
}

/// Implements `g << f` (backward composition) for `CFn`.
/// `(self << rhs)(x)` is equivalent to `self(rhs(x))`.
/// `CFn<B,C> << CFn<A,B>` results in `CFn<A,C>`.
impl<A: 'static, B: 'static, C: 'static> std::ops::Shl<CFn<A, B>> for CFn<B, C> {
    type Output = CFn<A, C>;
    fn shl(self, rhs: CFn<A, B>) -> Self::Output {
        // self is g: B -> C, rhs is f: A -> B
        // Result is g(f(x))
        CFn(compose(rhs.0, self.0))
    }
}

/// Implements `f >> g` (forward composition) for `CFnOnce`.
/// `(self >> rhs)(x)` is equivalent to `rhs(self(x))`.
/// `CFnOnce<A,B> >> CFnOnce<B,C>` results in `CFnOnce<A,C>`.
impl<A: 'static, B: 'static, C: 'static> std::ops::Shr<CFnOnce<B, C>> for CFnOnce<A, B> {
    type Output = CFnOnce<A, C>;
    fn shr(self, rhs: CFnOnce<B, C>) -> Self::Output {
        CFnOnce(compose_fn_once(self.0, rhs.0))
    }
}

/// Implements `g << f` (backward composition) for `CFnOnce`.
/// `(self << rhs)(x)` is equivalent to `self(rhs(x))`.
/// `CFnOnce<B,C> << CFnOnce<A,B>` results in `CFnOnce<A,C>`.
impl<A: 'static, B: 'static, C: 'static> std::ops::Shl<CFnOnce<A, B>> for CFnOnce<B, C> {
    type Output = CFnOnce<A, C>;
    fn shl(self, rhs: CFnOnce<A, B>) -> Self::Output {
        CFnOnce(compose_fn_once(rhs.0, self.0))
    }
}

/*
/// A wrapper for a function `A -> M<B>` (a Kleisli arrow) where `M` is a `Monad`.
///
/// This struct wraps a `CFn<A, BindType<M, A, B>>`, where `BindType<M, A, B>`
/// is an alias for `<M as Bind<A>>::Bind<B>` (i.e., `M<B>`).
/// So, `BindableFn` essentially holds a `CFn<A, M<B>>`.
///
/// This was likely intended for use with a more generic `bind` operation or
/// for Kleisli composition, but its usage seems limited in the current codebase
/// after `Bind` trait was simplified to take `Fn(A) -> Self::Bind<B>` directly.
///
/// The `BitOr` implementation for this type was removed due to orphan rules (E0210).
pub struct BindableFn<M: Bind<A> + Bind<B>, A, B>(CFn<A, BindType<M, A, B>>);

/// Type alias for the result type of a bind operation on monad `M`.
/// `BindType<M, A, B>` resolves to `<M as Bind<A>>::Bind<B>`, which is `M<B>`.
/// For example, if `M` is `Option<i32>` (so `A` is `i32`), and `B` is `String`,
/// then `BindType<Option<i32>, i32, String>` is `Option<String>`.
type BindType<M, A, B> = <M as Bind<A>>::Bind<B>;

/// Allows `BindableFn<M, A, B>` to be dereferenced to `&Box<dyn Fn(A) -> M<B> + 'static>`.
impl<M: Bind<A> + Bind<B>, A, B> Deref for BindableFn<M, A, B> {
    type Target = BFn<A, BindType<M, A, B>>; // This is BFn<A, M<B>>

    fn deref(&self) -> &Self::Target {
        &(self.0).0 // Dereference CFn to get BFn<A, M<B>>
    }
}

impl<M, A, B> BindableFn<M, A, B>
where
    // M must be Bind<A> and its output type M<B> must also be Bind<B> (for further composition).
    // The constraint `Bind<A, Bind<B> = M>` seems to imply M<B> = M, which is too restrictive.
    // It should likely be:
    // M: Bind<A, Bind<B> = TypeConstructorOfM<B>> + 'static,
    // TypeConstructorOfM<B>: Bind<B, Bind<Any> = TypeConstructorOfM<Any>> (if chaining binds)
    // Given the current Bind trait, M is the concrete type like Option<X>, Result<X,E>.
    // So M itself is not Bind<A> directly, but Self (e.g. Option<A>) is Bind<A>.
    // Let's assume M is the type constructor (e.g. Option, Result<_,E>)
    // This part of the code might need review based on HKT emulation strategy.
    // For now, sticking to existing bounds.
    M: Bind<A, Bind<B> = M> + Bind<B>, // This implies M<B> = M, which means B must be A.
                                      // This is likely not the intended general bound.
                                      // A more typical scenario is f: A -> M<B>.
                                      // The struct holds CFn<A, M<B>>.
                                      // Let's assume M is the type M<A> for the input.
                                      // And the function returns M<B>.
                                      // The struct is BindableFn<Option<A>, A, B> for f: A -> Option<B>
                                      // then M is Option<A>.
                                      // M must implement Bind<A>.
                                      // M must implement Bind<B> (this seems off, M is Option<A>, not Option<B>)
                                      // The BindType is <Option<A> as Bind<A>>::Bind<B> = Option<B>.
                                      // So the CFn is CFn<A, Option<B>>. This is correct.
                                      // The constraints on M for BindableFn itself are complex.
{
    /// Creates a new `BindableFn`.
    ///
    /// # Parameters
    /// - `f`: A function `A -> M` where `M` is `<OriginalMonadType as Bind<A>>::Bind<B>`.
    ///        Essentially, `f` is `A -> Monad<B>`.
    ///
    /// # Returns
    /// A new `BindableFn` instance.
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(A) -> M + 'static, // Here M is actually M<B> (the return type of the function)
                                 // The M in BindableFn<M,A,B> is the M from M<A> (the input monad type)
                                 // This is confusing. Let's clarify based on usage.
                                 // If f: A -> Option<B>, then BindableFn should wrap this.
                                 // BindableFn<Option<A>, A, B> would hold CFn<A, Option<B>>.
                                 // So, F: Fn(A) -> <Option<A> as Bind<A>>::Bind<B>
                                 // The M in new's F: Fn(A) -> M should be <SelfType as Bind<A>>::Bind<B>
    {
        BindableFn(CFn::new(f))
    }
}
*/
