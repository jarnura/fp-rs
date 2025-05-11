//! # ReaderT Monad Transformer
// HKT version is now default.

pub mod hkt {
    //! # Higher-Kinded Type (HKT) ReaderT Monad Transformer
    //!
    //! This module provides the HKT implementation of the `ReaderT` monad transformer.
    //! `ReaderT` (Reader Transformer) adds a read-only environment (of type `R`)
    //! to an underlying monad (represented by `MMarker`).
    //!
    //! Computations of type `ReaderT<R, MMarker, A>` are essentially functions
    //! of the form `R -> MMarker::Applied<A>`, where `MMarker::Applied<A>` is the
    //! actual inner monadic type (e.g., `Option<A>`, `Result<A, E>`).
    //!
    //! ## Key Components
    //! - [`ReaderT<R, MMarker, A>`]: The main struct representing a computation
    //!   that depends on an environment `R` and results in `MMarker::Applied<A>`.
    //! - [`ReaderTHKTMarker<R, MMarker>`]: The HKT marker for `ReaderT`.
    //! - [`MonadReader<REnv, AVal, MMarker>`]: A trait providing `ask` (to get the
    //!   environment) and `local` (to run a computation with a modified environment).
    //! - [`Reader<R, A>`]: A type alias for `ReaderT<R, IdentityHKTMarker, A>`,
    //!   representing a simple Reader monad (not a transformer).
    //!
    //! ## Example
    //! ```
    //! use monadify::transformers::reader::hkt::{ReaderT, ReaderTHKTMarker, MonadReader, Reader};
    //! use monadify::kind_based::kind::OptionHKTMarker; // OptionHKTMarker from kind_based::kind
    //! use monadify::IdentityHKTMarker; // IdentityHKTMarker re-exported from crate::identity
    //! use monadify::functor::hkt::Functor;
    //! use monadify::applicative::hkt::Applicative;
    //! use monadify::monad::hkt::{Bind, Monad};
    //! use std::rc::Rc;
    //!
    //! // Define a configuration environment
    //! #[derive(Clone)]
    //! struct Config {
    //!     greeting: String,
    //!     count: i32,
    //! }
    //!
    //! // Type alias for ReaderT with Option as inner monad and Config as environment
    //! type ConfigReaderOption<A> = ReaderT<Config, OptionHKTMarker, A>;
    //! type ConfigReaderOptionMarker = ReaderTHKTMarker<Config, OptionHKTMarker>;
    //!
    //! // 1. Using 'ask' to get the environment
    //! let ask_for_greeting_op: ConfigReaderOption<Config> = <ConfigReaderOptionMarker as MonadReader<Config, Config, OptionHKTMarker>>::ask();
    //! let ask_for_greeting: ConfigReaderOption<String> = <ConfigReaderOptionMarker as Functor<Config, String>>::map(
    //!     ask_for_greeting_op,
    //!     |config: Config| config.greeting
    //! );
    //! let config1 = Config { greeting: "Hello".to_string(), count: 5 };
    //! assert_eq!((ask_for_greeting.run_reader_t)(config1.clone()), Some("Hello".to_string()));
    //!
    //! // 2. Using 'pure' and 'map'
    //! let pure_val: ConfigReaderOption<i32> = ConfigReaderOptionMarker::pure(10);
    //! let mapped_val: ConfigReaderOption<i32> = <ConfigReaderOptionMarker as Functor<i32, i32>>::map(pure_val, |x| x * 2);
    //! assert_eq!((mapped_val.run_reader_t)(config1.clone()), Some(20));
    //!
    //! // 3. Using 'bind'
    //! let ask_op_for_bind: ConfigReaderOption<Config> = <ConfigReaderOptionMarker as MonadReader<Config, Config, OptionHKTMarker>>::ask();
    //! let computation: ConfigReaderOption<String> = ConfigReaderOptionMarker::bind(
    //!     ask_op_for_bind, // Gets Config
    //!     |config: Config| {
    //!         if config.count > 3 {
    //!             ConfigReaderOptionMarker::pure(format!("{} times {}", config.greeting, config.count))
    //!         } else {
    //!             ReaderT::new(|_cfg| None) // Create a None in the context
    //!         }
    //!     }
    //! );
    //! assert_eq!(
    //!     (computation.run_reader_t)(config1.clone()),
    //!     Some("Hello times 5".to_string())
    //! );
    //!
    //! // 4. Using 'local' to modify the environment for a sub-computation
    //! let local_computation: ConfigReaderOption<String> = ConfigReaderOptionMarker::local(
    //!     |mut cfg: Config| { cfg.greeting = "Hola".to_string(); cfg.count = 2; cfg },
    //!     computation.clone() // The computation from step 3
    //! );
    //! // The original computation would yield Some("Hello times 5") with config1.
    //! // The local_computation modifies env, so inner computation sees count=2, resulting in None.
    //! assert_eq!((local_computation.run_reader_t)(config1.clone()), None);
    //!
    //! // 5. Using 'join' (example with Reader<R, Reader<R, A>>)
    //! type SimpleReader<A> = Reader<Config, A>; // ReaderT<Config, IdentityHKTMarker, A>
    //! type SimpleReaderMarker = ReaderTHKTMarker<Config, IdentityHKTMarker>;
    //! use monadify::HKT; // For HKT::Applied
    //!
    //! let val_in_id: <IdentityHKTMarker as HKT>::Applied<SimpleReader<String>> =
    //!     IdentityHKTMarker::pure(ReaderT::new(|cfg: Config| IdentityHKTMarker::pure(cfg.greeting)));
    //!
    //! let nested_reader: SimpleReader<SimpleReader<String>> =
    //!     ReaderT::new(move |_cfg: Config| val_in_id.clone());
    //!
    //! let joined_reader: SimpleReader<String> = SimpleReaderMarker::join(nested_reader);
    //! assert_eq!((joined_reader.run_reader_t)(config1.clone()), IdentityHKTMarker::pure("Hello".to_string()));
    //! ```

