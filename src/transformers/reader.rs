//! # ReaderT Monad Transformer
//!
//! This module provides `ReaderT<R, M, A>`, a monad transformer that adds a read-only
//! environment of type `R` to an underlying monad `M` which wraps a value of type `A`.
//!
//! ## Purpose
//!
//! `ReaderT` is used to build computations that depend on a shared, read-only environment.
//! This is useful for:
//! - Dependency Injection: Passing configurations, database connections, or other services.
//! - Context Propagation: Making contextual information available throughout a computation
//!   without explicitly passing it through every function call.
//!
//! ## Structure
//!
//! ```
//! # use std::rc::Rc;
//! # use std::marker::PhantomData;
//! pub struct ReaderT<R, M, A> {
//!     pub run_reader_t: Rc<dyn Fn(R) -> M + 'static>,
//!     _phantom_a: PhantomData<A>,
//! }
//! ```
//!
//! - `R`: The type of the read-only environment.
//! - `M`: The inner monad (e.g., `Option<A>`, `Result<A, E>`, `Identity<A>`).
//!        Note that `M` here represents the monadic structure, so if the inner monad
//!        is `Option<A>`, `M` in `ReaderT<R, M, A>` conceptually refers to `Option<_>`.
//!        In Rust's type system, `M` will be the fully-qualified type like `Option<A>`.
//! - `A`: The type of the value computed within the context of `M` and `R`.
//! - `run_reader_t`: The core of `ReaderT`. It's a function (wrapped in `Rc` for shared
//!   ownership and dynamic dispatch) that takes an environment `R` and produces a
//!   value of the inner monad `M` (which itself contains or computes `A`).
//!
//! ## `Reader<R, A>` Type Alias
//!
//! A common use case is when the inner monad `M` is `Identity`. This simplifies `ReaderT`
//! to a basic `Reader` monad:
//!
//! ```
//! # use fp_rs::transformers::reader::ReaderT; // Or adjust path if used internally
//! # use fp_rs::identity::Identity; // Or adjust path
//! pub type Reader<R, A> = ReaderT<R, Identity<A>, A>;
//! ```
//!
//! This `Reader<R, A>` represents a computation that takes an `R` and produces an `A`.
//!
//! ## Trait Implementations
//!
//! `ReaderT` implements `Functor`, `Apply`, `Applicative`, `Monad`, and `MonadReader`
//! provided that the inner monad `M` and the types `R` and `A` satisfy certain constraints
//! (e.g., `M` must also implement the corresponding trait, `R` often needs to be `Clone`).
//!
//! - **`Functor`**: `map` applies a function to the value `A` *inside* the `ReaderT` context.
//!   This means it transforms the result of the computation, after the environment `R`
//!   has been supplied and the inner monad `M` has produced `A`.
//! - **`Apply`**: `apply` allows applying a function wrapped in `ReaderT` to a value
//!   wrapped in `ReaderT`. Both the function and the value computations will receive
//!   the same environment `R`.
//! - **`Applicative`**: `pure` lifts a value `A` into the `ReaderT` context. The resulting
//!   `ReaderT` will produce this value `A` (wrapped in `M::pure(A)`) regardless of the
//!   environment `R` supplied.
//! - **`Monad`**: `bind` (or `flat_map`) allows sequencing computations where the next
//!   computation depends on the result of the previous one. The environment `R` is
//!   passed through.
//! - **`MonadReader`**: This trait provides `ask` (to retrieve the current environment `R`)
//!   and `local` (to run a sub-computation with a modified environment).
//!
//! ## Key Constraints and Design Choices
//!
//! - **`Rc<dyn Fn(R) -> M + 'static>`**: The internal function `run_reader_t` uses `Rc`
//!   to allow the `ReaderT` to be cloned (sharing the underlying computation) and `dyn Fn`
//!   for type erasure. The `'static` bound is common for `dyn Trait` objects.
//!   This choice means that the functions used to construct and map `ReaderT` often
//!   need to be `Fn` (not `FnMut` or `FnOnce`) and `'static`.
//! - **`R: Clone + 'static`**: The environment `R` often needs to be `Clone` because
//!   it might be used multiple times (e.g., in `apply` or `bind` where the environment
//!   is passed to multiple sub-computations). It also often needs to be `'static`.
//! - **`A: 'static`**: The value type `A` usually needs to be `'static`.
//! - **`M: Trait<A> + 'static`**: The inner monad `M` (e.g., `Option<A>`) must typically
//!   implement the corresponding trait (e.g., `Functor<A>`, `Monad<A>`) and be `'static`.
//!   The associated types from `M`'s trait implementations (e.g., `<M as Functor<A>>::Functor<BVal>`)
//!   also often need to be `'static`.

