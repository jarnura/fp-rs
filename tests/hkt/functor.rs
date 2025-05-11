
use monadify::functor::hkt::Functor;
use monadify::kind_based::kind::{OptionHKTMarker, ResultHKTMarker, VecHKTMarker, CFnHKTMarker, CFnOnceHKTMarker};
use monadify::identity::{Identity, IdentityHKTMarker}; // Corrected import for IdentityHKTMarker
use monadify::function::{CFn, CFnOnce};
use monadify::transformers::reader::{ReaderT, ReaderTHKTMarker};
// HKT1 import removed as it was unused.


// Common error type for Result tests
type TestError = String;
type TestResult<T> = Result<T, TestError>;

// Helper for creating a cloneable FnMut for map
fn clone_fn_map<A, B, F>(f: F) -> impl FnMut(A) -> B + Clone + 'static
where
    F: Fn(A) -> B + Clone + 'static,
    A: 'static,
    B: 'static,
{
    f
}

// Unused helper `once_fn_map` removed.

pub mod option_hkt_functor_laws {
    use super::*;

    // Identity law: OptionHKTMarker::map(opt, |x| x) == opt
    #[test]
    fn option_hkt_functor_identity_some() {
        let opt = Some(10);
        let identity_fn = clone_fn_map(|x: i32| x);
        assert_eq!(OptionHKTMarker::map(opt.clone(), identity_fn), opt);
    }

    #[test]
    fn option_hkt_functor_identity_none() {
        let opt: Option<i32> = None;
        let identity_fn = clone_fn_map(|x: i32| x);
        assert_eq!(OptionHKTMarker::map(opt.clone(), identity_fn), opt);
    }

    // Composition law: OptionHKTMarker::map(opt, |x| g(f(x))) == OptionHKTMarker::map(OptionHKTMarker::map(opt, f), g)
    #[test]
    fn option_hkt_functor_composition_some() {
        let opt = Some(10);
        let f = clone_fn_map(|x: i32| x * 2); 
        let g = clone_fn_map(|y: i32| y + 5); 

        let f_clone = f.clone();
        let g_clone = g.clone();
        let composed_map = OptionHKTMarker::map(opt.clone(), move |x| g_clone.clone()(f_clone.clone()(x)));
        let sequential_map = OptionHKTMarker::map(OptionHKTMarker::map(opt, f), g);

        assert_eq!(composed_map, sequential_map);
        assert_eq!(composed_map, Some(25));
    }

    #[test]
    fn option_hkt_functor_composition_none() {
        let opt: Option<i32> = None;
        let f = clone_fn_map(|x: i32| x * 2);
        let g = clone_fn_map(|y: i32| y + 5);

        let f_clone = f.clone();
        let g_clone = g.clone();
        let composed_map = OptionHKTMarker::map(opt.clone(), move |x| g_clone.clone()(f_clone.clone()(x)));
        let sequential_map = OptionHKTMarker::map(OptionHKTMarker::map(opt, f), g);
        
        assert_eq!(composed_map, sequential_map);
        assert_eq!(composed_map, None);
    }

    #[test]
    fn option_hkt_functor_composition_some_str() {
        let opt = Some("hello");
        let f = clone_fn_map(|x: &str| x.to_uppercase()); 
        let g = clone_fn_map(|y: String| y.len()); 

        let f_clone = f.clone();
        let g_clone = g.clone();
        let composed_map = OptionHKTMarker::map(opt, move |x| g_clone.clone()(f_clone.clone()(x)));
        
        // Need to re-define for sequential map as opt is consumed if it's not Clone (like &str)
        // However, Some("hello") is Some(&'static str) which is Copy.
        let opt_for_seq = Some("hello"); 
        let sequential_map = OptionHKTMarker::map(OptionHKTMarker::map(opt_for_seq, f), g);

        assert_eq!(composed_map, sequential_map);
        assert_eq!(composed_map, Some(5));
    }
}

pub mod result_hkt_functor_laws {
    use super::*;

    // Identity law: ResultHKTMarker::map(res, |x| x) == res
    #[test]
    fn result_hkt_functor_identity_ok() {
        let res: TestResult<i32> = Ok(10);
        let identity_fn = clone_fn_map(|x: i32| x);
        assert_eq!(ResultHKTMarker::<TestError>::map(res.clone(), identity_fn), res);
    }

    #[test]
    fn result_hkt_functor_identity_err() {
        let res: TestResult<i32> = Err("error".to_string());
        let identity_fn = clone_fn_map(|x: i32| x);
        assert_eq!(ResultHKTMarker::<TestError>::map(res.clone(), identity_fn), res);
    }

