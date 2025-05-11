use monadify::monad::hkt::{Bind, Monad};
use monadify::applicative::hkt::Applicative;
use monadify::functor::hkt::Functor; // Import HKT Functor
use monadify::kind_based::kind::{ResultHKTMarker, OptionHKTMarker, VecHKTMarker, CFnHKTMarker, CFnOnceHKTMarker}; // Added CFnOnceHKTMarker
use monadify::function::{CFn, CFnOnce};


// Common error type for Result tests
type TestError = String;
type TestResult<T> = Result<T, TestError>;

// Helper for creating a cloneable FnMut for bind
fn clone_fn<A, B, F>(f: F) -> impl FnMut(A) -> B + Clone + 'static
where
    F: Fn(A) -> B + Clone + 'static,
    A: 'static,
    B: 'static,
{
    f
}

// Unused helper `once_fn` removed.

mod result_hkt_monad_laws {
    use super::*; // Imports Bind, Monad, Applicative, ResultHKTMarker, TestError, TestResult, clone_fn

    // 1. Left Identity: ResultHKTMarker::pure(a).bind(f) == f(a)
    #[test]
    fn result_hkt_monad_left_identity_ok() {
        let a: i32 = 10;
        let f = clone_fn(|x: i32| -> TestResult<String> { Ok((x * 2).to_string()) });

        let lhs = ResultHKTMarker::<TestError>::bind(
            ResultHKTMarker::<TestError>::pure(a),
            f.clone(),
        );
        let rhs = f.clone()(a); // Call the original f

        assert_eq!(lhs, rhs);
        assert_eq!(lhs, Ok("20".to_string()));
    }

    #[test]
    fn result_hkt_monad_left_identity_f_returns_err() {
        let a: i32 = 10;
        let f = clone_fn(|_x: i32| -> TestResult<String> { Err("f_error".to_string()) });

        let lhs = ResultHKTMarker::<TestError>::bind(
            ResultHKTMarker::<TestError>::pure(a),
            f.clone(),
        );
        let rhs = f.clone()(a);

        assert_eq!(lhs, rhs);
        assert_eq!(lhs, Err("f_error".to_string()));
    }

    // 2. Right Identity: m.bind(ResultHKTMarker::pure) == m
    #[test]
    fn result_hkt_monad_right_identity_ok() {
        let m: TestResult<i32> = Ok(10);
        let pure_fn = clone_fn(|val: i32| ResultHKTMarker::<TestError>::pure(val));

        let lhs = ResultHKTMarker::<TestError>::bind(m.clone(), pure_fn);
        let rhs = m;

        assert_eq!(lhs, rhs);
    }

    #[test]
    fn result_hkt_monad_right_identity_err() {
        let m: TestResult<i32> = Err("m_error".to_string());
        let pure_fn = clone_fn(|val: i32| ResultHKTMarker::<TestError>::pure(val));

        let lhs = ResultHKTMarker::<TestError>::bind(m.clone(), pure_fn);
        let rhs = m;

        assert_eq!(lhs, rhs);
    }

    // 3. Associativity: m.bind(f).bind(g) == m.bind(|x| f(x).bind(g))
    #[test]
    fn result_hkt_monad_associativity_all_ok() {
        let m: TestResult<i32> = Ok(10);
        let f = clone_fn(|x: i32| -> TestResult<f64> { Ok((x * 2) as f64) });
        let g = clone_fn(|y: f64| -> TestResult<String> { Ok(y.to_string()) });

        let lhs = ResultHKTMarker::<TestError>::bind(
            ResultHKTMarker::<TestError>::bind(m.clone(), f.clone()),
            g.clone(),
        );

        let f_inner = f.clone();
        let g_inner = g.clone();
        let composed_func = clone_fn(move |x: i32| {
            ResultHKTMarker::<TestError>::bind(f_inner.clone()(x), g_inner.clone())
        });
        let rhs = ResultHKTMarker::<TestError>::bind(m.clone(), composed_func);

        assert_eq!(lhs, rhs);
        assert_eq!(lhs, Ok("20".to_string()));
    }