    use std::marker::PhantomData;
    use std::rc::Rc;
    use crate::kind_based::kind::{HKT, HKT1};
    use crate::functor::hkt as functor_hkt;
    use crate::apply::hkt as apply_hkt;
    // Ensure applicative_hkt is available for MonadReader impl
    use crate::applicative::hkt as applicative_hkt; 
    use crate::monad::hkt as monad_hkt;
    use crate::function::CFn; // For Apply's function container type
    use crate::identity::hkt::IdentityHKTMarker; // HKT Identity for Reader alias

    /// The `ReaderT` monad transformer for HKTs.
    ///
    /// `ReaderT<R, MMarker, A>` represents a computation that:
    /// 1. Takes an environment of type `R`.
    /// 2. Produces a value of type `A` wrapped in an inner monad `MMarker`.
    ///
    /// The actual computation is stored in `run_reader_t`, which is a function
    /// `R -> MMarker::Applied<A>`. `MMarker::Applied<A>` is the concrete type
    /// of the inner monad (e.g., `Option<A>`, `Result<A, E>`).
    ///
    /// # Type Parameters
    /// - `R`: The type of the read-only environment.
    /// - `MMarker`: The HKT marker for the inner monad (e.g., [`crate::kind_based::kind::OptionHKTMarker`]).
    ///   It must implement [`HKT1`].
    /// - `A`: The type of the value produced by the computation within the inner monad.
    #[derive(Clone)]
    pub struct ReaderT<R, MMarker: HKT1, A> {
        /// The core function that defines the `ReaderT` computation.
        /// It takes an environment `R` and returns the result wrapped in the inner monad `MMarker::Applied<A>`.
        pub run_reader_t: Rc<dyn Fn(R) -> MMarker::Applied<A> + 'static>,
        _phantom_r: PhantomData<R>,
        _phantom_m_marker: PhantomData<MMarker>,
        _phantom_a: PhantomData<A>,
    }

    impl<R, MMarker: HKT1, A> ReaderT<R, MMarker, A> {
        /// Creates a new `ReaderT` from a function `R -> MMarker::Applied<A>`.
        pub fn new<F>(f: F) -> Self
        where
            F: Fn(R) -> MMarker::Applied<A> + 'static,
        {
            ReaderT {
                run_reader_t: Rc::new(f),
                _phantom_r: PhantomData,
                _phantom_m_marker: PhantomData,
                _phantom_a: PhantomData,
            }
        }
    }

    /// The HKT marker for `ReaderT<R, MMarker, _>`.
    ///
    /// This struct is used to implement HKT traits like `Functor`, `Applicative`, `Monad`
    /// for the `ReaderT` type constructor.
    ///
    /// # Type Parameters
    /// - `R`: The environment type.
    /// - `MMarker`: The HKT marker for the inner monad.
    #[derive(Default)]
    pub struct ReaderTHKTMarker<R, MMarker: HKT1>(PhantomData<(R, MMarker)>);

