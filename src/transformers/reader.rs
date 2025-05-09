use std::marker::PhantomData;
use crate::functor::Functor;
use crate::apply::Apply;
// use crate::function::CFn; // Removed unused import
use crate::applicative::Applicative;
use crate::monad::{Bind, Monad};
use crate::identity::Identity; // For the simple Reader case
use std::rc::Rc; // Use Rc for shared ownership

/// ReaderT is a monad transformer that adds a read-only environment to an inner monad.
///
/// `R` is the type of the environment.
/// `M` is the inner monad type constructor (e.g., `Option`, `Identity`).
/// `A` is the type of the value wrapped by the inner monad.
///
/// The core of `ReaderT` is a function `run_reader_t: Rc<dyn Fn(R) -> M>`.
/// (Using `M<A>` notation loosely here; in Rust, `M` would be `Option<A>`, `Identity<A>`, etc.)
pub struct ReaderT<R, M, A> {
    /// The function that, given an environment `R`, produces a monadic value `M` containing `A`.
    /// We use Rc<dyn Fn...> to store the function for shared access.
    pub run_reader_t: Rc<dyn Fn(R) -> M + 'static>,
    _phantom_a: PhantomData<A>, // To hold the type A for trait implementations
}

impl<R, M, A> ReaderT<R, M, A> {
    /// Creates a new `ReaderT` from a function `R -> M`.
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(R) -> M + 'static, // Is Fn
    {
        ReaderT {
            run_reader_t: Rc::new(f), // Wrap in Rc
            _phantom_a: PhantomData,
        }
    }
}

/// A simple Reader monad, equivalent to `ReaderT<R, Identity, A>`.
pub type Reader<R, A> = ReaderT<R, Identity<A>, A>;

// Functor for ReaderT
// R: Environment type
// M: Inner monad type (e.g., Option<A>, Identity<A>)
// A: Value type within M
// To be a Functor over A, M itself must be a Functor over A.
impl<R, M, A> Functor<A> for ReaderT<R, M, A>
where
    R: Clone + 'static, // Need Clone for env in closure
    M: Functor<A> + 'static,                         // Inner monad M (e.g. Option<A>) must be Functor over A and 'static
    A: 'static,                                      // Value type A must be 'static
    // The result of M.map (e.g. Option<B>) must also be 'static.
    // This is implicitly handled if B is 'static and M's structure preserves 'static.
    // Let's specify it for clarity for the associated type.
    // type Functor<T_val> = ReaderT<R, <M as Functor<A>>::Functor<T_val>, T_val>;
    // So, <M as Functor<A>>::Functor<BVal> must be 'static.
{
    type Functor<BVal> = ReaderT<R, <M as Functor<A>>::Functor<BVal>, BVal>;

    // Note: This impl requires Func: Fn, which is stricter than the Functor trait (FnMut).
    // This is a consequence of using Rc<dyn Fn> internally.
    fn map<B, Func>(self, f: Func) -> Self::Functor<B>
    where
        Func: Fn(A) -> B + Clone + 'static, // Changed to Fn, kept Clone for capture
    {
        // Clone the Rc for the new closure
        let run_reader_t_clone = self.run_reader_t.clone();
        ReaderT::new(move |env: R| { // Closure needs to be Fn
            // run_reader_t_clone is Rc<dyn Fn(R) -> M>
            // f is Fn(A) -> B + Clone
            // env needs to be Clone if used multiple times, added R: Clone to impl block
            let m_val = run_reader_t_clone(env); // Call the Rc<dyn Fn>
            // M::map should take FnMut, but we provide Fn. This is okay.
            m_val.map(f.clone()) // Clone f for the map call
        })
    }
}