use std::marker::PhantomData;
use std::rc::Rc; // Use Rc for shared ownership

use crate::applicative::Applicative;
use crate::apply::Apply;
// use crate::function::CFn; // Removed unused import
use crate::functor::Functor;
use crate::identity::Identity; // For the simple Reader case
use crate::monad::{Bind, Monad};

/// A monad transformer that adds a read-only environment `R` to an inner monad `M`.
///
/// `ReaderT<R, M, A>` represents a computation that, when provided with an environment
/// of type `R`, produces a value of type `A` wrapped in the context of an inner monad `M`.
///
/// # Type Parameters
///
/// - `R`: The type of the read-only environment.
/// - `M`: The inner monad type (e.g., `Option<A>`, `Result<A, E>`, `Identity<A>`).
///        This type parameter represents the full monadic value, like `Option<String>`,
///        not just the `Option` constructor.
/// - `A`: The type of the value computed by the `ReaderT` and wrapped within `M`.
///
/// # Core Idea
///
/// The essence of `ReaderT` is its `run_reader_t` field. This is a function that takes
/// an environment `R` and returns a monadic value `M` (which itself contains `A`).
/// All operations (`map`, `bind`, etc.) on `ReaderT` essentially transform this
/// underlying function.
///
/// # Example
///
/// ```
/// use fp_rs::transformers::reader::{ReaderT, Reader};
/// use fp_rs::identity::Identity;
/// use fp_rs::functor::Functor;
/// use fp_rs::monad::Monad; // For pure and bind if using Monad trait directly
/// use fp_rs::applicative::Applicative; // For pure
/// use std::rc::Rc;
///
/// // Configuration for our application
/// #[derive(Clone)]
/// struct Config {
///     greeting: String,
/// }
///
/// // A simple Reader using Identity as the inner monad
/// type AppReader<A> = Reader<Config, A>; // Equivalent to ReaderT<Config, Identity<A>, A>
///
/// fn greet_user(user_name: String) -> AppReader<String> {
///     ReaderT::new(move |config: Config| {
///         Identity(format!("{}, {}!", config.greeting, user_name))
///     })
/// }
///
/// fn main() {
///     let config = Config { greeting: "Hello".to_string() };
///     let computation = greet_user("Alice".to_string());
///
///     // To run the computation, call run_reader_t with the config
///     let result_identity = (computation.run_reader_t)(config.clone());
///     assert_eq!(result_identity.0, "Hello, Alice!");
///
///     // Using map
///     let computation_len = greet_user("Bob".to_string()).map(|s| s.len());
///     let len_identity = (computation_len.run_reader_t)(config);
///     assert_eq!(len_identity.0, "Hello, Bob!".len());
/// }
/// ```
pub struct ReaderT<R, M, A> {
    /// The function that, given an environment `R`, produces a monadic value `M` (containing `A`).
    ///
    /// This is the core of the `ReaderT` structure. It's wrapped in `Rc` to allow
    /// `ReaderT` instances to be cloned and share the underlying computation.
    /// The function must be `'static` due to the `dyn Fn` usage.
    pub run_reader_t: Rc<dyn Fn(R) -> M + 'static>,
    _phantom_a: PhantomData<A>, // PhantomData to correctly track the type `A`.
}

