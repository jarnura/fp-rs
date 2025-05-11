#[cfg(not(feature = "legacy"))]
// Imports adjusted for the new location
use monadify::{ReaderT, ReaderTHKTMarker, MonadReader}; // ReaderT, ReaderTHKTMarker, MonadReader (hkt version)
use monadify::functor::hkt::Functor;
use monadify::applicative::hkt::Applicative;
use monadify::monad::hkt::{Bind, Monad};
use monadify::identity::hkt::{Identity, IdentityHKTMarker};
use monadify::OptionHKTMarker; // Added import
// use fp_rs::kind_based::kind::OptionHKTMarker; // Removed unused import
// CFn is not directly used in these HKT tests for ReaderT, but good to have if needed for other HKT types
// use fp_rs::function::CFn;

#[derive(Clone, Debug, PartialEq)]
struct EnvConfig { pub val: i32 }

type TestReader<A> = ReaderT<EnvConfig, IdentityHKTMarker, A>;
type TestReaderHKT = ReaderTHKTMarker<EnvConfig, IdentityHKTMarker>;

#[test]
fn test_reader_t_hkt_functor_map() {
    let reader: TestReader<i32> = ReaderT::new(|cfg: EnvConfig| Identity(cfg.val + 1));
    let mapped_reader: TestReader<i32> = TestReaderHKT::map(reader, |x| x * 2);
    let result = (mapped_reader.run_reader_t)(EnvConfig { val: 10 });
    assert_eq!(result, Identity(22));
}

#[test]
fn test_reader_t_hkt_applicative_pure() {
    let pure_reader: TestReader<i32> = TestReaderHKT::pure(100);
    let result = (pure_reader.run_reader_t)(EnvConfig { val: 0 });
    assert_eq!(result, Identity(100));
}

#[test]
fn test_reader_t_hkt_monad_bind() {
    let reader1: TestReader<i32> = ReaderT::new(|cfg: EnvConfig| Identity(cfg.val));
    let f = |x: i32| -> TestReader<i32> {
        ReaderT::new(move |cfg: EnvConfig| Identity(x + cfg.val + 5))
    };
    let bound_reader: TestReader<i32> = TestReaderHKT::bind(reader1, f);
    let result = (bound_reader.run_reader_t)(EnvConfig { val: 3 });
    assert_eq!(result, Identity(11));
}

#[test]
fn test_monad_reader_hkt_ask() {
    let ask_reader: ReaderT<EnvConfig, IdentityHKTMarker, EnvConfig> =
        <TestReaderHKT as MonadReader<EnvConfig, EnvConfig, IdentityHKTMarker>>::ask();
    let result = (ask_reader.run_reader_t)(EnvConfig { val: 7 });
    assert_eq!(result, Identity(EnvConfig{val: 7}));
}

#[test]
fn test_monad_reader_hkt_local() {
    let reader: TestReader<i32> = ReaderT::new(|cfg: EnvConfig| Identity(cfg.val * 10));
    let modified_reader: TestReader<i32> = TestReaderHKT::local(|mut cfg: EnvConfig| { cfg.val +=1; cfg }, reader);
    let result = (modified_reader.run_reader_t)(EnvConfig{val: 2});
    assert_eq!(result, Identity(30));
}

// Helper to run reader and extract value for simple Identity case
fn run_test_reader<A: PartialEq + std::fmt::Debug>(
    reader: TestReader<A>,
    env: EnvConfig,
) -> A {
    (reader.run_reader_t)(env).0 // Assuming Identity.0 to get the value
}

// Helper to compare two TestReaders by running them with the same environment
fn assert_readers_eq<A: PartialEq + std::fmt::Debug + Clone>(
    reader1: TestReader<A>,
    reader2: TestReader<A>,
    env: EnvConfig,
) {
    let val1 = (reader1.run_reader_t)(env.clone()).0;
    let val2 = (reader2.run_reader_t)(env).0;
    assert_eq!(val1, val2);
}

#[test]
fn test_reader_t_hkt_monad_join() {
    let env = EnvConfig { val: 10 };
    let mma_reader: TestReader<TestReader<i32>> =
        TestReaderHKT::pure(TestReaderHKT::pure(100));

    let joined_reader: TestReader<i32> = TestReaderHKT::join(mma_reader);
    assert_eq!(run_test_reader(joined_reader, env.clone()), 100);

    let mma_reader_env_dependent: TestReader<TestReader<i32>> =
        ReaderT::new(move |cfg1: EnvConfig| {
            Identity(ReaderT::new(move |cfg2: EnvConfig| {
                Identity(cfg1.val + cfg2.val)
            }))
        });

    let joined_reader_env: TestReader<i32> = TestReaderHKT::join(mma_reader_env_dependent);
    assert_eq!(run_test_reader(joined_reader_env, env.clone()), env.val + env.val);
}