    // Composition law: ResultHKTMarker::map(res, |x| g(f(x))) == ResultHKTMarker::map(ResultHKTMarker::map(res, f), g)
    #[test]
    fn result_hkt_functor_composition_ok() {
        let res: TestResult<i32> = Ok(10);
        let f = clone_fn_map(|x: i32| x * 2);
        let g = clone_fn_map(|y: i32| y + 5);

        let f_clone = f.clone();
        let g_clone = g.clone();
        let composed_map = ResultHKTMarker::<TestError>::map(res.clone(), move |x| g_clone.clone()(f_clone.clone()(x)));
        let sequential_map = ResultHKTMarker::<TestError>::map(ResultHKTMarker::<TestError>::map(res, f), g);
        
        assert_eq!(composed_map, sequential_map);
        assert_eq!(composed_map, Ok(25));
    }

    #[test]
    fn result_hkt_functor_composition_err() {
        let res: TestResult<i32> = Err("error".to_string());
        let f = clone_fn_map(|x: i32| x * 2);
        let g = clone_fn_map(|y: i32| y + 5);

        let f_clone = f.clone();
        let g_clone = g.clone();
        let composed_map = ResultHKTMarker::<TestError>::map(res.clone(), move |x| g_clone.clone()(f_clone.clone()(x)));
        let sequential_map = ResultHKTMarker::<TestError>::map(ResultHKTMarker::<TestError>::map(res, f), g);

        assert_eq!(composed_map, sequential_map);
        assert_eq!(composed_map, Err("error".to_string()));
    }

    #[test]
    fn result_hkt_functor_composition_ok_str_err_u32() {
        let res: Result<&str, u32> = Ok("hello");
        let f = clone_fn_map(|x: &str| x.to_uppercase());
        let g = clone_fn_map(|y: String| y.len());

        let f_clone = f.clone();
        let g_clone = g.clone();
        let composed_map = ResultHKTMarker::<u32>::map(res, move |x| g_clone.clone()(f_clone.clone()(x)));
        
        let res_for_seq: Result<&str, u32> = Ok("hello");
        let sequential_map = ResultHKTMarker::<u32>::map(ResultHKTMarker::<u32>::map(res_for_seq, f), g);

        assert_eq!(composed_map, sequential_map);
        assert_eq!(composed_map, Ok(5));
    }

    #[test]
    fn result_hkt_functor_composition_err_str_err_u32() {
        let res: Result<&str, u32> = Err(404);
        let f = clone_fn_map(|x: &str| x.to_uppercase());
        let g = clone_fn_map(|y: String| y.len());

        let f_clone = f.clone();
        let g_clone = g.clone();
        let composed_map = ResultHKTMarker::<u32>::map(res, move |x| g_clone.clone()(f_clone.clone()(x)));

        let res_for_seq: Result<&str, u32> = Err(404);
        let sequential_map = ResultHKTMarker::<u32>::map(ResultHKTMarker::<u32>::map(res_for_seq, f), g);
        
        assert_eq!(composed_map, sequential_map);
        assert_eq!(composed_map, Err(404));
    }
}

pub mod vec_hkt_functor_laws {
    use super::*;

    // Identity law: VecHKTMarker::map(vec, |x| x) == vec
    #[test]
    fn vec_hkt_functor_identity_non_empty() {
        let vec_val = vec![10, 20, 30];
        let identity_fn = clone_fn_map(|x: i32| x);
        assert_eq!(VecHKTMarker::map(vec_val.clone(), identity_fn), vec_val);
    }

    #[test]
    fn vec_hkt_functor_identity_empty() {
        let vec_val: Vec<i32> = vec![];
        let identity_fn = clone_fn_map(|x: i32| x);
        assert_eq!(VecHKTMarker::map(vec_val.clone(), identity_fn), vec_val);
    }

    // Composition law: VecHKTMarker::map(vec, |x| g(f(x))) == VecHKTMarker::map(VecHKTMarker::map(vec, f), g)
    #[test]
    fn vec_hkt_functor_composition_non_empty() {
        let vec_val = vec![10, 20, 30];
        let f = clone_fn_map(|x: i32| x * 2); 
        let g = clone_fn_map(|y: i32| y + 5); 

        let f_clone = f.clone();
        let g_clone = g.clone();
        let composed_map = VecHKTMarker::map(vec_val.clone(), move |x| g_clone.clone()(f_clone.clone()(x)));
        let sequential_map = VecHKTMarker::map(VecHKTMarker::map(vec_val, f), g);

        assert_eq!(composed_map, sequential_map);
        assert_eq!(composed_map, vec![25, 45, 65]);
    }