// Apply for ReaderT
// To be Apply over A, M itself must be Apply over A.
// R must be Clone because the environment is used to run both self and i.
impl<R, M, A> Apply<A> for ReaderT<R, M, A>
where
    R: Clone + 'static, // Clone needed for env
    M: Apply<A> + 'static, // M is the inner monad structure, e.g., Option<_>, Identity<_>
                           // M must implement Apply for the type A it currently holds.
    A: 'static,
    // The type of the inner monad in `i` (the ReaderT containing the function).
    // This is <M as Functor<A>>::Functor<CFn<A, B_VAL_TYPE_FOR_APPLY_FN_OUTPUT>>.
    // Example: If M is Option<A>, this is Option<CFn<A, B_VAL_TYPE_FOR_APPLY_FN_OUTPUT>>.
    // This type must be 'static.
    // Note: Self::Fnn<A, B_VAL> is CFn<A, B_VAL>
    // These complex bounds involving CFn might not be necessary if M::apply works correctly.
    // Let's remove them for now and rely on the method constraints.
    // <M as Functor<A>>::Functor<CFn<A, <M as Apply<A>>::Apply<A>>>: 'static,
    // <M as Functor<A>>::Functor<CFn<A, A>>: 'static,
    // <M as Apply<A>>::Apply<A>: 'static,
{
    // type Apply<TVal> is the resulting ReaderT structure, containing TVal.
    // The inner monad of this result is <M as Apply<A>>::Apply<TVal>.
    type Apply<TVal> = ReaderT<R, <M as Apply<A>>::Apply<TVal>, TVal>;

    // type Fnn<TArg, TRes> is the type of the function *inside* the functor context
    // when `apply` is called. For ReaderT, this means the function inside the
    // *inner* monad M, when that inner monad is wrapped by ReaderT.
    // The `apply` signature is `self.apply(i: ReaderT<R, M<CFn<A,B>>, CFn<A,B>>)`
    // So, Fnn is CFn.
    // type Fnn<TArg, TRes> = CFn<TArg, TRes>; // Old
    type Fnn<TArg, TRes> = <M as Apply<A>>::Fnn<TArg, TRes>; // New: Defer to M's Fnn

    fn apply<B>(
        self, // Takes self by value (moves the Rc)
        i: <Self as Functor<A>>::Functor<Self::Fnn<A, B>>, // Takes i by value (moves the Rc)
    ) -> Self::Apply<B>
    where
        Self: Sized,
        B: 'static, // Added missing bound from trait
        // Add constraint for M's Fnn type if needed, but let's see if it compiles without.
        // <M as Apply<A>>::Fnn<A, B> = CFn<A, B>, // Removed unstable constraint
        <Self as Functor<A>>::Functor<<Self as Apply<A>>::Fnn<A, B>>: 'static,
    {
        // Clone Rcs for the new closure
        let self_run = self.run_reader_t.clone();
        let i_run = i.run_reader_t.clone();
        ReaderT::new(move |env: R| { // Closure needs to be Fn
            // env needs Clone
            let m_val = self_run(env.clone()); // Call the Rc<dyn Fn>
            let m_func = i_run(env);           // Call the Rc<dyn Fn>

            // Now, call Apply::apply on m_val (which is M<A>)
            // M::apply takes `self` (M<A>) and `i` (M<Fnn<A,B>>)
            // m_func is M<CFn<A,B>>. If M's Fnn is CFn, this should work.
            m_val.apply(m_func) // This returns <M as Apply<A>>::Apply<B>
        }) // Close the closure
} // Close apply method body
} // Close impl Apply block

