// Original content from src/monad.rs test modules
// with use statements adjusted for the new location.

use fp_rs::monad::{Bind, join, bind}; // bind is also a free function in monad.rs
use fp_rs::applicative::Applicative;
use fp_rs::function::CFn;
use fp_rs::{fn1, bfn1}; // Macros are at crate root

#[cfg(test)]
mod tests {
    // Imports from the top of the file will be in scope
    use super::*; // This brings in Bind, join, bind (fn), Applicative, CFn, fn1, bfn1

    #[test]
    fn bind_option() {
        let add_one = fn1!(|x: i32| Some(x + 1));
        let add_two = fn1!(|x: i32| Some(x + 2));
        let add_three = fn1!(|x: i32| Some(x + 3));
        let result = Some(1).bind(add_one).bind(add_two).bind(add_three);
        assert_eq!(result, Some(7))
    }

    #[test]
    fn bind_option_with_composing() {
        let add_one = fn1!(|x: i32| Some(x + 1));
        let add_two = fn1!(|x: i32| x + 2);
        let add_three = fn1!(|x: i32| x + 3);
        let composed = add_one << add_two << add_three;
        let result = Some(1).bind(composed);
        assert_eq!(result, Some(7));

        let result = join(Some(Some(1)));
        assert_eq!(result, Some(1));

        let result = join(Some(None::<i32>));
        assert_eq!(result, None);
    }

    #[test]
    fn bind_option_with_bind_composing() {
        let add_one = bfn1!(|x: i32| Some(x + 1));
        let add_two = bfn1!(|x: i32| Some(x + 2));
        let add_three = bfn1!(|x: i32| Some(x + 3));
        let result = Some(1) | add_one | add_two | add_three;
        assert_eq!(result, Some(7))
    }
}


#[cfg(test)]
mod monad_laws {
    use fp_rs::monad::Bind; // Bind is re-exported from lib.rs, but also defined in monad.rs
                           // Using fp_rs::Bind should be fine as it's re-exported.
    use fp_rs::Applicative; // For pure
    use fp_rs::function::CFn; // For wrapping functions for bind

    // 1. Left Identity: Option::pure(a).bind(f) == f(a)
    #[test]
    fn option_monad_left_identity() {
        let a = 10;
        let f = |x: i32| -> Option<String> { Some((x * 2).to_string()) };
        
        let lhs = Option::pure(a).bind(CFn::new(f));
        let rhs = f(a);

        assert_eq!(lhs, rhs);
        assert_eq!(lhs, Some("20".to_string()));
    }

    #[test]
    fn option_monad_left_identity_f_returns_none() {
        let a = 10;
        let f = |_x: i32| -> Option<String> { None };
        
        let lhs = Option::pure(a).bind(CFn::new(f));
        let rhs = f(a);

        assert_eq!(lhs, rhs);
        assert_eq!(lhs, None);
    }

    // 2. Right Identity: m.bind(Option::pure) == m
    #[test]
    fn option_monad_right_identity_some() {
        let m = Some(10);
        let pure_fn = Option::pure as fn(i32) -> Option<i32>;
        
        let lhs = m.bind(CFn::new(pure_fn));
        let rhs = Some(10);

        assert_eq!(lhs, rhs);
        assert_eq!(lhs, Some(10));
    }

     #[test]
    fn option_monad_right_identity_none() {
        let m: Option<i32> = None;
        let pure_fn = Option::pure as fn(i32) -> Option<i32>;
        
        let lhs = m.bind(CFn::new(pure_fn));
        let rhs = None::<i32>;

        assert_eq!(lhs, rhs);
        assert_eq!(lhs, None);
    }