    #[test]
    fn result_hkt_monad_associativity_m_is_err() {
        let m: TestResult<i32> = Err("m_error".to_string());
        let f = clone_fn(|x: i32| -> TestResult<f64> { Ok((x * 2) as f64) });
        let g = clone_fn(|y: f64| -> TestResult<String> { Ok(y.to_string()) });

        let lhs = ResultHKTMarker::<TestError>::bind(
            ResultHKTMarker::<TestError>::bind(m.clone(), f.clone()),
            g.clone(),
        );
        
        let f_inner = f.clone();
        let g_inner = g.clone();
        let composed_func = clone_fn(move |x: i32| {
            ResultHKTMarker::<TestError>::bind(f_inner.clone()(x), g_inner.clone())
        });
        let rhs = ResultHKTMarker::<TestError>::bind(m.clone(), composed_func);

        assert_eq!(lhs, rhs);
        assert_eq!(lhs, Err("m_error".to_string()));
    }

    #[test]
    fn result_hkt_monad_associativity_f_returns_err() {
        let m: TestResult<i32> = Ok(10);
        let f = clone_fn(|_x: i32| -> TestResult<f64> { Err("f_error".to_string()) });
        let g = clone_fn(|y: f64| -> TestResult<String> { Ok(y.to_string()) });

        let lhs = ResultHKTMarker::<TestError>::bind(
            ResultHKTMarker::<TestError>::bind(m.clone(), f.clone()),
            g.clone(),
        );

        let f_inner = f.clone();
        let g_inner = g.clone();
        let composed_func = clone_fn(move |x: i32| {
            ResultHKTMarker::<TestError>::bind(f_inner.clone()(x), g_inner.clone())
        });
        let rhs = ResultHKTMarker::<TestError>::bind(m.clone(), composed_func);
        
        assert_eq!(lhs, rhs);
        assert_eq!(lhs, Err("f_error".to_string()));
    }

    #[test]
    fn result_hkt_monad_associativity_g_returns_err() {
        let m: TestResult<i32> = Ok(10);
        let f = clone_fn(|x: i32| -> TestResult<f64> { Ok((x * 2) as f64) });
        let g = clone_fn(|_y: f64| -> TestResult<String> { Err("g_error".to_string()) });

        let lhs = ResultHKTMarker::<TestError>::bind(
            ResultHKTMarker::<TestError>::bind(m.clone(), f.clone()),
            g.clone(),
        );

        let f_inner = f.clone();
        let g_inner = g.clone();
        let composed_func = clone_fn(move |x: i32| {
            ResultHKTMarker::<TestError>::bind(f_inner.clone()(x), g_inner.clone())
        });
        let rhs = ResultHKTMarker::<TestError>::bind(m.clone(), composed_func);

        assert_eq!(lhs, rhs);
        assert_eq!(lhs, Err("g_error".to_string()));
    }

    // Monad::join laws for ResultHKTMarker
    // join(pure(pure(x))) == pure(x)
    // join(map(pure, m)) == m
    // join(pure(m)) == m

    #[test]
    fn result_hkt_monad_join_law1() { // join(pure(pure(x))) == pure(x)
        let x = 10;
        let mma: TestResult<TestResult<i32>> = 
            ResultHKTMarker::<TestError>::pure(ResultHKTMarker::<TestError>::pure(x));
        
        let lhs = ResultHKTMarker::<TestError>::join(mma);
        let rhs = ResultHKTMarker::<TestError>::pure(x);
        assert_eq!(lhs, rhs);
        assert_eq!(lhs, Ok(10));
    }

    #[test]
    fn result_hkt_monad_join_law1_outer_err() {
        let mma: TestResult<TestResult<i32>> = Err("outer_error".to_string());
        let lhs = ResultHKTMarker::<TestError>::join(mma);
        assert_eq!(lhs, Err("outer_error".to_string()));
    }

    #[test]
    fn result_hkt_monad_join_law1_inner_err() {
        let mma: TestResult<TestResult<i32>> = 
            ResultHKTMarker::<TestError>::pure(Err("inner_error".to_string()));
        let lhs = ResultHKTMarker::<TestError>::join(mma);
        assert_eq!(lhs, Err("inner_error".to_string()));
    }
    