    #[test]
    fn vec_hkt_functor_composition_empty() {
        let vec_val: Vec<i32> = vec![];
        let f = clone_fn_map(|x: i32| x * 2);
        let g = clone_fn_map(|y: i32| y + 5);

        let f_clone = f.clone();
        let g_clone = g.clone();
        let composed_map = VecHKTMarker::map(vec_val.clone(), move |x| g_clone.clone()(f_clone.clone()(x)));
        let sequential_map = VecHKTMarker::map(VecHKTMarker::map(vec_val, f), g);
        
        assert_eq!(composed_map, sequential_map);
        assert_eq!(composed_map, Vec::<i32>::new());
    }

    #[test]
    fn vec_hkt_functor_composition_str() {
        let vec_val = vec!["hello", "world"];
        let f = clone_fn_map(|x: &str| x.to_uppercase()); 
        let g = clone_fn_map(|y: String| y.len()); 

        let f_clone = f.clone();
        let g_clone = g.clone();
        let composed_map = VecHKTMarker::map(vec_val.clone(), move |x| g_clone.clone()(f_clone.clone()(x)));
        
        let sequential_map = VecHKTMarker::map(VecHKTMarker::map(vec_val, f), g);

        assert_eq!(composed_map, sequential_map);
        assert_eq!(composed_map, vec![5, 5]);
    }
}

pub mod cfn_hkt_functor_laws {
    use super::*;
    type Env = i32; // Common environment type for these tests

    // Identity law: CFnHKTMarker::map(cfn, |x| x) == cfn
    #[test]
    fn cfn_hkt_functor_identity() {
        let env_val: Env = 5;
        let cfn_creator = || CFn::new(move |env: Env| env * 2); // Example CFn: 5 -> 10
        
        let identity_fn = clone_fn_map(|x: i32| x);
        // CFn is not Clone, recreate if needed or structure test to consume.
        // map consumes the input cfn_val.
        let mapped_cfn: CFn<Env, i32> = CFnHKTMarker::<Env>::map(cfn_creator(), identity_fn);

        assert_eq!(mapped_cfn.call(env_val), cfn_creator().call(env_val));
        assert_eq!(mapped_cfn.call(env_val), 10);
    }

    // Composition law: CFnHKTMarker::map(cfn, |x| g(f(x))) == CFnHKTMarker::map(CFnHKTMarker::map(cfn, f), g)
    #[test]
    fn cfn_hkt_functor_composition() {
        let env_val: Env = 3;
        let cfn_creator = || CFn::new(move |env: Env| env + 1); // Example CFn: 3 -> 4

        let f = clone_fn_map(|x: i32| (x * x) as f64); // i32 -> f64, e.g., 4 -> 16.0
        let g = clone_fn_map(|y: f64| y.to_string());    // f64 -> String, e.g., 16.0 -> "16"

        let f_clone_for_composed = f.clone();
        let g_clone_for_composed = g.clone();
        let composed_map_cfn: CFn<Env, String> = 
            CFnHKTMarker::<Env>::map(cfn_creator(), move |x| g_clone_for_composed.clone()(f_clone_for_composed.clone()(x)));

        let mapped_f_cfn: CFn<Env, f64> = CFnHKTMarker::<Env>::map(cfn_creator(), f);
        let sequential_map_cfn: CFn<Env, String> = CFnHKTMarker::<Env>::map(mapped_f_cfn, g);

        assert_eq!(composed_map_cfn.call(env_val), sequential_map_cfn.call(env_val));
        assert_eq!(composed_map_cfn.call(env_val), "16".to_string());
    }
}

pub mod cfn_once_hkt_functor_laws {
    use super::*;
    type Env = i32; // Common environment type for these tests

    // Identity law: CFnOnceHKTMarker::map(cfn_once, |x| x) == cfn_once
    #[test]
    fn cfn_once_hkt_functor_identity() {
        let env_val: Env = 5;
        let cfn_once_val_creator = || CFnOnce::new(move |env: Env| env * 2); // Example CFnOnce: 5 -> 10
        
        let identity_fn = clone_fn_map(|x: i32| x); // Use clone_fn_map
        let mapped_cfn_once: CFnOnce<Env, i32> = CFnOnceHKTMarker::<Env>::map(cfn_once_val_creator(), identity_fn.clone());

        assert_eq!(mapped_cfn_once.call_once(env_val), cfn_once_val_creator().call_once(env_val));
        
        assert_eq!(CFnOnceHKTMarker::<Env>::map(cfn_once_val_creator(), identity_fn).call_once(env_val), 10);
    }

