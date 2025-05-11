// Original content from src/apply.rs mod tests
// with use statements adjusted for the new location.

// Assuming fn2 and fn3 macros are exported from the crate root (e.g. from function.rs via #[macro_export])
// and Apply, Functor are re-exported from lib.rs.
// lift2, lift3 are specific to the apply module.

// These imports will need to point to legacy versions.
// For now, using placeholder paths that will be fixed in Phase 5.
// use fp_rs::legacy_apply::{lift2, lift3};
// use fp_rs::legacy_functor::Functor;
// use fp_rs::{fn2, fn3}; // Assuming these macros are accessible from crate root

#[cfg(test)]
mod classic_apply_tests {
    // The lift2, lift3 functions are from crate::legacy::apply
    // The Apply and Functor traits are from crate::legacy::apply and crate::legacy::functor
    // Macros fn2, fn3 are assumed to be available via crate::fn2, crate::fn3

    #[test]
    fn apply_on_option() {
        let closure = fp_rs::fn2!(|x: i32| move |y: i8| format!("{x}{y}"));
        // Option::map uses Functor trait
        let some_closure = <Option<i32> as fp_rs::legacy::functor::Functor<i32>>::map(Some(1), closure.clone());
        let none_closure = <Option<i32> as fp_rs::legacy::functor::Functor<i32>>::map(None, closure);
        
        // Option::apply uses Apply trait
        assert_eq!(<Option<i8> as fp_rs::legacy::apply::Apply<i8>>::apply(Some(2), some_closure), Some("12".to_string()));
        assert_eq!(<Option<i8> as fp_rs::legacy::apply::Apply<i8>>::apply(Some(2), none_closure), None);

        let closure_lift = fp_rs::fn2!(|x: i32| move |y: i8| format!("{x}{y}"));
        assert_eq!(fp_rs::legacy::apply::lift2(closure_lift.clone(), Some(1), Some(2)), Some("12".to_string()));
        assert_eq!(fp_rs::legacy::apply::lift2(closure_lift, None, Some(2)), None);

        let closure_lift3 = fp_rs::fn3!(|x: i32| move |y: i8| move |z: i32| x + y as i32 + z);
        assert_eq!(fp_rs::legacy::apply::lift3(closure_lift3, Some(1), Some(2), Some(3)), Some(6));
    }
}