    #[test]
    fn result_hkt_monad_join_law2() { // join(map(pure, m)) == m
        let m_ok: TestResult<i32> = Ok(10);
        let m_err: TestResult<i32> = Err("m_error".to_string());

        let pure_fn = clone_fn(|val: i32| ResultHKTMarker::<TestError>::pure(val));

        let mapped_m_ok = ResultHKTMarker::<TestError>::map(m_ok.clone(), pure_fn.clone());
        assert_eq!(ResultHKTMarker::<TestError>::join(mapped_m_ok), m_ok);

        let mapped_m_err = ResultHKTMarker::<TestError>::map(m_err.clone(), pure_fn.clone());
        assert_eq!(ResultHKTMarker::<TestError>::join(mapped_m_err), m_err);
    }

    #[test]
    fn result_hkt_monad_join_law3() { // join(pure(m)) == m
        let m_ok: TestResult<i32> = Ok(10);
        let m_err: TestResult<i32> = Err("m_error".to_string());

        let pure_m_ok = ResultHKTMarker::<TestError>::pure(m_ok.clone());
        assert_eq!(ResultHKTMarker::<TestError>::join(pure_m_ok), m_ok);
        
        let pure_m_err = ResultHKTMarker::<TestError>::pure(m_err.clone());
        // This case is interesting: pure(Err(e)) -> Ok(Err(e))
        // join(Ok(Err(e))) -> Err(e)
        assert_eq!(ResultHKTMarker::<TestError>::join(pure_m_err), m_err);
    }
}

// Example HKT join tests for Option (can be moved to a dedicated option_hkt_monad_laws module later)
mod option_hkt_join_tests {
    use super::*; // Imports Bind, Monad, Applicative, OptionHKTMarker etc.

    #[test]
    fn option_hkt_join_some_some() {
        let mma: Option<Option<i32>> = Some(Some(10));
        assert_eq!(OptionHKTMarker::join(mma), Some(10));
    }

    #[test]
    fn option_hkt_join_some_none() {
        let mma: Option<Option<i32>> = Some(None);
        assert_eq!(OptionHKTMarker::join(mma), None);
    }

    #[test]
    fn option_hkt_join_none() {
        let mma: Option<Option<i32>> = None;
        assert_eq!(OptionHKTMarker::join(mma), None);
    }
}

mod vec_hkt_monad_laws {
    use super::*; // Imports Bind, Monad, Applicative, VecHKTMarker, clone_fn

    // 1. Left Identity: VecHKTMarker::pure(a).bind(f) == f(a)
    #[test]
    fn vec_hkt_monad_left_identity() {
        let a: i32 = 10;
        // f: i32 -> Vec<String>
        let f = clone_fn(|x: i32| -> Vec<String> { vec![x.to_string(), (x + 1).to_string()] });

        let lhs = VecHKTMarker::bind(VecHKTMarker::pure(a), f.clone());
        let rhs = f.clone()(a);

        assert_eq!(lhs, rhs);
        assert_eq!(lhs, vec!["10".to_string(), "11".to_string()]);
    }

    #[test]
    fn vec_hkt_monad_left_identity_f_returns_empty() {
        let a: i32 = 10;
        let f = clone_fn(|_x: i32| -> Vec<String> { vec![] });

        let lhs = VecHKTMarker::bind(VecHKTMarker::pure(a), f.clone());
        let rhs = f.clone()(a);

        assert_eq!(lhs, rhs);
        assert_eq!(lhs, Vec::<String>::new());
    }

    // 2. Right Identity: m.bind(VecHKTMarker::pure) == m
    #[test]
    fn vec_hkt_monad_right_identity_non_empty() {
        let m: Vec<i32> = vec![10, 20];
        let pure_fn = clone_fn(|val: i32| VecHKTMarker::pure(val));

        let lhs = VecHKTMarker::bind(m.clone(), pure_fn);
        let rhs = m;

        assert_eq!(lhs, rhs);
    }

    #[test]
    fn vec_hkt_monad_right_identity_empty() {
        let m: Vec<i32> = vec![];
        let pure_fn = clone_fn(|val: i32| VecHKTMarker::pure(val));

        let lhs = VecHKTMarker::bind(m.clone(), pure_fn);
        let rhs = m;

        assert_eq!(lhs, rhs);
    }