    // Composition law: CFnOnceHKTMarker::map(cfn_once, |x| g(f(x))) == CFnOnceHKTMarker::map(CFnOnceHKTMarker::map(cfn_once, f), g)
    #[test]
    fn cfn_once_hkt_functor_composition() {
        let env_val: Env = 3;
        let cfn_once_val_creator = || CFnOnce::new(move |env: Env| env + 1); // Example CFnOnce: 3 -> 4

        let f = clone_fn_map(|x: i32| (x * x) as f64); // i32 -> f64, e.g., 4 -> 16.0
        let g = clone_fn_map(|y: f64| y.to_string());    // f64 -> String, e.g., 16.0 -> "16"

        let f_clone_for_composed = f.clone();
        let g_clone_for_composed = g.clone();
        let composed_closure = clone_fn_map(move |x_val: i32| g_clone_for_composed.clone()(f_clone_for_composed.clone()(x_val)));
        let composed_map_cfn_once: CFnOnce<Env, String> = 
            CFnOnceHKTMarker::<Env>::map(cfn_once_val_creator(), composed_closure);
        
        let mapped_f_cfn_once: CFnOnce<Env, f64> = CFnOnceHKTMarker::<Env>::map(cfn_once_val_creator(), f.clone());
        let sequential_map_cfn_once: CFnOnce<Env, String> = CFnOnceHKTMarker::<Env>::map(mapped_f_cfn_once, g.clone());

        assert_eq!(composed_map_cfn_once.call_once(env_val), sequential_map_cfn_once.call_once(env_val));
        
        let f_check = clone_fn_map(|x: i32| (x * x) as f64);
        let g_check = clone_fn_map(|y: f64| y.to_string());
        let composed_closure_check = clone_fn_map(move |x_val: i32| g_check.clone()(f_check.clone()(x_val)));
        assert_eq!(CFnOnceHKTMarker::<Env>::map(cfn_once_val_creator(), composed_closure_check).call_once(env_val), "16".to_string());
    }
}

pub mod identity_hkt_functor_laws {
    use super::*;

    // Identity law: IdentityHKTMarker::map(id_val, |x| x) == id_val
    #[test]
    fn identity_hkt_functor_identity() {
        let id_val = Identity(10); // Corrected: Use tuple struct constructor
        let identity_fn = clone_fn_map(|x: i32| x);
        assert_eq!(IdentityHKTMarker::map(id_val.clone(), identity_fn), id_val);
    }

    // Composition law: IdentityHKTMarker::map(id_val, |x| g(f(x))) == IdentityHKTMarker::map(IdentityHKTMarker::map(id_val, f), g)
    #[test]
    fn identity_hkt_functor_composition() {
        let id_val = Identity(10); // Corrected: Use tuple struct constructor
        let f = clone_fn_map(|x: i32| x * 2); 
        let g = clone_fn_map(|y: i32| y + 5); 

        let f_clone = f.clone();
        let g_clone = g.clone();
        let composed_map = IdentityHKTMarker::map(id_val.clone(), move |x| g_clone.clone()(f_clone.clone()(x)));
        let sequential_map = IdentityHKTMarker::map(IdentityHKTMarker::map(id_val, f), g);

        assert_eq!(composed_map, sequential_map);
        assert_eq!(composed_map, Identity(25)); // Corrected: Use tuple struct constructor
    }

    #[test]
    fn identity_hkt_functor_composition_str() {
        let id_val = Identity("hello"); // Corrected: Use tuple struct constructor
        let f = clone_fn_map(|x: &str| x.to_uppercase()); 
        let g = clone_fn_map(|y: String| y.len()); 

        let f_clone = f.clone();
        let g_clone = g.clone();
        let composed_map = IdentityHKTMarker::map(id_val.clone(), move |x: &str| g_clone.clone()(f_clone.clone()(x)));
        
        let sequential_map = IdentityHKTMarker::map(IdentityHKTMarker::map(id_val, f), g);

        assert_eq!(composed_map, sequential_map);
        assert_eq!(composed_map, Identity(5)); // Corrected: Use tuple struct constructor
    }
}

pub mod reader_t_hkt_functor_laws {
    use super::*;
    // Env for ReaderT, InnerMonad for ReaderT (e.g. Option)
    type EnvReader = String; 
    type InnerMonadMarker = OptionHKTMarker; // ReaderT<R, Option<A>, A>

