#![cfg(all(test, not(feature = "kind")))] // Ensure these run only when 'kind' is NOT active

// Original imports from src/identity.rs, adjusted for tests directory
use fp_rs::identity::Identity; // Assuming Identity is re-exported from fp_rs::identity
use fp_rs::functor::Functor;
use fp_rs::apply::Apply;
use fp_rs::applicative::Applicative;
use fp_rs::monad::{Bind, Monad};
use fp_rs::function::CFn;

// The classic_tests module content
#[test]
fn test_identity_functor_map() {
    let id_val = Identity(String::from("hello"));
    let mapped_id = Functor::map(id_val, |s: String| s.len());
    assert_eq!(mapped_id, Identity(5));

    let id_num = Identity(10);
    let mapped_id_num = Functor::map(id_num, |x| x * x);
    assert_eq!(mapped_id_num, Identity(100));
}

#[test]
fn test_identity_apply() {
    let id_val = Identity(5);
    let id_fn: Identity<CFn<i32, i32>> = Identity(CFn::new(|x| x * 2));
    let result = Apply::apply(id_val, id_fn);
    assert_eq!(result, Identity(10));

    let id_str_val = Identity(String::from("test"));
    let id_str_fn: Identity<CFn<String, usize>> = Identity(CFn::new(|s: String| s.len()));
    let result_str = Apply::apply(id_str_val, id_str_fn);
    assert_eq!(result_str, Identity(4));
}

#[test]
fn test_identity_applicative_pure() {
    let pure_val: Identity<i32> = Applicative::pure(42);
    assert_eq!(pure_val, Identity(42));

    let pure_str: Identity<&str> = Applicative::pure("pure");
    assert_eq!(pure_str, Identity("pure"));
}

#[test]
fn test_identity_monad_bind() {
    let id_val = Identity(3);
    let f = |x: i32| Identity(x + 7);
    let result = Bind::bind(id_val, f);
    assert_eq!(result, Identity(10));

    let id_str = Identity(String::from("world"));
    let f_str = |s: String| Identity(format!("hello {}", s));
    let result_str = Bind::bind(id_str, f_str);
    assert_eq!(result_str, Identity(String::from("hello world")));
}

#[test]
fn test_identity_monad_left_identity() {
    let a = 10;
    let f = |x: i32| Identity(x * x);
    let lhs: Identity<i32> = Bind::bind(Applicative::pure(a), f);
    let rhs: Identity<i32> = f(a);
    assert_eq!(lhs, rhs);
}

#[test]
fn test_identity_monad_right_identity() {
    let m = Identity(20);
    let pure_fn = |x: i32| Applicative::pure(x);
    let lhs = Bind::bind(m.clone(), pure_fn);
    let rhs = m;
    assert_eq!(lhs, rhs);
}

#[test]
fn test_identity_monad_associativity() {
    let m = Identity(5);
    let f = |x: i32| Identity(x + 1);
    let g = |y: i32| Identity(y * 2);
    let lhs = Bind::bind(Bind::bind(m.clone(), f), g);
    let rhs_fn = move |x: i32| Bind::bind(f(x), g);
    let rhs = Bind::bind(m, rhs_fn);
    assert_eq!(lhs, rhs);
}