    // 3. Associativity: m.bind(f).bind(g) == m.bind(|x| f(x).bind(g))
    #[test]
    fn vec_hkt_monad_associativity() {
        let m: Vec<i32> = vec![1, 2];
        let f = clone_fn(|x: i32| -> Vec<i32> { vec![x, x * 10] });
        let g = clone_fn(|y: i32| -> Vec<String> { vec![y.to_string(), (y + 1).to_string()] });

        let lhs = VecHKTMarker::bind(
            VecHKTMarker::bind(m.clone(), f.clone()),
            g.clone(),
        );

        let f_inner = f.clone();
        let g_inner = g.clone();
        let composed_func = clone_fn(move |x: i32| {
            VecHKTMarker::bind(f_inner.clone()(x), g_inner.clone())
        });
        let rhs = VecHKTMarker::bind(m.clone(), composed_func);

        assert_eq!(lhs, rhs);
        assert_eq!(
            lhs,
            vec![
                "1".to_string(), "2".to_string(), "10".to_string(), "11".to_string(),
                "2".to_string(), "3".to_string(), "20".to_string(), "21".to_string()
            ]
        );
    }
    
    #[test]
    fn vec_hkt_monad_associativity_empty_start() {
        let m: Vec<i32> = vec![];
        let f = clone_fn(|x: i32| -> Vec<i32> { vec![x, x * 10] });
        let g = clone_fn(|y: i32| -> Vec<String> { vec![y.to_string(), (y + 1).to_string()] });

        let lhs = VecHKTMarker::bind(VecHKTMarker::bind(m.clone(), f.clone()), g.clone());
        let f_inner = f.clone();
        let g_inner = g.clone();
        let composed_func = clone_fn(move |x: i32| VecHKTMarker::bind(f_inner.clone()(x), g_inner.clone()));
        let rhs = VecHKTMarker::bind(m.clone(), composed_func);
        
        assert_eq!(lhs, rhs);
        assert_eq!(lhs, Vec::<String>::new());
    }

    #[test]
    fn vec_hkt_monad_associativity_f_returns_empty() {
        let m: Vec<i32> = vec![1, 2];
        let f = clone_fn(|_x: i32| -> Vec<i32> { vec![] });
        let g = clone_fn(|y: i32| -> Vec<String> { vec![y.to_string(), (y + 1).to_string()] });

        let lhs = VecHKTMarker::bind(VecHKTMarker::bind(m.clone(), f.clone()), g.clone());
        let f_inner = f.clone();
        let g_inner = g.clone();
        let composed_func = clone_fn(move |x: i32| VecHKTMarker::bind(f_inner.clone()(x), g_inner.clone()));
        let rhs = VecHKTMarker::bind(m.clone(), composed_func);

        assert_eq!(lhs, rhs);
        assert_eq!(lhs, Vec::<String>::new());
    }

    #[test]
    fn vec_hkt_monad_associativity_g_returns_empty() {
        let m: Vec<i32> = vec![1, 2];
        let f = clone_fn(|x: i32| -> Vec<i32> { vec![x, x * 10] });
        let g = clone_fn(|_y: i32| -> Vec<String> { vec![] });

        let lhs = VecHKTMarker::bind(VecHKTMarker::bind(m.clone(), f.clone()), g.clone());
        let f_inner = f.clone();
        let g_inner = g.clone();
        let composed_func = clone_fn(move |x: i32| VecHKTMarker::bind(f_inner.clone()(x), g_inner.clone()));
        let rhs = VecHKTMarker::bind(m.clone(), composed_func);

        assert_eq!(lhs, rhs);
        assert_eq!(lhs, Vec::<String>::new());
    }

