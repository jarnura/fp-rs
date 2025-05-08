// Original content from src/applicative.rs test modules
// with use statements adjusted for the new location.

use fp_rs::Applicative;
 // For map law test if Functor::map is used directly
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
    use fp_rs::function::CFn;
    use fp_rs::Applicative;
    use fp_rs::Apply;
    #[allow(unused_imports)] // Suppress incorrect warning; import needed for .map()
    use fp_rs::Functor; // For map if used directly // Restoring import

    // Helper for identity function
    fn identity<T>(x: T) -> T {
        x
    }

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

        assert_eq!(
            pure_x.clone().apply(Option::pure(CFn::new(f))),
            Option::pure(f(x))
        );
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
    fn identity<T>(x: T) -> T {
        x
    }

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

        assert_eq!(
            pure_x.clone().apply(Result::pure(CFn::new(f))),
            Result::pure(f(x))
        );
        assert_eq!(
            Result::pure(x).apply(Result::pure(CFn::new(f))),
            Ok::<i32, String>(20)
        );
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

#[cfg(test)]
mod vec_applicative_laws {
    use fp_rs::function::CFn;
    use fp_rs::{Applicative, Apply, Functor};

    fn identity<T: Clone>(x: T) -> T {
        x.clone()
    } // Ensure T is Clone for identity

    // 1. Identity law: v.apply(Vec::pure(identity_fn)) == v
    #[test]
    fn vec_applicative_identity_non_empty() {
        let v = vec![10, 20];
        // CFn::new(identity::<i32>) is not Clone. Vec::pure cannot be used on CFn.
        // Construct the function vector manually.
        let pure_identity_fn_vec = vec![CFn::new(identity::<i32>)];
        assert_eq!(v.clone().apply(pure_identity_fn_vec), v);
    }

    #[test]
    fn vec_applicative_identity_empty() {
        let v: Vec<i32> = vec![];
        let pure_identity_fn_vec = vec![CFn::new(identity::<i32>)];
        assert_eq!(v.clone().apply(pure_identity_fn_vec), v);
    }

    // 2. Homomorphism law: Vec::pure(x).apply(Vec::pure(f)) == Vec::pure(f(x))
    #[test]
    fn vec_applicative_homomorphism() {
        let x = 10;
        let f = |y: i32| y * 2;
        let pure_x_vec = Vec::pure(x); // vec![10]

        // Vec::pure(CFn::new(f)) is invalid as CFn is not Clone. Construct manually.
        let pure_f_vec = vec![CFn::new(f)];
        // lhs: vec![10].apply(vec![CFn::new(f)]) -> vec![ f(10) ] -> vec![20]
        let lhs = pure_x_vec.apply(pure_f_vec);
        // rhs: Vec::pure(f(x)) -> vec![ f(10) ] -> vec![20]
        let rhs = Vec::pure(f(x));
        assert_eq!(lhs, rhs);
        assert_eq!(lhs, vec![20]);
    }