    // 3. Associativity: m.bind(f).bind(g) == m.bind(|x| f(x).bind(g))
    #[test]
    fn option_monad_associativity_some() {
        let m = Some(10); 
        let f = |x: i32| -> Option<f64> { Some((x * 2) as f64) };
        let g = |y: f64| -> Option<String> { Some(y.to_string()) };

        let lhs = m.clone().bind(CFn::new(f)).bind(CFn::new(g));

        let f_inner = |x: i32| -> Option<f64> { Some((x * 2) as f64) };
        let g_inner = |y: f64| -> Option<String> { Some(y.to_string()) };
        let inner_closure = move |x: i32| (CFn::new(f_inner))(x).bind(CFn::new(g_inner));
        let rhs = m.bind(CFn::new(inner_closure));

        assert_eq!(lhs, rhs);
        assert_eq!(lhs, Some("20".to_string()));
    }

    #[test]
    fn option_monad_associativity_none_start() {
        let m: Option<i32> = None; 
        let f = |x: i32| -> Option<f64> { Some((x * 2) as f64) };
        let g = |y: f64| -> Option<String> { Some(y.to_string()) };
        
        let lhs = m.clone().bind(CFn::new(f)).bind(CFn::new(g));
        
        let f_inner = |x: i32| -> Option<f64> { Some((x * 2) as f64) };
        let g_inner = |y: f64| -> Option<String> { Some(y.to_string()) };
        let inner_closure = move |x: i32| (CFn::new(f_inner))(x).bind(CFn::new(g_inner));
        let rhs = m.bind(CFn::new(inner_closure));

        assert_eq!(lhs, rhs);
        assert_eq!(lhs, None);
    }

    #[test]
    fn option_monad_associativity_f_returns_none() {
        let m = Some(10);
        let f = |_x: i32| -> Option<f64> { None };
        let g = |y: f64| -> Option<String> { Some(y.to_string()) };

        let lhs = m.clone().bind(CFn::new(f)).bind(CFn::new(g));
        
        let f_inner = |_x: i32| -> Option<f64> { None };
        let g_inner = |y: f64| -> Option<String> { Some(y.to_string()) };
        let inner_closure = move |x: i32| (CFn::new(f_inner))(x).bind(CFn::new(g_inner));
        let rhs = m.bind(CFn::new(inner_closure));

        assert_eq!(lhs, rhs);
        assert_eq!(lhs, None);
    }

     #[test]
    fn option_monad_associativity_g_returns_none() {
        let m = Some(10);
        let f = |x: i32| -> Option<f64> { Some((x * 2) as f64) };
        let g = |_y: f64| -> Option<String> { None };

        let lhs = m.clone().bind(CFn::new(f)).bind(CFn::new(g));
        
        let f_inner = |x: i32| -> Option<f64> { Some((x * 2) as f64) };
        let g_inner = |_y: f64| -> Option<String> { None };
        let inner_closure = move |x: i32| (CFn::new(f_inner))(x).bind(CFn::new(g_inner));
        let rhs = m.bind(CFn::new(inner_closure));

        assert_eq!(lhs, rhs);
        assert_eq!(lhs, None);
    }
}

#[cfg(test)]
mod result_monad_laws {
    use fp_rs::monad::Bind;
    use fp_rs::Applicative; 
    use fp_rs::function::CFn;           

    // 1. Left Identity: Result::pure(a).bind(f) == f(a)
    #[test]
    fn result_monad_left_identity_ok() {
        let a = 10;
        let f = |x: i32| -> Result<String, String> { Ok((x * 2).to_string()) };
        
        let lhs = Result::pure(a).bind(CFn::new(f));
        let rhs = f(a);

        assert_eq!(lhs, rhs);
        assert_eq!(lhs, Ok("20".to_string()));
    }

    #[test]
    fn result_monad_left_identity_f_returns_err() {
        let a = 10;
        let f = |_x: i32| -> Result<String, String> { Err("f_error".to_string()) };
        
        let lhs = Result::pure(a).bind(CFn::new(f));
        let rhs = f(a);

        assert_eq!(lhs, rhs);
        assert_eq!(lhs, Err("f_error".to_string()));
    }

    // 2. Right Identity: m.bind(Result::pure) == m
    #[test]
    fn result_monad_right_identity_ok() {
        let m: Result<i32, String> = Ok(10);
        let pure_fn = Result::pure as fn(i32) -> Result<i32, String>;
        
        let lhs = m.clone().bind(CFn::new(pure_fn));
        let rhs = m;

        assert_eq!(lhs, rhs);
        assert_eq!(lhs, Ok(10));
    }