    // Monad::join laws for VecHKTMarker
    #[test]
    fn vec_hkt_monad_join_law1() { // join(pure(pure(x))) == pure(x)
        let x = 10; // T: Clone for pure
        let mma: Vec<Vec<i32>> = VecHKTMarker::pure(VecHKTMarker::pure(x)); // vec![vec![10]]
        
        let lhs = VecHKTMarker::join(mma); // vec![10]
        let rhs = VecHKTMarker::pure(x);   // vec![10]
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn vec_hkt_monad_join_law2() { // join(map(pure, m)) == m
        let m_non_empty: Vec<i32> = vec![10, 20];
        let m_empty: Vec<i32> = vec![];

        let pure_fn = clone_fn(|val: i32| VecHKTMarker::pure(val)); // val -> vec![val]

        // map(m_non_empty, pure_fn) -> vec![vec![10], vec![20]]
        let mapped_m_non_empty = VecHKTMarker::map(m_non_empty.clone(), pure_fn.clone());
        assert_eq!(VecHKTMarker::join(mapped_m_non_empty), m_non_empty);

        // map(m_empty, pure_fn) -> vec![]
        let mapped_m_empty = VecHKTMarker::map(m_empty.clone(), pure_fn.clone());
        assert_eq!(VecHKTMarker::join(mapped_m_empty), m_empty);
    }

    #[test]
    fn vec_hkt_monad_join_law3() { // join(pure(m)) == m
        let m_non_empty: Vec<i32> = vec![10, 20]; // T: Clone for pure
        let m_empty: Vec<i32> = vec![];         // T: Clone for pure

        // pure(m_non_empty) -> vec![vec![10, 20]]
        let pure_m_non_empty = VecHKTMarker::pure(m_non_empty.clone());
        assert_eq!(VecHKTMarker::join(pure_m_non_empty), m_non_empty);
        
        // pure(m_empty) -> vec![vec![]]
        let pure_m_empty = VecHKTMarker::pure(m_empty.clone());
        assert_eq!(VecHKTMarker::join(pure_m_empty), m_empty);
    }

    #[test]
    fn vec_hkt_join_specific_examples() {
        assert_eq!(VecHKTMarker::join(vec![vec![1, 2], vec![3, 4]]), vec![1, 2, 3, 4]);
        assert_eq!(VecHKTMarker::join(vec![vec![], vec![3, 4]]), vec![3, 4]);
        assert_eq!(VecHKTMarker::join(vec![vec![1, 2], vec![]]), vec![1, 2]);
        assert_eq!(VecHKTMarker::join(Vec::<Vec<i32>>::new()), Vec::<i32>::new()); // For join(vec![])
        assert_eq!(VecHKTMarker::join(vec![Vec::<i32>::new(), Vec::<i32>::new()]), Vec::<i32>::new()); // For join(vec![vec![], vec![]])
    }
}

mod cfn_hkt_monad_laws {
    use super::*; // Imports Bind, Monad, Applicative, CFnHKTMarker, CFn, clone_fn
    // Define a common environment type for these tests
    type Env = i32;

    // 1. Left Identity: CFnHKTMarker::pure(a).bind(f) == f(a)
    #[test]
    fn cfn_hkt_monad_left_identity() {
        let env_val: Env = 5;
        let a: i32 = 10; // Value to be pure'd, must be Clone for pure

        // f: i32 -> CFn<Env, String>
        let f = clone_fn(move |x: i32| -> CFn<Env, String> {
            CFn::new(move |env: Env| (x + env).to_string())
        });

        // lhs: CFnHKTMarker::pure(a).bind(f)
        // CFnHKTMarker::pure(a) -> CFn(|_env| a.clone())
        let pure_a_cfn: CFn<Env, i32> = CFnHKTMarker::<Env>::pure(a);
        let lhs_cfn: CFn<Env, String> = CFnHKTMarker::<Env>::bind(pure_a_cfn, f.clone());

        // rhs: f(a)
        // f(a) -> CFn(|env| (a + env).to_string())
        let rhs_cfn: CFn<Env, String> = f.clone()(a);

        // Compare by calling with the same environment
        assert_eq!(lhs_cfn.call(env_val), rhs_cfn.call(env_val));
        assert_eq!(lhs_cfn.call(env_val), "15".to_string()); // 10 + 5
    }

    // 2. Right Identity: m.bind(CFnHKTMarker::pure) == m
    #[test]
    fn cfn_hkt_monad_right_identity() {
        let env_val: Env = 7;
        // m_creator: () -> CFn<Env, i32>
        let m_creator = || CFn::new(move |env: Env| env * 2); 

        // pure_fn: i32 -> CFn<Env, i32>
        let pure_fn = clone_fn(|val: i32| CFnHKTMarker::<Env>::pure(val));
        
        let lhs_cfn: CFn<Env, i32> = CFnHKTMarker::<Env>::bind(m_creator(), pure_fn);
        let rhs_cfn: CFn<Env, i32> = m_creator();

        assert_eq!(lhs_cfn.call(env_val), rhs_cfn.call(env_val));
        assert_eq!(lhs_cfn.call(env_val), 14); // 7 * 2
    }