    // Identity law: ReaderTHKTMarker::map(reader_t, |x| x) == reader_t
    #[test]
    fn reader_t_hkt_functor_identity() {
        let env_val = "test_env".to_string();
        let reader_t_creator = || ReaderT::new(move |_env: EnvReader| Some(10));
        
        let identity_fn = clone_fn_map(|x: i32| x);
        let mapped_reader_t: ReaderT<EnvReader, InnerMonadMarker, i32> = 
            ReaderTHKTMarker::<EnvReader, InnerMonadMarker>::map(reader_t_creator(), identity_fn);

        assert_eq!((mapped_reader_t.run_reader_t)(env_val.clone()), (reader_t_creator().run_reader_t)(env_val.clone()));
        assert_eq!((mapped_reader_t.run_reader_t)(env_val), Some(10));
    }

    #[test]
    fn reader_t_hkt_functor_identity_inner_none() {
        let env_val = "test_env".to_string();
        let reader_t_creator = || ReaderT::new(move |_env: EnvReader| None::<i32>);
        
        let identity_fn = clone_fn_map(|x: i32| x);
        let mapped_reader_t: ReaderT<EnvReader, InnerMonadMarker, i32> = 
            ReaderTHKTMarker::<EnvReader, InnerMonadMarker>::map(reader_t_creator(), identity_fn);

        assert_eq!((mapped_reader_t.run_reader_t)(env_val.clone()), (reader_t_creator().run_reader_t)(env_val.clone()));
        assert_eq!((mapped_reader_t.run_reader_t)(env_val), None);
    }

    // Composition law
    #[test]
    fn reader_t_hkt_functor_composition() {
        let env_val = "test_env".to_string();
        let reader_t_creator = || ReaderT::new(move |_env: EnvReader| Some(10));

        let f = clone_fn_map(|x: i32| (x as f64 * 2.0)); // i32 -> f64
        let g = clone_fn_map(|y: f64| format!("Value: {:.1}", y));    // f64 -> String

        let f_clone_composed = f.clone();
        let g_clone_composed = g.clone();
        let composed_map_reader_t: ReaderT<EnvReader, InnerMonadMarker, String> = 
            ReaderTHKTMarker::<EnvReader, InnerMonadMarker>::map(reader_t_creator(), move |x| g_clone_composed.clone()(f_clone_composed.clone()(x)));

        let mapped_f_reader_t: ReaderT<EnvReader, InnerMonadMarker, f64> = 
            ReaderTHKTMarker::<EnvReader, InnerMonadMarker>::map(reader_t_creator(), f);
        let sequential_map_reader_t: ReaderT<EnvReader, InnerMonadMarker, String> = 
            ReaderTHKTMarker::<EnvReader, InnerMonadMarker>::map(mapped_f_reader_t, g);

        assert_eq!((composed_map_reader_t.run_reader_t)(env_val.clone()), (sequential_map_reader_t.run_reader_t)(env_val.clone()));
        assert_eq!((composed_map_reader_t.run_reader_t)(env_val), Some("Value: 20.0".to_string()));
    }

    #[test]
    fn reader_t_hkt_functor_composition_inner_none() {
        let env_val = "test_env".to_string();
        let reader_t_creator = || ReaderT::new(move |_env: EnvReader| None::<i32>);

        let f = clone_fn_map(|x: i32| (x as f64 * 2.0)); 
        let g = clone_fn_map(|y: f64| format!("Value: {:.1}", y));   

        let f_clone_composed = f.clone();
        let g_clone_composed = g.clone();
        let composed_map_reader_t: ReaderT<EnvReader, InnerMonadMarker, String> = 
            ReaderTHKTMarker::<EnvReader, InnerMonadMarker>::map(reader_t_creator(), move |x| g_clone_composed.clone()(f_clone_composed.clone()(x)));

        let mapped_f_reader_t: ReaderT<EnvReader, InnerMonadMarker, f64> = 
            ReaderTHKTMarker::<EnvReader, InnerMonadMarker>::map(reader_t_creator(), f);
        let sequential_map_reader_t: ReaderT<EnvReader, InnerMonadMarker, String> = 
            ReaderTHKTMarker::<EnvReader, InnerMonadMarker>::map(mapped_f_reader_t, g);

        assert_eq!((composed_map_reader_t.run_reader_t)(env_val.clone()), (sequential_map_reader_t.run_reader_t)(env_val.clone()));
        assert_eq!((composed_map_reader_t.run_reader_t)(env_val), None);
    }
}