// Applicative for ReaderT
// To be Applicative over A, M itself must be Applicative over A.
impl<R, M, A> Applicative<A> for ReaderT<R, M, A>
where
    R: Clone + 'static, // Need Clone for env in pure's closure? No, _env.
    M: Applicative<A> + 'static, // M is e.g. Option<_>, Identity<_>
    A: Clone + 'static,          // Clone for v in closure, 'static for M::pure
    // The result of M::pure(v), e.g. Option<A>, must be 'static
    <M as Applicative<A>>::Applicative<A>: 'static,
{
    // type Applicative<TVal> is the resulting ReaderT structure.
    // Inner monad is <M as Applicative<A>>::Applicative<TVal>
    type Applicative<TVal> = ReaderT<R, <M as Applicative<A>>::Applicative<TVal>, TVal>;

    fn pure(v: A) -> Self::Applicative<A> // Result is ReaderT<R, <M as Applicative<A>>::Applicative<A>, A>
    where
        // Ensure the specific resulting inner monad type for A is 'static
        <M as Applicative<A>>::Applicative<A>: 'static,
    {
        // Closure needs to be Fn
        ReaderT::new(move |_env: R| {
            // M::pure comes from Applicative<A> for M.
            // It takes A and returns <M as Applicative<A>>::Applicative<A>
            // e.g. if M is Option, A is i32, M::pure(v) is Some(v) of type Option<i32>
            M::pure(v.clone()) // v is cloned as it's captured by a 'static move closure
        })
    }
}

// Bind for ReaderT
// To be Bind over A, M itself must be Bind over A.
// R must be Clone because the environment is used by self.run_reader_t and
// also by the run_reader_t of the ReaderT returned by f.
impl<R, M, A> Bind<A> for ReaderT<R, M, A>
where
    R: Clone + 'static, // Need Clone for env
    M: Bind<A> + 'static, // M is e.g. Option<_>, Identity<_>
    A: 'static,
    // The type of the inner monad in the ReaderT returned by f.
    // This is <M as Bind<A>>::Bind<B_VAL_TYPE_FOR_BIND_FN_OUTPUT>.
    // e.g. if M is Option<A>, f returns ReaderT<R, Option<B>, B>, so inner is Option<B>.
    // This type must be 'static.
    // <M as Bind<A>>::Bind<A> is a placeholder for the generic B in the method.
    // Let's remove this placeholder bound.
    // <M as Bind<A>>::Bind<A>: 'static,
{
    // type Bind<TVal> is the resulting ReaderT structure.
    // The inner monad of this result is <M as Bind<A>>::Bind<TVal>.
    type Bind<TVal> = ReaderT<R, <M as Bind<A>>::Bind<TVal>, TVal>;

    fn bind<B, F>(self, f: F) -> Self::Bind<B> // Takes self by value (moves Rc)
    where
        F: Fn(A) -> Self::Bind<B> + Clone + 'static, // Added Clone, matches trait
    {
        // Clone Rcs/functions for the new closure
        let self_run = self.run_reader_t.clone();
        // f is Fn + Clone, so it can be captured by cloning and is 'static
        ReaderT::new(move |env: R| { // Closure needs to be Fn
            // env needs Clone
            let m_a_val = self_run(env.clone()); // Call the Rc<dyn Fn>

            // We need to call m_a_val.bind(g) where g: A -> <M as Bind<A>>::Bind<B>
            // The closure g captures f (cloned Fn) and env (cloned R).
            // g needs to return M<B>.
            let f_clone = f.clone(); // Clone f for the inner closure
            m_a_val.bind(move |a_val: A| {
                // f_clone is Fn(A) -> ReaderT<R, M<B>, B>
                // Call f_clone to get the next ReaderT
                let next_reader_t: Self::Bind<B> = f_clone(a_val);
                // Call the function inside the next ReaderT with the *cloned* env
                (next_reader_t.run_reader_t)(env.clone())
            })
        })
    }
}

// Monad for ReaderT
impl<R, M, A> Monad<A> for ReaderT<R, M, A>
where
    R: Clone + 'static, // From Apply/Bind
    M: Monad<A> + 'static, // Inner monad M must be a full Monad
    A: Clone + 'static,    // Clone from Applicative pure
    // Constraints from Applicative pure
    <M as Applicative<A>>::Applicative<A>: 'static, // Needed? Let's remove if not required by compiler.
    // Constraints from Bind (specifically for the associated type <M as Bind<A>>::Bind<A>)
    // <M as Bind<A>>::Bind<A>: 'static, // Removed placeholder
{
    // Monad is a marker trait, relies on Applicative and Bind implementations.
}

