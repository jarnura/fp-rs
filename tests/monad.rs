// Original content from src/monad.rs test modules
// with use statements adjusted for the new location.

use fp_rs::monad::{join, Bind}; // bind is also a free function in monad.rs
use fp_rs::fn1; // Macros are at crate root

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
        let add_one = fn1!(|x: i32| Some(x + 1)); // Changed bfn1! to fn1!
        let add_two = fn1!(|x: i32| Some(x + 2)); // Changed bfn1! to fn1!
        let add_three = fn1!(|x: i32| Some(x + 3)); // Changed bfn1! to fn1!
        // Changed from BitOr operator to direct bind calls
        let result = Some(1).bind(add_one).bind(add_two).bind(add_three);
        assert_eq!(result, Some(7))
    }
}

#[cfg(test)]
mod monad_laws {
    use fp_rs::monad::Bind; // Bind is re-exported from lib.rs, but also defined in monad.rs
                            // Using fp_rs::Bind should be fine as it's re-exported.
    use fp_rs::function::CFn;
    use fp_rs::Applicative; // For pure // For wrapping functions for bind

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
    use fp_rs::function::CFn;
    use fp_rs::monad::Bind;
    use fp_rs::Applicative;

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

#[cfg(test)]
mod vec_monad_laws {
    use fp_rs::function::CFn;
    use fp_rs::{Applicative, Bind}; // Use Bind directly

    // 1. Left Identity: Vec::pure(a).bind(f) == f(a)
    #[test]
    fn vec_monad_left_identity() {
        let a = 10; // Type: i32
                    // f: i32 -> Vec<String>
        let f = |x: i32| -> Vec<String> { vec![x.to_string(), (x + 1).to_string()] };

        // lhs: Vec::pure(a).bind(CFn::new(f))
        //      vec![10].bind(CFn::new(f)) -> f(10) -> vec!["10", "11"]
        let lhs = Vec::pure(a).bind(CFn::new(f));

        // rhs: f(a) -> vec!["10", "11"]
        let rhs = f(a);

        assert_eq!(lhs, rhs);
        assert_eq!(lhs, vec!["10".to_string(), "11".to_string()]);
    }

    #[test]
    fn vec_monad_left_identity_f_returns_empty() {
        let a = 10;
        // f: i32 -> Vec<String>
        let f = |_x: i32| -> Vec<String> { vec![] };

        let lhs = Vec::pure(a).bind(CFn::new(f));
        let rhs = f(a);

        assert_eq!(lhs, rhs);
        assert_eq!(lhs, Vec::<String>::new());
    }

    // 2. Right Identity: m.bind(Vec::pure) == m
    #[test]
    fn vec_monad_right_identity_non_empty() {
        let m = vec![10, 20]; // Vec<i32>
                              // pure_fn: i32 -> Vec<i32>
        let pure_fn = Vec::pure as fn(i32) -> Vec<i32>;

        // lhs: m.bind(CFn::new(pure_fn))
        //      vec![10, 20].bind(CFn::new(Vec::pure))
        //   -> pure_fn(10).extend(pure_fn(20))
        //   -> vec![10].extend(vec![20]) -> vec![10, 20]
        let lhs = m.clone().bind(CFn::new(pure_fn));
        let rhs = m; // vec![10, 20]

        assert_eq!(lhs, rhs);
    }

    #[test]
    fn vec_monad_right_identity_empty() {
        let m: Vec<i32> = vec![];
        let pure_fn = Vec::pure as fn(i32) -> Vec<i32>;

        let lhs = m.clone().bind(CFn::new(pure_fn));
        let rhs = m;

        assert_eq!(lhs, rhs);
        assert_eq!(lhs, Vec::<i32>::new());
    }