    // 3. Associativity: m.bind(f).bind(g) == m.bind(|x| f(x).bind(g))
    #[test]
    fn cfn_hkt_monad_associativity() {
        let env_val: Env = 3;
        // m_creator: () -> CFn<Env, i32>
        let m_creator = || CFn::new(move |env: Env| env + 1); // e.g., 3 -> 4

        // f: i32 -> CFn<Env, f64>
        let f = clone_fn(move |x: i32| -> CFn<Env, f64> {
            CFn::new(move |env: Env| (x * env) as f64) // e.g., x=4, env=3 -> 12.0
        });

        // g: f64 -> CFn<Env, String>
        let g = clone_fn(move |y: f64| -> CFn<Env, String> {
            CFn::new(move |env: Env| (y + (env as f64)).to_string()) // e.g., y=12.0, env=3 -> "15"
        });

        // lhs: m.bind(f).bind(g)
        let bound_f: CFn<Env, f64> = CFnHKTMarker::<Env>::bind(m_creator(), f.clone());
        let lhs_cfn: CFn<Env, String> = CFnHKTMarker::<Env>::bind(bound_f, g.clone());

        // rhs: m.bind(|x_val| f(x_val).bind(g))
        let f_inner = f.clone();
        let g_inner = g.clone();
        let composed_func = clone_fn(move |x_val: i32| -> CFn<Env, String> {
            let fx: CFn<Env, f64> = f_inner.clone()(x_val);
            CFnHKTMarker::<Env>::bind(fx, g_inner.clone())
        });
        let rhs_cfn: CFn<Env, String> = CFnHKTMarker::<Env>::bind(m_creator(), composed_func);

        assert_eq!(lhs_cfn.call(env_val), rhs_cfn.call(env_val));
        assert_eq!(lhs_cfn.call(env_val), "15".to_string());
    }

    // Monad::join laws for CFnHKTMarker
    // join(pure(pure(x))) == pure(x)
    // join(map(pure, m)) == m
    // join(pure(m)) == m : This law is problematic for CFn as pure(m) would require m (a CFn) to be Clone.
    #[test]
    fn cfn_hkt_monad_join_law1() { // join(pure(pure(x))) == pure(x)
        let env_val: Env = 5;
        let x: i32 = 10; // Must be Clone

        // mma: CFn<Env, CFn<Env, i32>>
        // Construct mma as CFn(|_env| pure(x))
        // pure(x) itself is CFn(|_env_inner| x.clone())
        let mma: CFn<Env, CFn<Env, i32>> = 
            CFn::new(move |_env_outer: Env| CFnHKTMarker::<Env>::pure(x.clone()));
        
        let lhs_cfn: CFn<Env, i32> = CFnHKTMarker::<Env>::join(mma);
        let rhs_cfn: CFn<Env, i32> = CFnHKTMarker::<Env>::pure(x);

        assert_eq!(lhs_cfn.call(env_val), rhs_cfn.call(env_val));
        assert_eq!(lhs_cfn.call(env_val), 10);
    }

    #[test]
    fn cfn_hkt_monad_join_law2() { // join(map(pure, m)) == m
        let env_val: Env = 7;
        // m_creator: () -> CFn<Env, i32>
        let m_creator = || CFn::new(move |env: Env| env * 3); 

        let pure_fn = clone_fn(|val: i32| CFnHKTMarker::<Env>::pure(val));
        
        let mapped_m_cfn: CFn<Env, CFn<Env, i32>> = CFnHKTMarker::<Env>::map(m_creator(), pure_fn);
        
        let lhs_cfn: CFn<Env, i32> = CFnHKTMarker::<Env>::join(mapped_m_cfn);
        let rhs_cfn: CFn<Env, i32> = m_creator();

        assert_eq!(lhs_cfn.call(env_val), rhs_cfn.call(env_val));
        assert_eq!(lhs_cfn.call(env_val), 21);
    }