/// Trait for monads that can read from a shared environment.
///
/// `REnv` is the type of the environment.
/// `AVal` is the type of the value within the monad (for `local`).
pub trait MonadReader<REnv, AVal>
where
    Self: Sized, // The type implementing the trait, e.g., ReaderT<REnv, M_Inner, AVal>
{
    /// The specific type of `Self` that `ask` returns.
    /// This will be `Self` but with `AVal` specialized to `REnv`.
    /// For `ReaderT<REnv, M_Inner, AVal>`, this is `ReaderT<REnv, M_Inner_Pure_R, REnv>`.
    type SelfWithEnvAsValue;

    /// Retrieves the current environment.
    fn ask() -> Self::SelfWithEnvAsValue
    where
        REnv: Clone + 'static, // Environment must be cloneable and static for pure.
        Self::SelfWithEnvAsValue: Sized; // Constraints for the concrete return type in impl.

    /// Executes a computation in a temporarily modified environment.
    ///
    /// `map_env_fn` is a function that transforms the current environment.
    /// `computation` is the monadic action to run with the transformed environment.
    fn local<FMapEnv>(
        map_env_fn: FMapEnv,
        computation: Self, // Self is e.g. ReaderT<REnv, M_Inner, AVal>
    ) -> Self
    where
        REnv: 'static, // Environment type for the computation.
        AVal: 'static, // Value type for the computation.
        FMapEnv: Fn(REnv) -> REnv + 'static; // Changed FnMut to Fn
}

impl<R, M, A> MonadReader<R, A> for ReaderT<R, M, A>
where
    R: 'static, // For local's map_env_fn and computation's R. Also for ask's R.
    A: 'static, // For local's computation's A.
    M: 'static, // For local's computation's M (the inner monad type).
    // For `ask`: M must be Applicative over R (the environment type).
    // M::pure(env.clone()) will be called.
    M: Applicative<R>,
    // The result of M::pure(env.clone()), e.g. Option<R>, Identity<R>, must be 'static.
    <M as Applicative<R>>::Applicative<R>: 'static,
{
    // SelfWithEnvAsValue for ReaderT<R,M,A> when A becomes R is:
    // ReaderT<R, Resulting_M_from_Pure_R, R>
    // Resulting_M_from_Pure_R is <M as Applicative<R>>::Applicative<R>
    type SelfWithEnvAsValue = ReaderT<R, <M as Applicative<R>>::Applicative<R>, R>;

    fn ask() -> Self::SelfWithEnvAsValue
    where
        R: Clone + 'static, // ask needs to clone the environment to pure it.
    {
        // Closure needs to be Fn
        ReaderT::new(move |env: R| {
            // M::pure comes from M's Applicative<R> implementation.
            // It takes R and returns <M as Applicative<R>>::Applicative<R>.
            // e.g., if M is Option<_> and R is String, M::pure takes String, returns Option<String>.
            M::pure(env.clone())
        }) // Close the closure
} // Close ask method body

    fn local<FMapEnv>(
        map_env_fn: FMapEnv, // No longer mut
        computation: Self,    // No longer mut, takes ownership of Rc
    ) -> Self
    where
        FMapEnv: Fn(R) -> R + 'static, // Changed to Fn
    {
        // Clone Rc for the new closure
        let computation_run = computation.run_reader_t.clone();
        // map_env_fn is Fn, capture by reference
        ReaderT::new(move |current_env: R| { // Closure needs to be Fn
            let modified_env = map_env_fn(current_env);
            // computation_run is Rc<dyn Fn(R) -> M>
            computation_run(modified_env) // Call the Rc<dyn Fn>
        })
    }
}