    impl<R, MMarker: HKT1> HKT for ReaderTHKTMarker<R, MMarker> {
        type Applied<A> = ReaderT<R, MMarker, A>;
    }
    // HKT1 is implemented by the blanket impl in kind.rs for types that impl HKT.

    /// A type alias for `ReaderT` with [`IdentityHKTMarker`] as the inner monad.
    /// This represents a simple Reader monad (not a transformer).
    /// `Reader<R, A>` is a computation `R -> Identity<A>`.
    pub type Reader<R, A> = ReaderT<R, IdentityHKTMarker, A>;

    // --- HKT Trait Implementations for ReaderTHKTMarker ---

    impl<R, MMarker, A, B> functor_hkt::Functor<A, B> for ReaderTHKTMarker<R, MMarker>
    where
        R: Clone + 'static,
        MMarker: functor_hkt::Functor<A, B> + HKT1 + 'static, // Inner MMarker must be Functor
        A: 'static,
        B: 'static,
        MMarker::Applied<A>: 'static, // M<A>
        MMarker::Applied<B>: 'static, // M<B>
    {
        /// Maps a function `A -> B` over the value within the `ReaderT` context.
        /// The environment `R` is passed through. The mapping happens within the inner monad `MMarker`.
        fn map(input: ReaderT<R, MMarker, A>, func: impl FnMut(A) -> B + Clone + 'static) -> ReaderT<R, MMarker, B> {
            let run_reader_t_clone = input.run_reader_t.clone();
            ReaderT::new(move |env: R| {
                let m_val: MMarker::Applied<A> = run_reader_t_clone(env);
                MMarker::map(m_val, func.clone()) // Delegate to MMarker's map
            })
        }
    }

    impl<R, MMarker, A, B> apply_hkt::Apply<A, B> for ReaderTHKTMarker<R, MMarker>
    where
        R: Clone + 'static,
        MMarker: apply_hkt::Apply<A, B> + HKT1 + 'static, // Inner MMarker must be Apply
        A: 'static,
        B: 'static,
        MMarker::Applied<A>: 'static, // M<A>
        MMarker::Applied<B>: 'static, // M<B>
        MMarker::Applied<CFn<A, B>>: 'static, // M<CFn<A,B>>
    {
        /// Applies a wrapped function within `ReaderT` to a wrapped value within `ReaderT`.
        /// Both computations share the same environment `R`. The application happens within `MMarker`.
        fn apply(
            value_container: ReaderT<R, MMarker, A>,
            function_container: ReaderT<R, MMarker, CFn<A, B>>,
        ) -> ReaderT<R, MMarker, B> {
            let value_run = value_container.run_reader_t.clone();
            let function_run = function_container.run_reader_t.clone();
            ReaderT::new(move |env: R| {
                let m_val: MMarker::Applied<A> = value_run(env.clone());
                let m_func: MMarker::Applied<CFn<A, B>> = function_run(env);
                MMarker::apply(m_val, m_func) // Delegate to MMarker's apply
            })
        }
    }

    impl<R, MMarker, T> applicative_hkt::Applicative<T> for ReaderTHKTMarker<R, MMarker>
    where
        R: Clone + 'static, // Though _env is not used, new needs Fn(R)
        MMarker: applicative_hkt::Applicative<T> + HKT1 + 'static, // Inner MMarker must be Applicative
        T: Clone + 'static, // For MMarker::pure(value.clone())
        MMarker::Applied<T>: 'static, // M<T>
    {
        /// Lifts a value `T` into the `ReaderT` context.
        /// The resulting computation ignores the environment and returns `MMarker::pure(value)`.
        fn pure(value: T) -> ReaderT<R, MMarker, T> {
            ReaderT::new(move |_env: R| MMarker::pure(value.clone()))
        }
    }