    // 3. Interchange law: Vec::pure(y).apply(u) == u.apply(Vec::pure(|f_prime| f_prime(y)))
    //    where u is Vec<CFn<A,B>>
    //    and f_prime is CFn<A,B> -> B (effectively, it takes a function and applies y to it)
    #[test]
    fn vec_applicative_interchange() {
        let y = 10; // y: i32
        let add_5 = |x: i32| x + 5; // add_5: i32 -> i32
        let mul_2 = |x: i32| x * 2; // mul_2: i32 -> i32

        // u = vec![CFn::new(add_5), CFn::new(mul_2)]
        let _u: Vec<CFn<i32, i32>> = vec![CFn::new(add_5), CFn::new(mul_2)]; // Prefixed with _ as it's unused

        // lhs: Vec::pure(y).apply(u.clone())
        //      vec![10].apply(vec![CFn(add_5), CFn(mul_2)])
        //   -> vec![ add_5(10), mul_2(10) ] -> vec![15, 20]
        // To avoid u.clone(), we create u for lhs, and u_rhs for rhs if needed,
        // or ensure the operations don't require u to be cloned.
        // LHS consumes u. RHS also consumes its version of u.
        let u_lhs: Vec<CFn<i32, i32>> = vec![CFn::new(add_5), CFn::new(mul_2)];
        let lhs = Vec::pure(y).apply(u_lhs);

        // rhs: u.apply(Vec::pure( |f_prime: CFn<i32,i32>| (*f_prime)(y) ))
        //      u is vec![CFn(add_5), CFn(mul_2)]
        //      Vec::pure(...) is vec![ CFn::new( |f_prime| (*f_prime)(y) ) ]
        //      This inner CFn takes CFn<i32,i32> and returns i32.
        //      So, the type of f_prime_applier_fn is CFn<CFn<i32,i32>, i32>
        //      This means u should be Vec<CFn<CFn<i32,i32>, i32>> which is not what u is.
        //
        // The law is: pure y <*> u = u <*> pure ($ y)
        // where ($ y) is a function that takes another function `g` and applies `y` to it, i.e. `g(y)`.
        // So, `pure ($ y)` is `Vec::pure(CFn::new(move |g: CFn<i32,i32>| (*g)(y)))`
        // This means `u` (Vec<CFn<i32,i32>>) is applied to `Vec<CFn< (CFn<i32,i32>) , i32>>`
        // This doesn't type check directly with the current `apply` signature.
        //
        // Let's use the alternative formulation: pure y <*> u == u <**> pure y
        // where <**> is `map` essentially: u.map(|f_val| (*f_val)(y))
        // This requires `CFn` to be the item in `u` and `y` to be `Clone`.
        // `u.map(move |f_val: CFn<i32,i32>| (*f_val)(y.clone()))`
        // The `map` here is the Functor map.
        // `u` is `Vec<CFn<i32,i32>>`. `map` is on `Vec`.
        // The closure for map is `move |f_val: CFn<i32,i32>| (*f_val)(y.clone())`.
        // This closure takes `CFn<i32,i32>` (moved from `u`) and returns `i32`.
        // So `rhs` will be `Vec<i32>`.
        let y_cloned = y.clone();
        // u for RHS needs to be a new Vec<CFn> as the original u was consumed by map.
        let u_rhs: Vec<CFn<i32, i32>> = vec![CFn::new(add_5), CFn::new(mul_2)];
        let rhs = u_rhs.map(move |f_val: CFn<i32, i32>| (*f_val)(y_cloned.clone()));

        assert_eq!(lhs, rhs);
        assert_eq!(lhs, vec![15, 20]);
    }

    #[test]
    fn vec_applicative_interchange_empty_u() {
        let y = 10;
        let u_lhs: Vec<CFn<i32, i32>> = vec![];
        let u_rhs: Vec<CFn<i32, i32>> = vec![];

        let lhs = Vec::pure(y).apply(u_lhs);

        let y_cloned = y.clone();
        let rhs = u_rhs.map(move |f_val: CFn<i32, i32>| (*f_val)(y_cloned.clone()));

        assert_eq!(lhs, rhs);
        assert_eq!(lhs, Vec::<i32>::new());
    }

    // 4. Map law (derived): v.map(f) == v.apply(Vec::pure(f))
    #[test]
    fn vec_applicative_map_non_empty() {
        let v = vec![10, 20]; // Vec<i32>
        let f = |x: i32| x.to_string(); // i32 -> String

        // lhs: v.map(CFn::new(f)) -> Vec<String>
        // Need to ensure f is Copy or provide a new one for map if it's consumed.
        // Closures like |x| x.to_string() are typically Copy.
        let lhs = v.clone().map(f);

        // rhs: v.apply(Vec::pure(CFn::new(f))) -> invalid, construct manually
        //      vec![10,20].apply(vec![CFn(f)])
        //   -> vec![ f(10), f(20) ] -> vec!["10", "20"]
        let pure_f_vec = vec![CFn::new(f)];
        let rhs = v.apply(pure_f_vec);

        assert_eq!(lhs, rhs);
        assert_eq!(lhs, vec!["10".to_string(), "20".to_string()]);
    }

    #[test]
    fn vec_applicative_map_empty() {
        let v: Vec<i32> = vec![];
        let f = |x: i32| x.to_string();

        let lhs = v.clone().map(f);
        // rhs: v.apply(Vec::pure(CFn::new(f))) -> invalid, construct manually
        let pure_f_vec = vec![CFn::new(f)];
        let rhs = v.apply(pure_f_vec);

        assert_eq!(lhs, rhs);
        assert_eq!(lhs, Vec::<String>::new());
    }
}
