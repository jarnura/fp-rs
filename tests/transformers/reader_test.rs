// Integration tests for ReaderT

// Import necessary items from the fp_rs crate
use fp_rs::transformers::reader::{ReaderT, Reader}; // Assuming Reader is also pub used
use fp_rs::identity::Identity;
use fp_rs::applicative::Applicative; // For pure in tests
use fp_rs::monad::Monad; // For MonadReader trait
use fp_rs::transformers::reader::MonadReader; // Specific trait

// Note: If CFn is used directly in tests and not re-exported at crate::function,
// this might need to be crate::function::CFn
// Assuming it's re-exported or made public at a higher level if needed by tests.
// For now, let's assume it's accessible via crate::function::CFn if test_reader_t_apply needs it.

#[test]
fn test_reader_t_new_and_run() {
    // R = i32, M = Option<String>, A = String
    let reader_fn = |env: i32| -> Option<String> {
        if env > 0 {
            Some(format!("Env: {}", env))
        } else {
            None
        }
    };

    let reader_t: ReaderT<i32, Option<String>, String> = ReaderT::new(reader_fn);

    assert_eq!((reader_t.run_reader_t)(10), Some(String::from("Env: 10")));
    assert_eq!((reader_t.run_reader_t)(-5), None);
}

#[test]
fn test_simple_reader_type_alias() {
    // R = String, A = usize
    let simple_reader_fn = |env: String| -> Identity<usize> {
        Identity(env.len())
    };
    
    let simple_reader: Reader<String, usize> = ReaderT::new(simple_reader_fn);

    let result_identity = (simple_reader.run_reader_t)(String::from("hello"));
    assert_eq!(result_identity.0, 5);
}

#[test]
fn test_reader_t_functor_map() {
    // R = i32, M = Option<String>, A = String
    // We want to map String -> usize (its length)
    // So, B = usize
    // Resulting M should be Option<usize>
    // Resulting ReaderT should be ReaderT<i32, Option<usize>, usize>

    let reader_fn = |env: i32| -> Option<String> {
        if env > 0 { Some(format!("Env: {}", env)) } else { None }
    };
    let reader_t: ReaderT<i32, Option<String>, String> = ReaderT::new(reader_fn);

    let map_fn = |s: String| s.len();
    let mapped_reader_t: ReaderT<i32, Option<usize>, usize> = reader_t.map(map_fn);

    assert_eq!((mapped_reader_t.run_reader_t)(10), Some(7)); // "Env: 10".len() = 7
    assert_eq!((mapped_reader_t.run_reader_t)(-5), None);

    // Test with Identity
    // R = &str, M = Identity<i32>, A = i32
    // Map i32 -> i32 (e.g. x * x)
    let id_reader_fn = |env_str: &'static str| -> Identity<i32> {
        Identity(if env_str == "ok" { 10 } else { 0 })
    };
    let id_reader_t: ReaderT<&'static str, Identity<i32>, i32> = ReaderT::new(id_reader_fn);
    
    let square_fn = |x: i32| x * x;
    let mapped_id_reader_t: ReaderT<&'static str, Identity<i32>, i32> = id_reader_t.map(square_fn);

    assert_eq!((mapped_id_reader_t.run_reader_t)("ok").0, 100);
    assert_eq!((mapped_id_reader_t.run_reader_t)("other").0, 0);
}

