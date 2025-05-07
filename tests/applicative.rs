// Original content from src/applicative.rs test modules
// with use statements adjusted for the new location.

use fp_rs::Applicative;
use fp_rs::Apply;
use fp_rs::Functor; // For map law test if Functor::map is used directly
use fp_rs::function::CFn;
// lift_a1 is defined in src/applicative.rs (and thus in the fp_rs::applicative module)
use fp_rs::applicative::lift_a1;

// Re-checking lift_a1 location: it's defined in src/applicative.rs, so it will be fp_rs::applicative::lift_a1
// However, the test `test_lift_a1` uses `lift_a1` directly after `use super::*;`.
// So, in the new test file, it should be `use fp_rs::applicative::lift_a1;` at the top,
// or `use fp_rs::applicative::*;` if many items are needed.
// Let's be specific.

// Corrected imports for the test file structure:
// Items from fp_rs (re-exported from lib.rs)
// Items from fp_rs::module_name (specific modules)

#[cfg(test)]
mod tests {
    // Use imports from the top of the file via super::*
    use super::*;

    #[test]
    fn check_some() {
        assert_eq!(Option::pure(1), Some(1))
    }

    #[test]
    fn test_lift_a1() {
        let c = |x: i32| format!("{x}");
        let result = lift_a1(c, Some(1));
        assert_eq!(result, Some("1".to_string()))
    }
}


#[cfg(test)]
mod applicative_laws {
    use fp_rs::Applicative;
    use fp_rs::Apply;
    use fp_rs::Functor; // For map if used directly
    use fp_rs::function::CFn;

    // Helper for identity function
    fn identity<T>(x: T) -> T { x }


    // 1. Identity law: v.apply(Option::pure(identity)) == v
    #[test]
    fn option_applicative_identity_some() {
        let v = Some(10);
        assert_eq!(v.apply(Option::pure(CFn::new(identity::<i32>))), v);
    }

    #[test]
    fn option_applicative_identity_none() {
        let v: Option<i32> = None;
        assert_eq!(v.apply(Option::pure(CFn::new(identity::<i32>))), v);
    }

    // 2. Homomorphism law: Option::pure(x).apply(Option::pure(f)) == Option::pure(f(x))
    #[test]
    fn option_applicative_homomorphism() {
        let x = 10;
        let f = |y: i32| y * 2;
        let pure_x = Option::pure(x);

        assert_eq!(pure_x.clone().apply(Option::pure(CFn::new(f))), Option::pure(f(x)));
        assert_eq!(pure_x.apply(Option::pure(CFn::new(f))), Some(20));
    }

    // 3. Interchange law: Option::pure(y).apply(u) == u.map(|f_ref| (*f_ref)(y))
    #[test]
    fn option_applicative_interchange_some_fn() {
        let y = 10;
        let f = |x: i32| x + 5;
        
        let lhs = Option::pure(y).apply(Some(CFn::new(f)));

        let u_for_rhs = Some(CFn::new(f));
        let eval_at_y = move |f_func: CFn<i32, i32>| (*f_func)(y);
        let rhs_interchange = u_for_rhs.map(eval_at_y); 

        assert_eq!(lhs, rhs_interchange);
        assert_eq!(lhs, Some(15));
    }

     #[test]
    fn option_applicative_interchange_none_fn() {
        let y = 10;
        let u: Option<CFn<i32, i32>> = None;

        let lhs = Option::pure(y).apply(None::<CFn<i32, i32>>);

        let eval_at_y = move |f_func: CFn<i32, i32>| (*f_func)(y);
        let rhs_interchange = u.map(eval_at_y);

        assert_eq!(lhs, rhs_interchange);
        assert_eq!(lhs, None);
    }


    // 4. Map law (derived): v.map(f) == v.apply(Option::pure(f))
    #[test]
    fn option_applicative_map_some() {
        let v = Some(10);
        let f = |x: i32| x.to_string();
        let pure_f = Option::pure(CFn::new(f));

        let lhs = v.map(f);
        let rhs = Some(10).apply(pure_f);

        assert_eq!(lhs, rhs);
        assert_eq!(lhs, Some("10".to_string()));
    }

