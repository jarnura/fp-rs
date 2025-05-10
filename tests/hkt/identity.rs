#![cfg(all(test, feature = "kind"))] // Ensure these run only when 'kind' IS active

// Original imports from src/identity.rs hkt_tests module, adjusted
use fp_rs::identity::{Identity, IdentityHKTMarker}; // Assuming these are re-exported
use fp_rs::functor::hkt::Functor;
use fp_rs::apply::hkt::Apply;
use fp_rs::applicative::hkt::Applicative;
use fp_rs::monad::hkt::{Bind, Monad};
use fp_rs::function::CFn;

// The hkt_tests module content
#[test]
fn test_identity_hkt_functor_map() {
    let id_val: Identity<String> = Identity(String::from("hello"));
    let mapped_id: Identity<usize> = IdentityHKTMarker::map(id_val, |s: String| s.len());
    assert_eq!(mapped_id, Identity(5));

    let id_num: Identity<i32> = Identity(10);
    let mapped_id_num: Identity<i32> = IdentityHKTMarker::map(id_num, |x| x * x);
    assert_eq!(mapped_id_num, Identity(100));
}

#[test]
fn test_identity_hkt_apply() {
    let id_val: Identity<i32> = Identity(5);
    let id_fn: Identity<CFn<i32, i32>> = Identity(CFn::new(|x| x * 2));
    let result: Identity<i32> = IdentityHKTMarker::apply(id_val, id_fn);
    assert_eq!(result, Identity(10));
}

#[test]
fn test_identity_hkt_applicative_pure() {
    let pure_val: Identity<i32> = IdentityHKTMarker::pure(42);
    assert_eq!(pure_val, Identity(42));
}

#[test]
fn test_identity_hkt_monad_bind() {
    let id_val: Identity<i32> = Identity(3);
    let f = |x: i32| -> Identity<i32> { Identity(x + 7) };
    let result: Identity<i32> = IdentityHKTMarker::bind(id_val, f);
    assert_eq!(result, Identity(10));
}

#[test]
fn test_identity_hkt_monad_join() {
    let nested_id: Identity<Identity<i32>> = Identity(Identity(42));
    let joined_id: Identity<i32> = IdentityHKTMarker::join(nested_id);
    assert_eq!(joined_id, Identity(42));

    let nested_str_id: Identity<Identity<String>> = Identity(Identity(String::from("test")));
    let joined_str_id: Identity<String> = IdentityHKTMarker::join(nested_str_id);
    assert_eq!(joined_str_id, Identity(String::from("test")));
}

// Law tests for HKT version
#[test]
fn test_identity_hkt_left_identity() {
    let a = 10;
    let f = |x: i32| -> Identity<i32> { Identity(x * x) };
    let lhs: Identity<i32> = IdentityHKTMarker::bind(
        IdentityHKTMarker::pure(a), 
        f
    );
    let rhs: Identity<i32> = f(a);
    assert_eq!(lhs, rhs);
}

#[test]
fn test_identity_hkt_right_identity() {
    let m: Identity<i32> = Identity(20);
    let pure_fn = |x: i32| -> Identity<i32> { IdentityHKTMarker::pure(x) };
    let lhs = IdentityHKTMarker::bind(m.clone(), pure_fn);
    let rhs = m;
    assert_eq!(lhs, rhs);
}

#[test]
fn test_identity_hkt_associativity() {
    let m: Identity<i32> = Identity(5);
    let f = |x: i32| -> Identity<i32> { Identity(x + 1) };
    let g = |y: i32| -> Identity<i32> { Identity(y * 2) };

    let lhs = IdentityHKTMarker::bind(IdentityHKTMarker::bind(m.clone(), f), g);
    let rhs_fn = move |x: i32| -> Identity<i32> { IdentityHKTMarker::bind(f(x), g) };
    let rhs = IdentityHKTMarker::bind(m, rhs_fn);
    assert_eq!(lhs, rhs);
}