    // #[test] // Commented out: join(pure(m)) == m is problematic for non-Clone CFn
    // fn cfn_hkt_monad_join_law3() { 
    //     let env_val: Env = 4;
    //     let m_creator = || CFn::new(move |env: Env| env + 10); 

    //     // pure_m_cfn requires m_creator() to be Clone, which CFn is not.
    //     // let pure_m_cfn: CFn<Env, CFn<Env, i32>> = CFnHKTMarker::<Env>::pure(m_creator());
        
    //     // let lhs_cfn: CFn<Env, i32> = CFnHKTMarker::<Env>::join(pure_m_cfn);
    //     // let rhs_cfn: CFn<Env, i32> = m_creator();

    //     // assert_eq!(lhs_cfn.call(env_val), rhs_cfn.call(env_val));
    //     // assert_eq!(lhs_cfn.call(env_val), 14);
    // }
}

mod cfn_once_hkt_monad_laws {
    use super::*; // Imports Bind, Monad, Applicative, CFnOnceHKTMarker, CFnOnce, once_fn
    type Env = i32;

    // 1. Left Identity: CFnOnceHKTMarker::pure(a).bind(f) == f(a)
    #[test]
    fn cfn_once_hkt_monad_left_identity() {
        let env_val: Env = 5;
        let a: i32 = 10; 

        // f: i32 -> CFnOnce<Env, String>
        let f = |x: i32| -> CFnOnce<Env, String> {
            CFnOnce::new(move |env: Env| (x + env).to_string())
        };

        let pure_a_cfn_once: CFnOnce<Env, i32> = CFnOnceHKTMarker::<Env>::pure(a);
        // bind consumes pure_a_cfn_once and the FnMut from f
        let lhs_cfn_once: CFnOnce<Env, String> = CFnOnceHKTMarker::<Env>::bind(pure_a_cfn_once, f);
        
        // For RHS, f needs to be callable again. Since it's FnOnce by signature in bind,
        // we need a fresh f for the RHS if we consumed the original in LHS.
        // Or, ensure f is actually Fn (like the closure above is).
        let f_for_rhs = |x: i32| -> CFnOnce<Env, String> {
            CFnOnce::new(move |env: Env| (x + env).to_string())
        };
        let rhs_cfn_once: CFnOnce<Env, String> = f_for_rhs(a);

        assert_eq!(lhs_cfn_once.call_once(env_val), rhs_cfn_once.call_once(env_val));
        // Re-create for second assert if needed, or assert the value directly
        let f_for_assert = |x: i32| -> CFnOnce<Env, String> {
            CFnOnce::new(move |env: Env| (x + env).to_string())
        };
        let pure_a_for_assert: CFnOnce<Env, i32> = CFnOnceHKTMarker::<Env>::pure(a);
        assert_eq!(CFnOnceHKTMarker::<Env>::bind(pure_a_for_assert, f_for_assert).call_once(env_val), "15".to_string());
    }

    // 2. Right Identity: m.bind(CFnOnceHKTMarker::pure) == m
    #[test]
    fn cfn_once_hkt_monad_right_identity() {
        let env_val: Env = 7;
        let m_creator = || CFnOnce::new(move |env: Env| env * 2);

        let pure_fn = |val: i32| CFnOnceHKTMarker::<Env>::pure(val);
        
        let lhs_cfn_once: CFnOnce<Env, i32> = CFnOnceHKTMarker::<Env>::bind(m_creator(), pure_fn);
        let rhs_cfn_once: CFnOnce<Env, i32> = m_creator();

        assert_eq!(lhs_cfn_once.call_once(env_val), rhs_cfn_once.call_once(env_val));
        assert_eq!(m_creator().call_once(env_val), 14);
    }