    impl<R, MMarker, A, B> monad_hkt::Bind<A, B> for ReaderTHKTMarker<R, MMarker>
    where
        R: Clone + 'static,
        MMarker: monad_hkt::Bind<A, B> + HKT1 + 'static, // Inner MMarker must be Bind
        A: 'static,
        B: 'static,
        MMarker::Applied<A>: 'static, // M<A>
        MMarker::Applied<B>: 'static, // M<B>
    {
        /// Sequentially composes a `ReaderT` computation with a function that returns a new `ReaderT`.
        /// The environment `R` is passed to both the initial computation and the one produced by `func`.
        /// The `bind` operation itself is delegated to the inner monad `MMarker`.
        fn bind(input: ReaderT<R, MMarker, A>, func: impl FnMut(A) -> ReaderT<R, MMarker, B> + Clone + 'static) -> ReaderT<R, MMarker, B> {
            let self_run = input.run_reader_t.clone();
            ReaderT::new(move |env: R| {
                let m_a_val: MMarker::Applied<A> = self_run(env.clone());
                let mut f_clone = func.clone();
                // Delegate to MMarker's bind.
                // The function for MMarker::bind takes `A` and must return `MMarker::Applied<B>`.
                // `f_clone(a_val)` returns `ReaderT<R, MMarker, B>`.
                // We run this `ReaderT` with the current `env` to get `MMarker::Applied<B>`.
                MMarker::bind(m_a_val, move |a_val: A| {
                    let next_reader_t: ReaderT<R, MMarker, B> = f_clone(a_val);
                    (next_reader_t.run_reader_t)(env.clone())
                })
            })
        }
    }

    impl<R, MMarker, A> monad_hkt::Monad<A> for ReaderTHKTMarker<R, MMarker>
    where
        R: Clone + 'static,
        MMarker: applicative_hkt::Applicative<A> // For ReaderTHKTMarker's Monad<A> supertrait Applicative<A>
                 + monad_hkt::Bind<ReaderT<R, MMarker, A>, A> // For the join implementation
                 + HKT1
                 + 'static,
        A: Clone + 'static, // From Applicative<A> constraint on ReaderTHKTMarker
        MMarker::Applied<A>: 'static, // M<A>
        MMarker::Applied<ReaderT<R, MMarker, A>>: 'static, // M<ReaderT<R,M,A>>
    {
        /// Flattens a nested `ReaderT<R, MMarker, ReaderT<R, MMarker, A>>` into
        /// `ReaderT<R, MMarker, A>`.
        /// This is achieved by running the outer `ReaderT` to get `MMarker::Applied<ReaderT<R,MMarker,A>>`,
        /// then using `MMarker::bind` to run the inner `ReaderT` with the same environment.
        fn join(mma: ReaderT<R, MMarker, ReaderT<R, MMarker, A>>) -> ReaderT<R, MMarker, A> {
            ReaderT::new(move |env: R| {
                let m_reader_t_a: MMarker::Applied<ReaderT<R, MMarker, A>> = (mma.run_reader_t)(env.clone());

                let m_a: MMarker::Applied<A> =
                    <MMarker as monad_hkt::Bind<ReaderT<R, MMarker, A>, A>>::bind(
                        m_reader_t_a,
                        move |inner_reader_t: ReaderT<R, MMarker, A>| {
                            (inner_reader_t.run_reader_t)(env.clone())
                        },
                    );
                m_a
            })
        }
    }

