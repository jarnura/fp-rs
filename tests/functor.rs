// Original content from src/functor.rs mod tests, functor_laws, and result_functor_laws
// with `use super::Functor` changed to `use fp_rs::Functor`

// Note: The `crate::function::CFn` import was commented out in the original functor_laws,
// so it's not included here unless it was actually used.
// If other `crate::` or `super::` imports were present and used, they'd be adjusted similarly.
// For example, `use crate::function::CFn;` would become `use fp_rs::function::CFn;`

#[cfg(test)]
mod tests {
    use fp_rs::Functor; // Changed from super::Functor

    #[test]
    fn add_one() {
        let closure = |x| x + 1;
        assert_eq!(Some(1).map(closure), Some(2))
    }
}

#[cfg(test)]
mod functor_laws {
    use fp_rs::Functor; // Changed from super::Functor
    // use fp_rs::function::CFn; // Example if CFn was from fp_rs::function and used

    // Identity law: functor.map(identity) == identity(functor)
    // For Option, this means: opt.map(|x| x) == opt

    #[test]
    fn option_functor_identity_some() {
        let opt = Some(10);
        let identity_fn = |x: i32| x;
        assert_eq!(opt.map(identity_fn), opt);
    }

    #[test]
    fn option_functor_identity_none() {
        let opt: Option<i32> = None;
        let identity_fn = |x: i32| x;
        assert_eq!(opt.map(identity_fn), opt);
    }

    // Composition law: functor.map(g . f) == functor.map(f).map(g)
    // For Option: opt.map(|x| g(f(x))) == opt.map(f).map(g)

    #[test]
    fn option_functor_composition_some() {
        let opt = Some(10);
        let f = |x: i32| x * 2; // First function
        let g = |y: i32| y + 5; // Second function

        // opt.map(|x| g(f(x)))
        let composed_map = opt.map(|x| g(f(x)));

        // opt.map(f).map(g)
        let sequential_map = opt.map(f).map(g);

        assert_eq!(composed_map, sequential_map);
        assert_eq!(composed_map, Some(25)); // 10 * 2 = 20; 20 + 5 = 25
    }

    #[test]
    fn option_functor_composition_none() {
        let opt: Option<i32> = None;
        let f = |x: i32| x * 2;
        let g = |y: i32| y + 5;

        let composed_map = opt.map(|x| g(f(x)));
        let sequential_map = opt.map(f).map(g);

        assert_eq!(composed_map, sequential_map);
        assert_eq!(composed_map, None);
    }

    // Test with different types
    #[test]
    fn option_functor_composition_some_str() {
        let opt = Some("hello");
        let f = |x: &str| x.to_uppercase(); // &str -> String
        let g = |y: String| y.len();      // String -> usize

        // opt.map(|x| g(f(x)))
        let composed_map = opt.map(|x| g(f(x)));

        // opt.map(f).map(g)
        let sequential_map = opt.map(f).map(g);
        
        assert_eq!(composed_map, sequential_map);
        assert_eq!(composed_map, Some(5)); // "HELLO".len() = 5
    }
}

#[cfg(test)]
mod result_functor_laws {
    use fp_rs::Functor; // Changed from super::Functor

    // Identity law: functor.map(identity) == identity(functor)
    // For Result, this means: res.map(|x| x) == res

    #[test]
    fn result_functor_identity_ok() {
        let res: Result<i32, String> = Ok(10);
        let identity_fn = |x: i32| x;
        assert_eq!(res.map(identity_fn), Ok(10));
    }

    #[test]
    fn result_functor_identity_err() {
        let res: Result<i32, String> = Err("error".to_string());
        let identity_fn = |x: i32| x;
        assert_eq!(res.map(identity_fn), Err("error".to_string()));
    }

    // Composition law: functor.map(g . f) == functor.map(f).map(g)
    // For Result: res.map(|x| g(f(x))) == res.map(f).map(g)

    #[test]
    fn result_functor_composition_ok() {
        let res: Result<i32, String> = Ok(10);
        let f = |x: i32| x * 2; // First function
        let g = |y: i32| y + 5; // Second function

        // res.map(|x| g(f(x)))
        let composed_map = res.clone().map(|x| g(f(x))); // Clone res

        // res.map(f).map(g)
        let sequential_map = res.map(f).map(g); // Original res consumed here

        assert_eq!(composed_map, sequential_map);
        assert_eq!(composed_map, Ok(25)); // 10 * 2 = 20; 20 + 5 = 25
    }

    #[test]
    fn result_functor_composition_err() {
        let res: Result<i32, String> = Err("error".to_string());
        let f = |x: i32| x * 2;
        let g = |y: i32| y + 5;

        let composed_map = res.clone().map(|x| g(f(x))); // Clone res
        let sequential_map = res.map(f).map(g); // Original res consumed here

        assert_eq!(composed_map, sequential_map);
        assert_eq!(composed_map, Err("error".to_string()));
    }

    // Test with different types for success and error
    #[test]
    fn result_functor_composition_ok_str_err_u32() {
        let res: Result<&str, u32> = Ok("hello");
        let f = |x: &str| x.to_uppercase(); // &str -> String
        let g = |y: String| y.len();      // String -> usize

        let composed_map = res.clone().map(|x| g(f(x))); // Clone res
        let sequential_map = res.map(f).map(g); // Original res consumed here
        
        assert_eq!(composed_map, sequential_map);
        assert_eq!(composed_map, Ok(5)); // "HELLO".len() = 5
    }

    #[test]
    fn result_functor_composition_err_str_err_u32() {
        let res: Result<&str, u32> = Err(404);
        let f = |x: &str| x.to_uppercase();
        let g = |y: String| y.len();

        let composed_map = res.clone().map(|x| g(f(x))); // Clone res
        let sequential_map = res.map(f).map(g); // Original res consumed here

        assert_eq!(composed_map, sequential_map);
        assert_eq!(composed_map, Err(404));
    }
}