impl<R, M, A> ReaderT<R, M, A> {
    /// Creates a new `ReaderT` from a function `f: R -> M`.
    ///
    /// The provided function `f` is the computation that will be executed when the
    /// `ReaderT` is run with an environment.
    ///
    /// # Parameters
    ///
    /// - `f`: A function that takes an environment of type `R` and returns a value
    ///        of the inner monad type `M` (which wraps `A`). The function must
    ///        be `Fn` (not `FnMut` or `FnOnce`) and `'static`.
    ///
    /// # Returns
    ///
    /// A new `ReaderT<R, M, A>` instance.
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(R) -> M + 'static,
    {
        ReaderT {
            run_reader_t: Rc::new(f),
            _phantom_a: PhantomData,
        }
    }
}

/// A type alias for `ReaderT` where the inner monad `M` is `Identity<A>`.
///
/// This represents a simpler "Reader" monad that directly produces a value `A`
/// from an environment `R`, without an additional monadic layer beyond `Identity`.
///
/// `Reader<R, A>` is equivalent to `ReaderT<R, Identity<A>, A>`.
pub type Reader<R, A> = ReaderT<R, Identity<A>, A>;

/// Implements `Functor` for `ReaderT<R, M, A>`.
///
/// For `ReaderT` to be a `Functor` over `A`, its inner monad `M` must also be a
/// `Functor` over `A`. The environment type `R` needs to be `Clone` because the
/// mapping function might be applied after the environment is captured.
///
/// The `map` operation transforms the eventual result `A` of the `ReaderT` computation.
/// It does this by creating a new `ReaderT` whose `run_reader_t` function, when called,
/// will first run the original `ReaderT`'s function with the environment, and then
/// `map` the provided mapping function `f` over the resulting inner monad `M`.
impl<R, M, A> Functor<A> for ReaderT<R, M, A>
where
    R: Clone + 'static, // `R` must be `Clone` as the environment is captured and potentially reused.
    M: Functor<A> + 'static, // Inner monad `M` (e.g., `Option<A>`) must be a `Functor` over `A`.
    A: 'static,          // Value type `A` must be `'static`.
                         // The result of `M.map` (e.g., `Option<B>`) must also be `'static`.
                         // This is typically ensured if `B` is `'static` and `M`'s `Functor` structure preserves static lifetimes.
                         // Specifically, `<M as Functor<A>>::Functor<BVal>` (the type of the inner monad after mapping) must be `'static`.
{
    /// The resulting `Functor` type after mapping.
    /// For `ReaderT<R, M, A>`, mapping to `BVal` results in
    /// `ReaderT<R, <M as Functor<A>>::Functor<BVal>, BVal>`.
    type Functor<BVal> = ReaderT<R, <M as Functor<A>>::Functor<BVal>, BVal>;

    /// Applies a function `f` to the value `A` computed by this `ReaderT`.
    ///
    /// The function `f` must be `Fn(A) -> B`, `Clone`, and `'static`.
    /// `Clone` is required because `f` is captured by the new `ReaderT`'s closure
    /// and might be called multiple times if the `ReaderT` is run multiple times.
    /// `Fn` (not `FnMut`) is required due to the `Rc<dyn Fn>` internal storage.
    ///
    /// # Returns
    ///
    /// A new `ReaderT` that, when run, will produce a value of type `B`
    /// (wrapped in the mapped inner monad).
    fn map<B, Func>(self, f: Func) -> Self::Functor<B>
    where
        Func: Fn(A) -> B + Clone + 'static, // `f` must be `Fn` and `Clone`.
    {
        let run_reader_t_clone = self.run_reader_t.clone();
        ReaderT::new(move |env: R| {
            let m_val = run_reader_t_clone(env); // Run original computation with the environment.
            m_val.map(f.clone()) // Map `f` over the result from the inner monad `M`.
        })
    }
}