// Monad Laws for ReaderTHKTMarker<EnvConfig, IdentityHKTMarker>
#[test]
fn test_reader_t_hkt_monad_law_left_identity() {
    let env = EnvConfig { val: 5 };
    let a = 10;
    let f = |x: i32| -> TestReader<String> {
        ReaderT::new(move |cfg: EnvConfig| Identity((x + cfg.val).to_string()))
    };

    let lhs: TestReader<String> = TestReaderHKT::bind(TestReaderHKT::pure(a), f.clone());
    let rhs: TestReader<String> = f(a);

    assert_readers_eq(lhs, rhs, env.clone());
    let expected_val_rhs = ((f(a)).run_reader_t)(env).0;
    assert_eq!(expected_val_rhs, "15");
}

#[test]
fn test_reader_t_hkt_monad_law_right_identity() {
    let env = EnvConfig { val: 7 };
    let m_orig: TestReader<i32> = ReaderT::new(move |cfg: EnvConfig| Identity(cfg.val * 2));

    let pure_fn = |x: i32| TestReaderHKT::pure(x);

    let lhs: TestReader<i32> = TestReaderHKT::bind(m_orig.clone(), pure_fn);
    let rhs: TestReader<i32> = m_orig.clone();

    assert_readers_eq(lhs, rhs, env.clone());
    let val_m_orig = (m_orig.run_reader_t)(env).0;
    assert_eq!(val_m_orig, 14);
}

#[test]
fn test_reader_t_hkt_monad_law_associativity() {
    let env = EnvConfig { val: 3 };
    let m_orig: TestReader<i32> = ReaderT::new(move |cfg: EnvConfig| Identity(cfg.val + 1));

    let f = |x: i32| -> TestReader<i32> {
        ReaderT::new(move |cfg: EnvConfig| Identity(x * cfg.val))
    };
    let g = |y: i32| -> TestReader<String> {
        ReaderT::new(move |cfg: EnvConfig| Identity((y + cfg.val).to_string()))
    };

    let lhs: TestReader<String> = TestReaderHKT::bind(TestReaderHKT::bind(m_orig.clone(), f.clone()), g.clone());

    let f_clone_for_rhs = f.clone();
    let g_clone_for_rhs = g.clone();
    let composed_func = move |x_val: i32| -> TestReader<String> {
        TestReaderHKT::bind(f_clone_for_rhs(x_val), g_clone_for_rhs.clone())
    };
    let rhs: TestReader<String> = TestReaderHKT::bind(m_orig.clone(), composed_func);

    assert_readers_eq(lhs.clone(), rhs.clone(), env.clone());
    let val_lhs = (lhs.run_reader_t)(env).0;
    assert_eq!(val_lhs, "15");
}

// Helper to run ReaderT with Identity inner monad and compare
fn run_hkt_reader_t_identity<R: Clone + 'static, A: PartialEq + std::fmt::Debug + Clone + 'static>(
    reader: ReaderT<R, IdentityHKTMarker, A>,
    env: R,
) -> Identity<A> {
    // Assuming ReaderT is Clone because it uses Rc internally
    (reader.run_reader_t)(env)
}

// Helper to run ReaderT with Option inner monad and compare
fn run_hkt_reader_t_option<R: Clone + 'static, A: PartialEq + std::fmt::Debug + Clone + 'static>(
    reader: ReaderT<R, OptionHKTMarker, A>,
    env: R,
) -> Option<A> {
    (reader.run_reader_t)(env)
}

