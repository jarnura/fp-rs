// Original content from src/profunctor.rs mod tests
// with use statements adjusted for the new location.

// Items re-exported from lib.rs
use fp_rs::{Profunctor, Strong, Choice}; // These are re-exported

// Items specific to the profunctor module
use fp_rs::profunctor::{
    view, _1, _2, _key, lcmap, rmap, lens, lens_, Forget, Optic, Lens, Check, // Added Check struct
    // CFn, CFnOnce are from function module, but Profunctor impls for them are in profunctor.rs
    // They are not directly used in tests, but the Profunctor trait methods are.
};

// Items from other modules
use fp_rs::fn1; // Macro is at crate root


#[cfg(test)]
mod tests {
    // Bring all top-level imports from this file into the module's scope
    use super::*;

    #[test]
    fn test_fn_dimap() {
        let closure = fn1!(|x: i32| format!("{x}"));
        // Assuming Profunctor is in scope via `use fp_rs::Profunctor;` at the top of this test file
        // or `use super::Profunctor;` if it were defined in this file's parent.
        // Since Profunctor is a trait, `closure.dimap` should work if `closure` (a CFn) implements Profunctor.
        let proclosure = closure.dimap(|x: i8| (x + 1).into(), |s| vec![s]);
        let result = proclosure(1);
        assert_eq!(result, vec!["2"])
    }

    #[test]
    fn test_fn_lcmap() {
        let profunctor_val = fn1!(|x: i32| format!("{x}")); // Renamed from profunctor to avoid conflict
        let proclosure = lcmap(|x: i8| x as i32 + 1, profunctor_val);
        let result = proclosure(1);
        assert_eq!(result, "2")
    }

    #[test]
    fn test_fn_rmap() {
        let profunctor_val = fn1!(|x: i32| format!("{x}")); // Renamed
        let proclosure = rmap(|s| vec![s], profunctor_val);
        let result = proclosure(1);
        assert_eq!(result, vec!["1"])
    }

    #[test]
    fn test_fn_rmap_with_identity() {
        let profunctor_val = fn1!(|x: i32| x); // Renamed
        let proclosure = rmap(|s| vec![s], profunctor_val);
        let result = proclosure(1);
        assert_eq!(result, vec![1])
    }

    #[test]
    fn test_1() {
        let tuple = (1, 3);
        // The AGetter type alias might need to be defined or imported if it's not automatically resolved.
        // For now, assuming view can infer types or AGetter is accessible.
        // AGetter is `Fold<A, S, T, A, B>`, Fold is `Optic<Forget<R,S,T>, Forget<R,A,B>, S,T,A,B>`
        // These are complex types defined in profunctor.rs.
        // The `_1()` function returns a `Lens`. `into()` converts it to `Optic`.
        // This should work if `Lens`, `Optic`, `Forget`, `view`, `_1`, `_2` are in scope.
        let r = view::<_, _, _, ()>(_1().into(), tuple);
        assert_eq!(r, 1);
        let r = view::<_, _, _, ()>(_2().into(), tuple);
        assert_eq!(r, 3)
    }

    #[test]
    fn test_key() {
        let rec = Check { key: 1, other: 1}; // Check struct needs to be in scope
        let r = view(_key().0, rec); // _key() returns Lens, .0 accesses the Optic inside
        assert_eq!(r, 1);
    }
}