     #[test]
    fn option_applicative_map_none() {
        let v: Option<i32> = None;
        let f = |x: i32| x.to_string();
        let pure_f = Option::pure(CFn::new(f));

        let lhs = v.map(f);
        let rhs = None::<i32>.apply(pure_f);

        assert_eq!(lhs, rhs);
        assert_eq!(lhs, None);
    }
}

#[cfg(test)]
mod result_applicative_laws {
    use fp_rs::Applicative;
    use fp_rs::Apply;
    // use fp_rs::Functor; // Not directly used if map law uses apply
    use fp_rs::function::CFn;

    // Helper for identity function
    fn identity<T>(x: T) -> T { x }

    // 1. Identity law: v.apply(Result::pure(identity)) == v
    #[test]
    fn result_applicative_identity_ok() {
        let v: Result<i32, String> = Ok(10);
        assert_eq!(v.clone().apply(Result::pure(CFn::new(identity::<i32>))), v);
    }

    #[test]
    fn result_applicative_identity_err() {
        let v: Result<i32, String> = Err("error".to_string());
        assert_eq!(v.clone().apply(Result::pure(CFn::new(identity::<i32>))), v);
    }

    #[test]
    fn result_applicative_identity_ok_apply_err() {
        let v: Result<i32, String> = Ok(10);
        let f_err: Result<CFn<i32, i32>, String> = Err("function error".to_string());
        assert_eq!(v.apply(f_err), Err("function error".to_string()));
    }

    // 2. Homomorphism law: Result::pure(x).apply(Result::pure(f)) == Result::pure(f(x))
    #[test]
    fn result_applicative_homomorphism_ok() {
        let x = 10;
        let f = |y: i32| y * 2;
        let pure_x: Result<i32, String> = Result::pure(x);

        assert_eq!(pure_x.clone().apply(Result::pure(CFn::new(f))), Result::pure(f(x)));
        assert_eq!(Result::pure(x).apply(Result::pure(CFn::new(f))), Ok::<i32, String>(20));
    }

    #[test]
    fn result_applicative_homomorphism_err_val() {
        let x = 10;
        let _f = |y: i32| y * 2;
        let pure_f_err: Result<CFn<i32, i32>, String> = Err("function error".to_string());
        let pure_x: Result<i32, String> = Result::pure(x);
        assert_eq!(pure_x.apply(pure_f_err), Err("function error".to_string()));
    }


    // 3. Interchange law: Result::pure(y).apply(u) == u.map(|f_ref| (*f_ref)(y))
    //    where u is Result<CFn<A,B>, E>
    #[test]
    fn result_applicative_interchange_ok_fn() {
        let y = 10;
        let f = move |x: i32| x + y;
        
        let lhs = Result::pure(y).apply(Ok(CFn::new(f)));

        let u_for_rhs: Result<CFn<i32, i32>, String> = Ok(CFn::new(f));
        let rhs = u_for_rhs.map(move |f_func: CFn<i32, i32>| (*f_func)(y)); 

        assert_eq!(lhs, rhs);
        assert_eq!(lhs, Ok(20));
    }

    #[test]
    fn result_applicative_interchange_err_fn() {
        let y = 10;
        let u: Result<CFn<i32, i32>, String> = Err("function error".to_string());

        let lhs = Result::pure(y).apply(Err("function error".to_string()));
        let rhs = u.map(move |f_func: CFn<i32, i32>| (*f_func)(y)); 
        
        assert_eq!(lhs, rhs);
        assert_eq!(lhs, Err("function error".to_string()));
    }

    // 4. Map law (derived): v.map(f) == v.apply(Result::pure(CFn::new(f)))
    #[test]
    fn result_applicative_map_ok() {
        let v: Result<i32, String> = Ok(10);
        let f = |x: i32| x.to_string();
        
        let lhs = v.clone().map(f); 
        let rhs = v.apply(Result::pure(CFn::new(f)));

        assert_eq!(lhs, rhs);
        assert_eq!(lhs, Ok("10".to_string()));
    }

    #[test]
    fn result_applicative_map_err() {
        let v: Result<i32, String> = Err("error".to_string());
        let f = |x: i32| x.to_string();

        let lhs = v.clone().map(f);
        let rhs = v.apply(Result::pure(CFn::new(f)));

        assert_eq!(lhs, rhs);
        assert_eq!(lhs, Err("error".to_string()));
    }
}
