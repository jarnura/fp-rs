#![cfg(all(test, feature = "kind"))]

// Imports adjusted for the new location
use fp_rs::transformers::reader::hkt::*; // ReaderT, ReaderTHKTMarker, MonadReader (hkt version)
use fp_rs::functor::hkt::Functor;
use fp_rs::applicative::hkt::Applicative;
use fp_rs::monad::hkt::{Bind, Monad};
use fp_rs::identity::hkt::{Identity, IdentityHKTMarker};
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
