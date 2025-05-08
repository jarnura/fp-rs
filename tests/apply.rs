// Original content from src/apply.rs mod tests
// with use statements adjusted for the new location.

// Assuming fn2 and fn3 macros are exported from the crate root (e.g. from function.rs via #[macro_export])
// and Apply, Functor are re-exported from lib.rs.
// lift2, lift3 are specific to the apply module.

use fp_rs::apply::{lift2, lift3}; // lift2, lift3 are in the apply module
use fp_rs::Apply; // Apply is re-exported
#[allow(unused_imports)] // Suppress incorrect warning; import needed for .map()
use fp_rs::Functor; // Functor is re-exported // Restoring import
use fp_rs::{fn2, fn3}; // Macros are at crate root

#[cfg(test)]
mod tests {
    // Use the more specific imports from above
    use super::*; // This will pull in the fp_rs::... imports from the parent scope of this file

    #[test]
    fn apply_on_option() {
        let closure = fn2!(|x: i32| move |y: i8| format!("{x}{y}"));
        let some_closure = Some(1).map(closure);
        let none_closure = None.map(closure); // Assuming Functor<A> for Option<A> handles None.map correctly
        assert_eq!(Some(2).apply(some_closure), Some("12".to_string()));
        assert_eq!(Some(2).apply(none_closure), None);

        let closure = fn2!(|x: i32| move |y: i8| format!("{x}{y}"));
        assert_eq!(lift2(closure, Some(1), Some(2)), Some("12".to_string()));
        assert_eq!(lift2(closure, None, Some(2)), None);

        let closure = fn3!(|x: i32| move |y: i8| move |z: i32| x + y as i32 + z);

        assert_eq!(lift3(closure, Some(1), Some(2), Some(3)), Some(6));
    }
}