#[test]
fn test_hkt_reader_t_monad_laws_option_inner() {
    type Env = i32;
    type A = i32;
    type B = String;
    type C = usize;

    let env_val: Env = 10;
    let a_val: A = 5;

    // f_opt: A -> ReaderT<Env, OptionHKTMarker, B>
    let f_opt = move |x: A| -> ReaderT<Env, OptionHKTMarker, B> {
        ReaderT::new(move |env: Env| -> Option<B> {
            if x > 0 && env > 0 {
                Some(format!("f_opt: val={}, env={}", x, env))
            } else {
                None
            }
        })
    };

    // g_opt: B -> ReaderT<Env, OptionHKTMarker, C>
    let g_opt = move |s: B| -> ReaderT<Env, OptionHKTMarker, C> {
        ReaderT::new(move |env: Env| -> Option<C> {
            if !s.is_empty() && env > 0 {
                Some(s.len() + env as usize)
            } else {
                None
            }
        })
    };

    // m_opt_some: ReaderT<Env, OptionHKTMarker, A> (yields Some)
    let m_opt_some: ReaderT<Env, OptionHKTMarker, A> =
        ReaderT::new(move |env: Env| if env > 0 { Some(a_val + env) } else { None });

    // m_opt_none: ReaderT<Env, OptionHKTMarker, A> (yields None)
    let m_opt_none: ReaderT<Env, OptionHKTMarker, A> =
        ReaderT::new(move |_env: Env| None::<A>);

    // --- Test with m_opt_some ---
    // Law 1: pure(a).bind(f) == f(a)
    let pure_a_opt: ReaderT<Env, OptionHKTMarker, A> =
        ReaderT::<Env, OptionHKTMarker, A>::pure(a_val);
    let left_law1_opt = pure_a_opt.bind(f_opt);
    let right_law1_opt = f_opt(a_val);
    assert_eq!(
        run_hkt_reader_t_option(left_law1_opt.clone(), env_val),
        run_hkt_reader_t_option(right_law1_opt.clone(), env_val)
    );

    // Law 2: m.bind(pure) == m
    let left_law2_opt = m_opt_some
        .clone()
        .bind(|x: A| ReaderT::<Env, OptionHKTMarker, A>::pure(x));
    let right_law2_opt = m_opt_some.clone();
    assert_eq!(
        run_hkt_reader_t_option(left_law2_opt, env_val),
        run_hkt_reader_t_option(right_law2_opt, env_val)
    );

    // Law 3: m.bind(f).bind(g) == m.bind(|x| f(x).bind(g))
    let left_law3_opt = m_opt_some.clone().bind(f_opt).bind(g_opt);
    let right_law3_opt = m_opt_some.bind(move |x: A| f_opt(x).bind(g_opt));
    assert_eq!(
        run_hkt_reader_t_option(left_law3_opt.clone(), env_val),
        run_hkt_reader_t_option(right_law3_opt.clone(), env_val)
    );

    // --- Test with m_opt_none (should propagate None) ---
    // Law 1: Test f_opt with a value that makes it None (doesn't involve m_opt_none directly for this form)
    let val_for_f_none: A = -1;
    let pure_val_for_f_none: ReaderT<Env, OptionHKTMarker, A> =
        ReaderT::<Env, OptionHKTMarker, A>::pure(val_for_f_none);
    let left_law1_f_none = pure_val_for_f_none.bind(f_opt);
    let right_law1_f_none = f_opt(val_for_f_none);
     assert_eq!(
        run_hkt_reader_t_option(left_law1_f_none.clone(), env_val),
        run_hkt_reader_t_option(right_law1_f_none.clone(), env_val)
    );
    assert_eq!(run_hkt_reader_t_option(right_law1_f_none.clone(), env_val), None);


    // Law 2: m.bind(pure) == m (with m_opt_none)
    let left_law2_none = m_opt_none
        .clone()
        .bind(|x: A| ReaderT::<Env, OptionHKTMarker, A>::pure(x));
    let right_law2_none = m_opt_none.clone();
    assert_eq!(
        run_hkt_reader_t_option(left_law2_none.clone(), env_val),
        run_hkt_reader_t_option(right_law2_none.clone(), env_val)
    );
    assert_eq!(run_hkt_reader_t_option(right_law2_none.clone(), env_val), None);

    // Law 3: m.bind(f).bind(g) == m.bind(|x| f(x).bind(g)) (with m_opt_none)
    let left_law3_none = m_opt_none.clone().bind(f_opt).bind(g_opt);
    let right_law3_none = m_opt_none.bind(move |x: A| f_opt(x).bind(g_opt));
    assert_eq!(
        run_hkt_reader_t_option(left_law3_none.clone(), env_val),
        run_hkt_reader_t_option(right_law3_none.clone(), env_val)
    );
    assert_eq!(run_hkt_reader_t_option(right_law3_none.clone(), env_val), None);
    }