    /// Trait for monads that can access a read-only environment `REnv`.
    ///
    /// # Type Parameters
    /// - `REnv`: The type of the environment.
    /// - `AVal`: The type of the value produced by computations in this monad.
    /// - `MMarker`: The HKT marker for the inner monad (if `Self` is a transformer like `ReaderT`).
    ///
    /// This HKT version is specific to `ReaderT`. A more general `MonadReader` might be
    /// generic over `Self` directly if `Self` is the HKT marker of the reader-like monad.
    pub trait MonadReader<REnv, AVal, MMarker: HKT1>
    where
        Self: Sized, // The HKT marker implementing this trait, e.g., ReaderTHKTMarker<REnv, MMarker>
    {
        /// Retrieves the environment `REnv` from the context.
        ///
        /// The result is wrapped in the `ReaderT` structure, e.g., `ReaderT<REnv, MMarker, REnv>`.
        ///
        /// # Example
        /// ```
        /// use monadify::transformers::reader::hkt::{ReaderT, ReaderTHKTMarker, MonadReader};
        /// use monadify::kind_based::kind::OptionHKTMarker;
        ///
        /// #[derive(Clone, PartialEq, Debug)]
        /// struct MyConfig { id: i32 }
        /// type ConfigReader<A> = ReaderT<MyConfig, OptionHKTMarker, A>;
        /// type ConfigReaderMarker = ReaderTHKTMarker<MyConfig, OptionHKTMarker>;
        ///
        /// // Specify the type REnv and AVal for ask, which are both MyConfig here.
        /// let get_config: ConfigReader<MyConfig> = <ConfigReaderMarker as MonadReader<MyConfig, MyConfig, OptionHKTMarker>>::ask();
        /// let env = MyConfig { id: 123 };
        /// assert_eq!((get_config.run_reader_t)(env.clone()), Some(env));
        /// ```
        fn ask() -> ReaderT<REnv, MMarker, REnv>
        where
            REnv: Clone + 'static,
            MMarker: applicative_hkt::Applicative<REnv> + 'static,
            MMarker::Applied<REnv>: 'static;

        /// Executes a computation in a modified environment.
        ///
        /// # Parameters
        /// - `map_env_fn`: A function `REnv -> REnv` that transforms the current environment.
        /// - `computation`: The `ReaderT` computation to run with the modified environment.
        ///
        /// # Example
        /// ```
        /// use monadify::transformers::reader::hkt::{ReaderT, ReaderTHKTMarker, MonadReader};
        /// use monadify::kind_based::kind::OptionHKTMarker;
        /// use monadify::applicative::hkt::Applicative; // For pure
        /// use monadify::functor::hkt::Functor; // For map
        ///
        /// #[derive(Clone, PartialEq, Debug)]
        /// struct MyConfig { prefix: String, value: i32 }
        /// type ConfigReader<A> = ReaderT<MyConfig, OptionHKTMarker, A>;
        /// type ConfigReaderMarker = ReaderTHKTMarker<MyConfig, OptionHKTMarker>;
        ///
        /// let initial_config = MyConfig { prefix: "Value: ".to_string(), value: 10 };
        ///
        /// let get_value_str_op: ConfigReader<MyConfig> = <ConfigReaderMarker as MonadReader<MyConfig, MyConfig, OptionHKTMarker>>::ask();
        /// let get_value_str: ConfigReader<String> = <ConfigReaderMarker as Functor<MyConfig, String>>::map(
        ///     get_value_str_op,
        ///     |cfg: MyConfig| format!("{}{}", cfg.prefix, cfg.value)
        /// );
        ///
        /// let modified_computation = ConfigReaderMarker::local(
        ///     |mut cfg: MyConfig| { cfg.prefix = "New Value: ".to_string(); cfg.value = 20; cfg },
        ///     get_value_str.clone()
        /// );
        ///
        /// assert_eq!((get_value_str.run_reader_t)(initial_config.clone()), Some("Value: 10".to_string()));
        /// assert_eq!((modified_computation.run_reader_t)(initial_config.clone()), Some("New Value: 20".to_string()));
        /// ```
        fn local<FMapEnv>(
            map_env_fn: FMapEnv,
            computation: ReaderT<REnv, MMarker, AVal>,
        ) -> ReaderT<REnv, MMarker, AVal>
        where
            REnv: 'static,
            AVal: 'static,
            MMarker: 'static,
            MMarker::Applied<AVal>: 'static,
            FMapEnv: Fn(REnv) -> REnv + 'static;
    }

    impl<R, MMarkerImpl, A> MonadReader<R, A, MMarkerImpl> for ReaderTHKTMarker<R, MMarkerImpl>
    where
        R: 'static,
        A: 'static,
        MMarkerImpl: HKT1 + 'static, // MMarkerImpl is the HKT marker for the inner monad
        MMarkerImpl::Applied<A>: 'static, // M<A>
    {
        fn ask() -> ReaderT<R, MMarkerImpl, R>
        where
            R: Clone + 'static,
            MMarkerImpl: applicative_hkt::Applicative<R> + 'static, // Changed to applicative_hkt
            MMarkerImpl::Applied<R>: 'static,
        {
            ReaderT::new(move |env: R| MMarkerImpl::pure(env.clone()))
        }

        fn local<FMapEnv>(
            map_env_fn: FMapEnv,
            computation: ReaderT<R, MMarkerImpl, A>,
        ) -> ReaderT<R, MMarkerImpl, A>
        where
            FMapEnv: Fn(R) -> R + 'static,
        {
            let computation_run = computation.run_reader_t.clone();
            ReaderT::new(move |current_env: R| {
                let modified_env = map_env_fn(current_env);
                computation_run(modified_env)
            })
        }
    }
}


// Directly export HKT versions
pub use hkt::{ReaderT, Reader, ReaderTHKTMarker, MonadReader};