    // 3. Associativity: m.bind(f).bind(g) == m.bind(|x| f(x).bind(g))
    #[test]
    fn cfn_once_hkt_monad_associativity() {
        let env_val: Env = 3;
        let m_creator = || CFnOnce::new(move |env: Env| env + 1); // 3 -> 4

        let f = |x: i32| -> CFnOnce<Env, f64> {
            CFnOnce::new(move |env: Env| (x * env) as f64) // x=4, env=3 -> 12.0
        };
        let g = |y: f64| -> CFnOnce<Env, String> {
            CFnOnce::new(move |env: Env| (y + (env as f64)).to_string()) // y=12.0, env=3 -> "15"
        };

        // lhs: m.bind(f).bind(g)
        // Need to ensure f and g are "cloneable" in spirit for multiple uses in the law.
        // Since bind takes FnMut, the closures f and g above are fine.
        let bound_f: CFnOnce<Env, f64> = CFnOnceHKTMarker::<Env>::bind(m_creator(), f);
        let lhs_cfn_once: CFnOnce<Env, String> = CFnOnceHKTMarker::<Env>::bind(bound_f, g);

        // rhs: m.bind(|x_val| f(x_val).bind(g))
        // Re-define f and g for the RHS to ensure they are fresh if consumed.
        let f_for_rhs = |x: i32| -> CFnOnce<Env, f64> {
            CFnOnce::new(move |env: Env| (x * env) as f64)
        };
        let g_for_rhs = |y: f64| -> CFnOnce<Env, String> {
            CFnOnce::new(move |env: Env| (y + (env as f64)).to_string())
        };
        let composed_func = move |x_val: i32| -> CFnOnce<Env, String> {
            let fx: CFnOnce<Env, f64> = f_for_rhs(x_val);
            CFnOnceHKTMarker::<Env>::bind(fx, g_for_rhs)
        };
        let rhs_cfn_once: CFnOnce<Env, String> = CFnOnceHKTMarker::<Env>::bind(m_creator(), composed_func);

        assert_eq!(lhs_cfn_once.call_once(env_val), rhs_cfn_once.call_once(env_val));
         // For direct value check, re-evaluate one side:
        let m_check = || CFnOnce::new(move |env: Env| env + 1);
        let f_check = |x: i32| -> CFnOnce<Env, f64> { CFnOnce::new(move |env: Env| (x * env) as f64) };
        let g_check = |y: f64| -> CFnOnce<Env, String> { CFnOnce::new(move |env: Env| (y + (env as f64)).to_string()) };
        let bound_f_check = CFnOnceHKTMarker::<Env>::bind(m_check(), f_check);
        assert_eq!(CFnOnceHKTMarker::<Env>::bind(bound_f_check, g_check).call_once(env_val), "15".to_string());
    }

    // Monad::join laws for CFnOnceHKTMarker
    #[test]
    fn cfn_once_hkt_monad_join_law1() { // join(pure(pure(x))) == pure(x)
        let env_val: Env = 5;
        let x: i32 = 10; 

        let mma: CFnOnce<Env, CFnOnce<Env, i32>> = 
            CFnOnce::new(move |_env_outer: Env| CFnOnceHKTMarker::<Env>::pure(x.clone()));
        
        let lhs_cfn_once: CFnOnce<Env, i32> = CFnOnceHKTMarker::<Env>::join(mma);
        let rhs_cfn_once: CFnOnce<Env, i32> = CFnOnceHKTMarker::<Env>::pure(x);

        assert_eq!(lhs_cfn_once.call_once(env_val), rhs_cfn_once.call_once(env_val));
        assert_eq!(CFnOnceHKTMarker::<Env>::pure(x).call_once(env_val), 10);
    }

    #[test]
    fn cfn_once_hkt_monad_join_law2() { // join(map(pure, m)) == m
        let env_val: Env = 7;
        let m_creator = || CFnOnce::new(move |env: Env| env * 3); 

        let pure_fn = |val: i32| CFnOnceHKTMarker::<Env>::pure(val);
        
        let mapped_m_cfn_once: CFnOnce<Env, CFnOnce<Env, i32>> = CFnOnceHKTMarker::<Env>::map(m_creator(), pure_fn);
        
        let lhs_cfn_once: CFnOnce<Env, i32> = CFnOnceHKTMarker::<Env>::join(mapped_m_cfn_once);
        let rhs_cfn_once: CFnOnce<Env, i32> = m_creator();

        assert_eq!(lhs_cfn_once.call_once(env_val), rhs_cfn_once.call_once(env_val));
        assert_eq!(m_creator().call_once(env_val), 21);
    }
}