#[test]
fn test_reader_t_apply() {
    use fp_rs::function::CFn; // Assuming CFn is needed and accessible

    // R = i32 (env)
    // A = i32 (value in reader_val's Option)
    // B = String (final value type in result's Option)
    // M = Option

    // reader_val: ReaderT<i32, Option<i32>, i32>
    let reader_val_fn = |env: i32| -> Option<i32> {
        if env > 10 { Some(env + 5) } else { None } // e.g., env=15 -> Some(20)
    };
    let reader_val: ReaderT<i32, Option<i32>, i32> = ReaderT::new(reader_val_fn);

    // reader_fn: ReaderT<i32, Option<CFn<i32, String>>, CFn<i32, String>>
    let reader_func_fn = |env: i32| -> Option<CFn<i32, String>> {
        if env > 0 {
            Some(CFn::new(move |x: i32| format!("Env: {}, Val: {}", env, x)))
        } else {
            None
        }
    };
    let reader_func: ReaderT<i32, Option<CFn<i32, String>>, CFn<i32, String>> = ReaderT::new(reader_func_fn);
    
    let result_reader: ReaderT<i32, Option<String>, String> = reader_val.apply(reader_func);

    assert_eq!((result_reader.run_reader_t)(15), Some(String::from("Env: 15, Val: 20")));
    assert_eq!((result_reader.run_reader_t)(5), None);

    let reader_val_always_some_fn = |_env: i32| -> Option<i32> { Some(100) };
    let reader_val_always_some: ReaderT<i32, Option<i32>, i32> = ReaderT::new(reader_val_always_some_fn);

    let reader_func_fn_env_neg = |env: i32| -> Option<CFn<i32, String>> {
         if env > 0 { Some(CFn::new(move |x: i32| format!("Env: {}, Val: {}", env, x))) } else { None }
    };
    let reader_func_env_neg: ReaderT<i32, Option<CFn<i32, String>>, CFn<i32, String>> = ReaderT::new(reader_func_fn_env_neg);

    let result_reader_2: ReaderT<i32, Option<String>, String> = reader_val_always_some.apply(reader_func_env_neg);
    assert_eq!((result_reader_2.run_reader_t)(-5), None);
}

#[test]
fn test_reader_t_applicative_pure() {
    // R = String, A = i32, M = Option
    let pure_reader: ReaderT<String, Option<i32>, i32> = 
        <ReaderT<String, Option<i32>, i32> as Applicative<i32>>::pure(10);

    assert_eq!((pure_reader.run_reader_t)(String::from("any env")), Some(10));
    
    // R = i32, A = &str, M = Identity
    let pure_reader_id: ReaderT<i32, Identity<&'static str>, &'static str> =
        <ReaderT<i32, Identity<&'static str>, &'static str> as Applicative<&'static str>>::pure("hello");
    
    assert_eq!((pure_reader_id.run_reader_t)(123).0, "hello");
}

#[test]
fn test_reader_t_monad_bind() {
    // R = String (env)
    // A = usize (value from first reader's Identity)
    // B = String (final value type in result's Identity)
    // M = Identity

    let reader1_fn = |env: String| -> Identity<usize> {
        Identity(env.len())
    };
    let reader1: ReaderT<String, Identity<usize>, usize> = ReaderT::new(reader1_fn);

    let bind_fn = |len: usize| -> ReaderT<String, Identity<String>, String> {
        ReaderT::new(Box::new(move |env2: String| -> Identity<String> {
            Identity(format!("Env: '{}', Len: {}, Doubled: {}", env2, len, len * 2))
        }))
    };

    let result_reader = reader1.bind(bind_fn);
    let final_result = (result_reader.run_reader_t)(String::from("hello"));
    assert_eq!(final_result.0, "Env: 'hello', Len: 5, Doubled: 10");

    // Test with Option
    let reader_opt1_fn = |env: i32| if env > 0 { Some(env * 2) } else { None };
    let reader_opt1: ReaderT<i32, Option<i32>, i32> = ReaderT::new(reader_opt1_fn);

    let bind_opt_fn = |val_a: i32| -> ReaderT<i32, Option<i32>, i32> {
        ReaderT::new(Box::new(move |env2: i32| {
            if val_a > env2 { Some(val_a + env2) } else { None }
        }))
    };
    
    let result_opt_reader = reader_opt1.bind(bind_opt_fn);
    assert_eq!((result_opt_reader.run_reader_t)(10), Some(30));
    
    let reader_opt1_for_none_case: ReaderT<i32, Option<i32>, i32> = ReaderT::new(reader_opt1_fn);
    assert_eq!((reader_opt1_for_none_case.run_reader_t)(-5), None);

    let bind_opt_fn_can_be_none = |val_a: i32| -> ReaderT<i32, Option<i32>, i32> {
        ReaderT::new(Box::new(move |env2: i32| {
            if val_a < env2 { Some(val_a - env2) } else { None }
        }))
    };
    let reader_opt1_again: ReaderT<i32, Option<i32>, i32> = ReaderT::new(reader_opt1_fn);
    let result_opt_reader_2 = reader_opt1_again.bind(bind_opt_fn_can_be_none);
    assert_eq!((result_opt_reader_2.run_reader_t)(10), None);
}