    #[test]
    fn result_monad_right_identity_err() {
        let m: Result<i32, String> = Err("m_error".to_string());
        let pure_fn = Result::pure as fn(i32) -> Result<i32, String>;
        
        let lhs = m.clone().bind(CFn::new(pure_fn));
        let rhs = m;

        assert_eq!(lhs, rhs);
        assert_eq!(lhs, Err("m_error".to_string()));
    }

    // 3. Associativity: m.bind(f).bind(g) == m.bind(|x| f(x).bind(g))
    #[test]
    fn result_monad_associativity_all_ok() {
        let m: Result<i32, String> = Ok(10);
        let f = |x: i32| -> Result<f64, String> { Ok((x * 2) as f64) };
        let g = |y: f64| -> Result<String, String> { Ok(y.to_string()) };

        let lhs = m.clone().bind(CFn::new(f)).bind(CFn::new(g));
        
        let f_inner = |x: i32| -> Result<f64, String> { Ok((x * 2) as f64) };
        let g_inner = |y: f64| -> Result<String, String> { Ok(y.to_string()) };
        let inner_closure = move |x: i32| (CFn::new(f_inner))(x).bind(CFn::new(g_inner));
        let rhs = m.bind(CFn::new(inner_closure));

        assert_eq!(lhs, rhs);
        assert_eq!(lhs, Ok("20".to_string()));
    }

    #[test]
    fn result_monad_associativity_m_is_err() {
        let m: Result<i32, String> = Err("m_error".to_string());
        let f = |x: i32| -> Result<f64, String> { Ok((x * 2) as f64) };
        let g = |y: f64| -> Result<String, String> { Ok(y.to_string()) };

        let lhs = m.clone().bind(CFn::new(f)).bind(CFn::new(g));
        
        let f_inner = |x: i32| -> Result<f64, String> { Ok((x * 2) as f64) };
        let g_inner = |y: f64| -> Result<String, String> { Ok(y.to_string()) };
        let inner_closure = move |x: i32| (CFn::new(f_inner))(x).bind(CFn::new(g_inner));
        let rhs = m.bind(CFn::new(inner_closure));

        assert_eq!(lhs, rhs);
        assert_eq!(lhs, Err("m_error".to_string()));
    }

    #[test]
    fn result_monad_associativity_f_returns_err() {
        let m: Result<i32, String> = Ok(10);
        let f = |_x: i32| -> Result<f64, String> { Err("f_error".to_string()) };
        let g = |y: f64| -> Result<String, String> { Ok(y.to_string()) };

        let lhs = m.clone().bind(CFn::new(f)).bind(CFn::new(g));
        
        let f_inner = |_x: i32| -> Result<f64, String> { Err("f_error".to_string()) };
        let g_inner = |y: f64| -> Result<String, String> { Ok(y.to_string()) };
        let inner_closure = move |x: i32| (CFn::new(f_inner))(x).bind(CFn::new(g_inner));
        let rhs = m.bind(CFn::new(inner_closure));

        assert_eq!(lhs, rhs);
        assert_eq!(lhs, Err("f_error".to_string()));
    }

    #[test]
    fn result_monad_associativity_g_returns_err() {
        let m: Result<i32, String> = Ok(10);
        let f = |x: i32| -> Result<f64, String> { Ok((x * 2) as f64) };
        let g = |_y: f64| -> Result<String, String> { Err("g_error".to_string()) };

        let lhs = m.clone().bind(CFn::new(f)).bind(CFn::new(g));
        
        let f_inner = |x: i32| -> Result<f64, String> { Ok((x * 2) as f64) };
        let g_inner = |_y: f64| -> Result<String, String> { Err("g_error".to_string()) };
        let inner_closure = move |x: i32| (CFn::new(f_inner))(x).bind(CFn::new(g_inner));
        let rhs = m.bind(CFn::new(inner_closure));

        assert_eq!(lhs, rhs);
        assert_eq!(lhs, Err("g_error".to_string()));
    }
}