    // 3. Associativity: m.bind(f).bind(g) == m.bind(|x| f(x).bind(g))
    #[test]
    fn vec_monad_associativity() {
        let m = vec![1, 2]; // Vec<i32>
                            // f: i32 -> Vec<i32>
        let f = |x: i32| -> Vec<i32> { vec![x, x * 10] };
        // g: i32 -> Vec<String>
        let g = |y: i32| -> Vec<String> { vec![y.to_string(), (y + 1).to_string()] };

        // lhs: m.bind(f).bind(g)
        //      m.bind(f) -> vec![1, 10, 2, 20]
        //      vec![1, 10, 2, 20].bind(g)
        //   -> g(1) -> vec!["1", "2"]
        //   -> g(10) -> vec!["10", "11"]
        //   -> g(2) -> vec!["2", "3"]
        //   -> g(20) -> vec!["20", "21"]
        //   -> vec!["1", "2", "10", "11", "2", "3", "20", "21"]
        let lhs = m.clone().bind(CFn::new(f)).bind(CFn::new(g));

        // rhs: m.bind(|x| f(x).bind(g))
        //      inner_closure = |x: i32| f(x).bind(g)
        //      inner_closure(1): f(1).bind(g) -> vec![1, 10].bind(g) -> g(1) ++ g(10) -> vec!["1", "2", "10", "11"]
        //      inner_closure(2): f(2).bind(g) -> vec![2, 20].bind(g) -> g(2) ++ g(20) -> vec!["2", "3", "20", "21"]
        //      m.bind(inner_closure) -> inner_closure(1) ++ inner_closure(2)
        //   -> vec!["1", "2", "10", "11", "2", "3", "20", "21"]
        let f_inner = |x: i32| -> Vec<i32> { vec![x, x * 10] };
        let g_inner = |y: i32| -> Vec<String> { vec![y.to_string(), (y + 1).to_string()] };
        // Need CFn for the inner bind call as well
        let inner_closure = move |x: i32| (CFn::new(f_inner))(x).bind(CFn::new(g_inner));
        let rhs = m.bind(CFn::new(inner_closure));

        assert_eq!(lhs, rhs);
        assert_eq!(
            lhs,
            vec![
                "1".to_string(),
                "2".to_string(),
                "10".to_string(),
                "11".to_string(),
                "2".to_string(),
                "3".to_string(),
                "20".to_string(),
                "21".to_string()
            ]
        );
    }

    #[test]
    fn vec_monad_associativity_empty_start() {
        let m: Vec<i32> = vec![];
        let f = |x: i32| -> Vec<i32> { vec![x, x * 10] };
        let g = |y: i32| -> Vec<String> { vec![y.to_string(), (y + 1).to_string()] };

        let lhs = m.clone().bind(CFn::new(f)).bind(CFn::new(g));

        let f_inner = |x: i32| -> Vec<i32> { vec![x, x * 10] };
        let g_inner = |y: i32| -> Vec<String> { vec![y.to_string(), (y + 1).to_string()] };
        let inner_closure = move |x: i32| (CFn::new(f_inner))(x).bind(CFn::new(g_inner));
        let rhs = m.bind(CFn::new(inner_closure));

        assert_eq!(lhs, rhs);
        assert_eq!(lhs, Vec::<String>::new());
    }

    #[test]
    fn vec_monad_associativity_f_returns_empty() {
        let m = vec![1, 2];
        let f = |_x: i32| -> Vec<i32> { vec![] };
        let g = |y: i32| -> Vec<String> { vec![y.to_string(), (y + 1).to_string()] };

        let lhs = m.clone().bind(CFn::new(f)).bind(CFn::new(g));

        let f_inner = |_x: i32| -> Vec<i32> { vec![] };
        let g_inner = |y: i32| -> Vec<String> { vec![y.to_string(), (y + 1).to_string()] };
        let inner_closure = move |x: i32| (CFn::new(f_inner))(x).bind(CFn::new(g_inner));
        let rhs = m.bind(CFn::new(inner_closure));

        assert_eq!(lhs, rhs);
        assert_eq!(lhs, Vec::<String>::new());
    }

    #[test]
    fn vec_monad_associativity_g_returns_empty() {
        let m = vec![1, 2];
        let f = |x: i32| -> Vec<i32> { vec![x, x * 10] };
        let g = |_y: i32| -> Vec<String> { vec![] };

        let lhs = m.clone().bind(CFn::new(f)).bind(CFn::new(g));

        let f_inner = |x: i32| -> Vec<i32> { vec![x, x * 10] };
        let g_inner = |_y: i32| -> Vec<String> { vec![] };
        let inner_closure = move |x: i32| (CFn::new(f_inner))(x).bind(CFn::new(g_inner));
        let rhs = m.bind(CFn::new(inner_closure));

        assert_eq!(lhs, rhs);
        assert_eq!(lhs, Vec::<String>::new());
    }
}