#[test]
fn test_reader_t_monad_reader_ask() {
    let asked_reader: ReaderT<String, Option<String>, String> =
        <ReaderT<String, Option<String>, String> as MonadReader<String, String>>::ask();

    let env1 = String::from("test_env1");
    assert_eq!((asked_reader.run_reader_t)(env1.clone()), Some(env1));

    let asked_reader_id: ReaderT<i32, Identity<i32>, i32> =
        <ReaderT<i32, Identity<i32>, i32> as MonadReader<i32, i32>>::ask();
    
    assert_eq!((asked_reader_id.run_reader_t)(123).0, 123);
}

#[test]
fn test_reader_t_monad_reader_local() {
    let initial_reader_fn = |env: i32| -> Option<i32> { Some(env * 2) };
    let initial_reader: ReaderT<i32, Option<i32>, i32> = ReaderT::new(initial_reader_fn);

    let map_env_fn = |env: i32| env + 10;

    let local_reader =
        <ReaderT<i32, Option<i32>, i32> as MonadReader<i32, i32>>::local(map_env_fn, initial_reader);

    assert_eq!((local_reader.run_reader_t)(5), Some(30));
}

#[cfg(feature = "kind")]
mod hkt_tests {
    use fp_rs::identity::Identity;
    use fp_rs::kind_based::markers::{IdentityHKTMarker, OptionHKTMarker};
    use fp_rs::transformers::reader::hkt::ReaderT;
    // HKT traits
    use fp_rs::applicative::hkt::Applicative;
    use fp_rs::monad::hkt::{Bind, Monad}; // Monad trait itself might not be used directly in laws if bind/pure are sufficient

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
    fn test_hkt_reader_t_monad_laws_identity_inner() {
        type Env = i32;
        type A = i32;
        type B = String;
        type C = usize;

        let env_val: Env = 10;
        let a_val: A = 5;

        // f: A -> ReaderT<Env, IdentityHKTMarker, B>
        let f = move |x: A| -> ReaderT<Env, IdentityHKTMarker, B> {
            ReaderT::new(move |env: Env| -> Identity<B> {
                Identity(format!("f: val={}, env={}", x, env))
            })
        };

        // g: B -> ReaderT<Env, IdentityHKTMarker, C>
        let g = move |s: B| -> ReaderT<Env, IdentityHKTMarker, C> {
            ReaderT::new(move |env: Env| -> Identity<C> {
                Identity(s.len() + env as usize)
            })
        };

        // m: ReaderT<Env, IdentityHKTMarker, A>
        let m: ReaderT<Env, IdentityHKTMarker, A> =
            ReaderT::new(move |env: Env| Identity(a_val + env));

        // Law 1: pure(a).bind(f) == f(a)
        let pure_a: ReaderT<Env, IdentityHKTMarker, A> =
            ReaderT::<Env, IdentityHKTMarker, A>::pure(a_val);
        let left_law1 = pure_a.bind(f);
        let right_law1 = f(a_val);
        assert_eq!(
            run_hkt_reader_t_identity(left_law1.clone(), env_val),
            run_hkt_reader_t_identity(right_law1.clone(), env_val)
        );

        // Law 2: m.bind(pure) == m
        let left_law2 = m
            .clone()
            .bind(|x: A| ReaderT::<Env, IdentityHKTMarker, A>::pure(x));
        let right_law2 = m.clone();
        assert_eq!(
            run_hkt_reader_t_identity(left_law2, env_val),
            run_hkt_reader_t_identity(right_law2, env_val)
        );

        // Law 3: m.bind(f).bind(g) == m.bind(|x| f(x).bind(g))
        let left_law3 = m.clone().bind(f).bind(g);
        let right_law3 = m.bind(move |x: A| f(x).bind(g)); // f, g are Fn, so they are Copy/Clone if they capture Copy/Clone types
        assert_eq!(
            run_hkt_reader_t_identity(left_law3.clone(), env_val),
            run_hkt_reader_t_identity(right_law3.clone(), env_val)
        );
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
}