/// Implements `Apply` for `ReaderT<R, M, A>`.
///
/// For `ReaderT` to be `Apply` over `A`, its inner monad `M` must also be `Apply`
/// over `A`. The environment `R` must be `Clone` because it's supplied to both
/// the `ReaderT` containing the value and the `ReaderT` containing the function.
///
/// `apply` takes a `ReaderT` containing a function (`i`) and applies it to this
/// `ReaderT` (containing a value). Both computations receive the same environment.
impl<R, M, A> Apply<A> for ReaderT<R, M, A>
where
    R: Clone + 'static,      // `R` must be `Clone` to be passed to both computations.
    M: Apply<A> + 'static,   // Inner monad `M` must be `Apply` over `A`.
    A: 'static,              // Value type `A` must be `'static`.
                             // Constraints on associated types like `<M as Apply<A>>::Apply<TVal>` and
                             // `<M as Functor<A>>::Functor<Self::Fnn<A, B>>` needing to be `'static`
                             // are important for the resulting `ReaderT`'s `run_reader_t` function.
{
    /// The resulting `Apply` type.
    /// For `ReaderT<R, M, A>`, applying to get `TVal` results in
    /// `ReaderT<R, <M as Apply<A>>::Apply<TVal>, TVal>`.
    type Apply<TVal> = ReaderT<R, <M as Apply<A>>::Apply<TVal>, TVal>;

    /// The type of the function `A -> B` when wrapped within the context of `M`.
    /// This defers to the inner monad `M`'s `Fnn` associated type.
    type Fnn<TArg, TRes> = <M as Apply<A>>::Fnn<TArg, TRes>;

    /// Applies a function wrapped in a `ReaderT` to the value computed by this `ReaderT`.
    ///
    /// `self` is `ReaderT<R, M, A>` (the value part).
    /// `i` is `ReaderT<R, M', Fnn<A, B>>` (the function part), where `M'` is the
    /// result of `M.map` and `Fnn<A, B>` is the function type `A -> B` within `M`.
    ///
    /// Both `self.run_reader_t` and `i.run_reader_t` are called with the same cloned
    /// environment. The resulting `M<A>` and `M<Fnn<A,B>>` are then combined using
    /// `M`'s `apply` method.
    ///
    /// # Type Parameters
    /// - `B`: The result type of the function application. Must be `'static`.
    fn apply<B>(
        self,
        i: <Self as Functor<A>>::Functor<Self::Fnn<A, B>>,
    ) -> Self::Apply<B>
    where
        Self: Sized,
        B: 'static, // Result type `B` must be `'static`.
        // This bound ensures that the specific type of `i` (the ReaderT containing the function)
        // has an inner monad part that is 'static.
        <Self as Functor<A>>::Functor<<Self as Apply<A>>::Fnn<A, B>>: 'static,
    {
        let self_run = self.run_reader_t.clone();
        let i_run = i.run_reader_t.clone();
        ReaderT::new(move |env: R| {
            let m_val = self_run(env.clone()); // Get `M<A>` using the environment.
            let m_func = i_run(env);           // Get `M<Fnn<A,B>>` using the same environment.
            m_val.apply(m_func) // Use inner monad `M`'s `apply` method.
        })
    }
}

/// Implements `Applicative` for `ReaderT<R, M, A>`.
///
/// For `ReaderT` to be `Applicative` over `A`, its inner monad `M` must also be
/// `Applicative` over `A`. The value `A` needs to be `Clone` and `'static`
/// because it's captured by the closure in `pure` and lifted into `M`.
///
/// `pure` lifts a value `A` into the `ReaderT` context. The resulting `ReaderT`
/// will always produce `M::pure(v)` regardless of the environment.
impl<R, M, A> Applicative<A> for ReaderT<R, M, A>
where
    R: Clone + 'static, // `R` is part of the `ReaderT` structure, though not directly used by `pure`'s logic.
    M: Applicative<A> + 'static, // Inner monad `M` must be `Applicative` over `A`.
    A: Clone + 'static,          // `A` must be `Clone` (for capture) and `'static`.
    // The result of `M::pure(v)` (e.g., `Option<A>`) must be `'static`.
    // This is `<M as Applicative<A>>::Applicative<A>`.
    <M as Applicative<A>>::Applicative<A>: 'static,
{
    /// The resulting `Applicative` type.
    /// For `ReaderT<R, M, A>`, `pure`ing `TVal` results in
    /// `ReaderT<R, <M as Applicative<A>>::Applicative<TVal>, TVal>`.
    type Applicative<TVal> = ReaderT<R, <M as Applicative<A>>::Applicative<TVal>, TVal>;

    /// Lifts a value `v` into the `ReaderT` applicative context.
    ///
    /// The returned `ReaderT` will, for any given environment, produce `M::pure(v.clone())`.
    /// The value `v` is cloned as it's captured by a `'static` closure.
    fn pure(v: A) -> Self::Applicative<A>
    where
        // This bound ensures the specific inner monad type resulting from M::pure(A) is 'static.
        <M as Applicative<A>>::Applicative<A>: 'static,
    {
        ReaderT::new(move |_env: R| { // The environment `_env` is ignored.
            M::pure(v.clone()) // Use inner monad `M`'s `pure` method.
        })
    }
}

/// Implements `Bind` (and thus `Monad`) for `ReaderT<R, M, A>`.
///
/// For `ReaderT` to be `Bind` over `A`, its inner monad `M` must also be `Bind`
/// over `A`. The environment `R` must be `Clone` because it's used by the initial
/// computation and potentially by the computation returned by the function `f`.
///
/// `bind` allows sequencing `ReaderT` computations. It takes a function `f` that
/// receives the result `A` of the current `ReaderT` and returns a new `ReaderT`.
/// The environment is passed to both stages.
impl<R, M, A> Bind<A> for ReaderT<R, M, A>
where
    R: Clone + 'static,      // `R` must be `Clone` to be passed to both computations.
    M: Bind<A> + 'static,    // Inner monad `M` must be `Bind` over `A`.
    A: 'static,              // Value type `A` must be `'static`.
                             // The inner monad type of the `ReaderT` returned by `f`,
                             // which is `<M as Bind<A>>::Bind<BVal>`, must be `'static`.
{
    /// The resulting `Bind` type.
    /// For `ReaderT<R, M, A>`, binding to `TVal` results in
    /// `ReaderT<R, <M as Bind<A>>::Bind<TVal>, TVal>`.
    type Bind<TVal> = ReaderT<R, <M as Bind<A>>::Bind<TVal>, TVal>;

    /// Sequentially composes this `ReaderT` with another `ReaderT`-producing function `f`.
    ///
    /// `f` takes the result `A` of the current `ReaderT`'s computation and returns
    /// a new `ReaderT<R, <M as Bind<A>>::Bind<B>, B>`.
    ///
    /// The process:
    /// 1. The current `ReaderT` (`self`) is run with the environment `env` to get `m_a_val: M<A>`.
    /// 2. `m_a_val` (the inner monad) is bound with a new function. This new function:
    ///    a. Takes `a_val: A` (the unwrapped value from `m_a_val`).
    ///    b. Calls `f(a_val)` to get `next_reader_t: ReaderT<R, ..., B>`.
    ///    c. Runs `next_reader_t` with the *same environment* `env` (cloned) to get the
    ///       final inner monadic value `M<B>`.
    ///
    /// The function `f` must be `Fn(A) -> Self::Bind<B>`, `Clone`, and `'static`.
    fn bind<B, F>(self, f: F) -> Self::Bind<B>
    where
        F: Fn(A) -> Self::Bind<B> + Clone + 'static, // `f` must be `Fn` and `Clone`.
        // Ensure the inner monad type of the ReaderT returned by f is 'static. (This was too strict)
        // This is for a generic B. The 'static nature should come from M's impl or B: 'static.
        // <M as Bind<A>>::Bind<B>: 'static, // Removed stricter bound
    {
        let self_run = self.run_reader_t.clone();
        ReaderT::new(move |env: R| {
            let m_a_val = self_run(env.clone()); // Run self with env -> M<A>

            let f_clone = f.clone(); // Clone `f` for the inner closure.
            m_a_val.bind(move |a_val: A| { // Bind on M<A>
                // `a_val` is the `A` from `M<A>`.
                // `f_clone` takes `A` and returns `ReaderT<R, M_bind_B, B>`.
                let next_reader_t: Self::Bind<B> = f_clone(a_val);
                // Run the returned ReaderT with the same (cloned) environment.
                (next_reader_t.run_reader_t)(env.clone()) // -> M_bind_B (e.g. Option<B>)
            })
        })
    }
}

/// Implements the `Monad` marker trait for `ReaderT<R, M, A>`.
///
/// `Monad` combines `Applicative` and `Bind`. Since `ReaderT` implements both (given
/// appropriate constraints on `R`, `M`, and `A`), it is a `Monad`.
/// This is a marker trait and does not add new methods here beyond those inherited
/// from `Applicative` and `Bind`.
impl<R, M, A> Monad<A> for ReaderT<R, M, A>
where
    R: Clone + 'static, // Required by `Apply` and `Bind`.
    M: Monad<A> + 'static, // Inner monad `M` must be a full `Monad` over `A`.
    A: Clone + 'static,    // Required by `Applicative::pure`.
    // Constraint from `Applicative::pure` ensuring `M::pure(A)` result is `'static`.
    <M as Applicative<A>>::Applicative<A>: 'static,
    // Constraint from `Bind::bind` ensuring the inner monad of the result for a generic B is 'static.
    // This is a general statement; specific instantiations like <M as Bind<A>>::Bind<SomeType>
    // would be checked at call sites of bind. We state it for A as a representative.
    // If this causes issues, it might be better to ensure <M as Bind<A>>::Bind<BVal>: 'static in bind's where clause.
    // For now, let's assume the compiler infers this correctly or it's covered by M: Monad<A> + 'static.
    // <M as Bind<A>>::Bind<A>: 'static, // Placeholder, typically handled by M: Monad.
{
    // Monad is a marker trait, relying on `Applicative` and `Bind` implementations.
}

/// Trait for monads that provide read-only access to a shared environment.
///
/// This trait is typically implemented by `ReaderT` or similar structures.
///
/// # Type Parameters
///
/// - `REnv`: The type of the environment that can be read.
/// - `AVal`: The type of the value within the monad. This is relevant for `local`,
///           as `local` operates on `Self` which is `MonadReader<REnv, AVal>`.
pub trait MonadReader<REnv, AVal>
where
    Self: Sized, // The type implementing the trait, e.g., `ReaderT<REnv, MInner, AVal>`.
{
    /// The specific type of `Self` that `ask` returns.
    ///
    /// When `ask` is called, it returns a monadic computation that yields the environment
    /// itself. So, if `Self` is `MonadReader<REnv, AVal>`, `ask` returns a
    /// `MonadReader<REnv, REnv>` (conceptually). This associated type specifies the
    /// concrete type for that, e.g., for `ReaderT<REnv, MInner, AVal>`, this would be
    /// `ReaderT<REnv, <MInner as Applicative<REnv>>::Applicative<REnv>, REnv>`.
    type SelfWithEnvAsValue;

    /// Retrieves the current environment `REnv` from within the monad.
    ///
    /// # Returns
    ///
    /// A monadic computation (of type `Self::SelfWithEnvAsValue`) that, when run,
    /// will produce the current environment.
    ///
    /// # Constraints
    ///
    /// - `REnv` must be `Clone + 'static` because the environment is cloned and
    ///   lifted into the monadic context using `Applicative::pure`.
    fn ask() -> Self::SelfWithEnvAsValue
    where
        REnv: Clone + 'static,
        Self::SelfWithEnvAsValue: Sized; // Ensures the return type is known.

    /// Executes a computation `computation` within a temporarily modified environment.
    ///
    /// The `map_env_fn` function takes the current environment and returns a new
    /// environment. The `computation` (which is `Self`) is then run using this
    /// modified environment. After `local` completes, the environment reverts to
    /// what it was before `local` was called.
    ///
    /// # Parameters
    ///
    /// - `map_env_fn`: A function `Fn(REnv) -> REnv + 'static` that transforms the environment.
    /// - `computation`: The monadic action (`Self`) to run with the modified environment.
    ///
    /// # Returns
    ///
    /// A new monadic computation (`Self`) that encapsulates this localized execution.
    ///
    /// # Constraints
    ///
    /// - `REnv` and `AVal` (from `Self`) must be `'static`.
    /// - `FMapEnv` (the type of `map_env_fn`) must be `Fn(REnv) -> REnv + 'static`.
    fn local<FMapEnv>(
        map_env_fn: FMapEnv,
        computation: Self,
    ) -> Self
    where
        REnv: 'static,
        AVal: 'static,
        FMapEnv: Fn(REnv) -> REnv + 'static;
}

/// Implements `MonadReader` for `ReaderT<R, M, A>`.
impl<R, M, A> MonadReader<R, A> for ReaderT<R, M, A>
where
    R: 'static, // Environment type `R` must be `'static`.
    A: 'static, // Value type `A` (from `ReaderT<R,M,A>`) must be `'static`.
    M: 'static, // Inner monad type `M` (e.g. `Option<A>`) must be `'static`.
    // For `ask`: The inner monad `M` must be `Applicative` over the environment type `R`
    // because `ask` uses `M::pure(environment_value)`.
    M: Applicative<R>,
    // The result of `M::pure(env.clone())` (e.g., `Option<R>`, `Identity<R>`) must be `'static`.
    // This is `<M as Applicative<R>>::Applicative<R>`.
    <M as Applicative<R>>::Applicative<R>: 'static,
{
    /// The type of `ReaderT` when its value part `A` becomes the environment `R`.
    /// This is `ReaderT<R, InnerMonadPureR, R>`, where `InnerMonadPureR` is
    /// the type of the inner monad `M` after `M::pure(env_value_of_type_R)` is called.
    /// Specifically, it's `<M as Applicative<R>>::Applicative<R>`.
    type SelfWithEnvAsValue = ReaderT<R, <M as Applicative<R>>::Applicative<R>, R>;

    /// Retrieves the current environment `R` by wrapping it with `M::pure`.
    ///
    /// The returned `ReaderT` will, when run with an environment `env`, produce
    /// `M::pure(env.clone())`.
    ///
    /// `R` must be `Clone + 'static` for this operation.
    fn ask() -> Self::SelfWithEnvAsValue
    where
        R: Clone + 'static, // `R` must be `Clone` to be `pure`d by `M`.
    {
        ReaderT::new(move |env: R| {
            // `M::pure` comes from `M`'s `Applicative<R>` implementation.
            // It takes `R` and returns `<M as Applicative<R>>::Applicative<R>`.
            // e.g., if `M` is `Option<_>` and `R` is `String`, `M::pure` takes `String`, returns `Option<String>`.
            M::pure(env.clone())
        })
    }

    /// Executes `computation` with an environment modified by `map_env_fn`.
    ///
    /// `map_env_fn` must be `Fn(R) -> R + 'static`.
    /// The `computation` is the `ReaderT<R, M, A>` to run.
    ///
    /// A new `ReaderT` is returned. When this new `ReaderT` is run with an
    /// `current_env: R`, it will first compute `modified_env = map_env_fn(current_env)`,
    /// and then run the original `computation`'s `run_reader_t` function with this
    /// `modified_env`.
    fn local<FMapEnv>(
        map_env_fn: FMapEnv,
        computation: Self,
    ) -> Self
    where
        FMapEnv: Fn(R) -> R + 'static,
    {
        let computation_run = computation.run_reader_t.clone();
        ReaderT::new(move |current_env: R| {
            let modified_env = map_env_fn(current_env);
            computation_run(modified_env) // Run original computation with the modified environment.
        })
    }
}
