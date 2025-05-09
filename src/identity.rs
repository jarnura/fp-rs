//! # Identity Monad
//!
//! This module provides the `Identity<A>` struct, which is the simplest possible monad.
//! It wraps a value `A` without adding any additional computational context or structure
//! beyond the monadic interface itself.
//!
//! ## Purpose
//!
//! The `Identity` monad serves several purposes:
//!
//! - **Base Case:** It can act as a base case for monad transformers. For example,
//!   `ReaderT<R, Identity, A>` simplifies to a basic `Reader<R, A>` monad.
//! - **Uniform Interface:** It allows treating plain values monadically, which can be
//!   useful in generic programming where a monadic structure is expected.
//! - **Simplicity:** It's a good starting point for understanding monads due to its
//!   straightforward implementation of `Functor`, `Apply`, `Applicative`, and `Monad` traits.
//!
//! ## Structure
//!
//! ```
//! pub struct Identity<A>(pub A);
//! ```
//!
//! It's a simple tuple struct holding a single value of type `A`.
//!
//! ## Trait Implementations
//!
//! - **`Functor`**: `map` applies a function directly to the wrapped value.
//! - **`Apply`**: `apply` takes an `Identity` wrapping a function and applies it to the
//!   `Identity` wrapping a value.
//! - **`Applicative`**: `pure` wraps a value in `Identity`.
//! - **`Monad`**: `bind` (or `flat_map`) takes a function that returns an `Identity` and
//!   applies it to the wrapped value, returning the result directly (as it's already an `Identity`).

use crate::functor::Functor;
use crate::apply::Apply;
use crate::applicative::Applicative;
use crate::monad::{Bind, Monad};
use crate::function::CFn; // For Fnn type

/// Represents a simple wrapper around a value, acting as the identity monad.
///
/// The `Identity` monad is the simplest monad, which does not add any computational
/// context beyond wrapping a value. It's often used as a base case for monad
/// transformers or when a monadic interface is required for a plain value.
///
/// The primary field `0` holds the wrapped value of type `A`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Identity<A>(pub A);

/// Implements the `Functor` trait for `Identity<A>`.
///
/// Mapping over an `Identity` simply applies the function to the wrapped value
/// and returns a new `Identity` with the result.
impl<A> Functor<A> for Identity<A> {
    type Functor<BVal> = Identity<BVal>;

    /// Applies a function `f` to the value wrapped in `Identity<A>`.
    ///
    /// The function `f` takes the wrapped value of type `A` and returns a new value
    /// of type `B`. This new value is then wrapped in `Identity<B>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fp_rs::functor::Functor;
    /// use fp_rs::identity::Identity;
    ///
    /// let id_val = Identity(5);
    /// let mapped_id = id_val.map(|x| x + 1);
    /// assert_eq!(mapped_id, Identity(6));
    ///
    /// let id_string = Identity(String::from("hello"));
    /// let length_id = id_string.map(|s| s.len());
    /// assert_eq!(length_id, Identity(5));
    /// ```
    fn map<B, F>(self, mut f: F) -> Self::Functor<B>
    where
        F: FnMut(A) -> B + 'static,
    {
        Identity(f(self.0))
    }
}

/// Implements the `Apply` trait for `Identity<A>`.
///
/// `apply` takes an `Identity` containing a function (`Identity<CFn<A, B>>`)
/// and applies this function to the value within the current `Identity<A>`.
impl<A: 'static> Apply<A> for Identity<A> {
    type Apply<T> = Identity<T>;
    type Fnn<T, U> = CFn<T, U>; // The function type within the Apply context.

    /// Applies a function wrapped in `Identity` to a value wrapped in `Identity`.
    ///
    /// Given `self` as `Identity(value)` and `i` as `Identity(function)`,
    /// this method returns `Identity(function(value))`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fp_rs::apply::Apply;
    /// use fp_rs::identity::Identity;
    /// use fp_rs::function::CFn;
    ///
    /// let id_val = Identity(10);
    /// let id_fn: Identity<CFn<i32, i32>> = Identity(CFn::new(|x| x * 2));
    /// let result = id_val.apply(id_fn); // Note: apply is (value).apply(function_container)
    /// assert_eq!(result, Identity(20));
    /// ```
    fn apply<B>(self, i: Identity<Self::Fnn<A, B>>) -> Identity<B>
    where
        Self: Sized,
    {
        // self is Identity<A>, i is Identity<CFn<A,B>>
        // i.0 is CFn<A,B> (the function), self.0 is A (the value)
        Identity(i.0.call(self.0))
    }
}

/// Implements the `Applicative` trait for `Identity<A>`.
///
/// `pure` simply wraps a given value `v` into an `Identity<A>`.
impl<A: 'static> Applicative<A> for Identity<A> {
    type Applicative<T> = Identity<T>;

    /// Wraps a value `v` into an `Identity<A>`.
    ///
    /// This is the `pure` function of the `Applicative` interface for `Identity`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fp_rs::applicative::Applicative;
    /// use fp_rs::identity::Identity;
    ///
    /// let pure_id: Identity<i32> = <Identity<i32> as Applicative<i32>>::pure(42);
    /// assert_eq!(pure_id, Identity(42));
    /// ```
    fn pure(v: A) -> Self::Applicative<A> {
        Identity(v)
    }
}

/// Implements the `Bind` (or `Monad::bind`) trait for `Identity<A>`.
///
/// `bind` applies a function `f` to the wrapped value. The function `f` itself
/// must return a new `Identity`. Since `f` already returns an `Identity`,
/// `bind` simply returns the result of `f(self.0)`.
impl<A: 'static> Bind<A> for Identity<A> {
    type Bind<T> = Identity<T>;

    /// Applies a function `f` that returns an `Identity` to the wrapped value.
    ///
    /// Given `self` as `Identity(value)` and `f` as a function `A -> Identity<B>`,
    /// this method returns the `Identity<B>` produced by `f(value)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fp_rs::monad::Bind; // or use fp_rs::monad::Monad;
    /// use fp_rs::identity::Identity;
    ///
    /// let id_val = Identity(3);
    /// let f = |x: i32| Identity(x + 7);
    /// let result = id_val.bind(f);
    /// assert_eq!(result, Identity(10));
    /// ```
    fn bind<B, F>(self, f: F) -> Self::Bind<B>
    where
        F: Fn(A) -> Self::Bind<B> + 'static,
    {
        // self is Identity<A>, f is A -> Identity<B>
        // The result of f(self.0) is already Identity<B>.
        f(self.0)
    }
}

/// Implements the `Monad` marker trait for `Identity<A>`.
///
/// `Monad` combines `Applicative` and `Bind`. Since `Identity` implements both,
/// it is a `Monad`. This is a marker trait and does not add new methods here.
impl<A: 'static> Monad<A> for Identity<A> {}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::function::CFn;

    #[test]
    fn test_identity_functor_map() {
        let id_val = Identity(String::from("hello"));
        let mapped_id = id_val.map(|s| s.len());
        assert_eq!(mapped_id, Identity(5));

        let id_num = Identity(10);
        let mapped_id_num = id_num.map(|x| x * x);
        assert_eq!(mapped_id_num, Identity(100));
    }

    #[test]
    fn test_identity_apply() {
        let id_val = Identity(5);
        let id_fn: Identity<CFn<i32, i32>> = Identity(CFn::new(|x| x * 2));
        let result = id_val.apply(id_fn);
        assert_eq!(result, Identity(10));

        let id_str_val = Identity(String::from("test"));
        let id_str_fn: Identity<CFn<String, usize>> = Identity(CFn::new(|s: String| s.len()));
        let result_str = id_str_val.apply(id_str_fn);
        assert_eq!(result_str, Identity(4));
    }

    #[test]
    fn test_identity_applicative_pure() {
        let pure_val: Identity<i32> = <Identity<i32> as Applicative<i32>>::pure(42);
        assert_eq!(pure_val, Identity(42));

        let pure_str: Identity<&str> = <Identity<&str> as Applicative<&str>>::pure("pure");
        assert_eq!(pure_str, Identity("pure"));
    }

    #[test]
    fn test_identity_monad_bind() {
        let id_val = Identity(3);
        let f = |x: i32| Identity(x + 7);
        let result = id_val.bind(f);
        assert_eq!(result, Identity(10));

        let id_str = Identity(String::from("world"));
        let f_str = |s: String| Identity(format!("hello {}", s));
        let result_str = id_str.bind(f_str);
        assert_eq!(result_str, Identity(String::from("hello world")));
    }

    // Law tests (simplified for Identity)
    #[test]
    fn test_identity_monad_left_identity() {
        // pure(a).bind(f) == f(a)
        let a = 10;
        let f = |x: i32| Identity(x * x);

        let lhs: Identity<i32> = <Identity<i32> as Applicative<i32>>::pure(a).bind(f);
        let rhs: Identity<i32> = f(a);
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_identity_monad_right_identity() {
        // m.bind(pure) == m
        let m = Identity(20);
        let pure_fn = |x: i32| <Identity<i32> as Applicative<i32>>::pure(x);

        let lhs = m.clone().bind(pure_fn); // clone m as bind consumes
        let rhs = m;
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_identity_monad_associativity() {
        // m.bind(f).bind(g) == m.bind(|x| f(x).bind(g))
        let m = Identity(5);
        let f = |x: i32| Identity(x + 1); // 5 -> Identity(6)
        let g = |y: i32| Identity(y * 2); // 6 -> Identity(12)

        let lhs = m.clone().bind(f).bind(g); // Identity(5) -> Identity(6) -> Identity(12)

        let rhs_fn = move |x: i32| f(x).bind(g); // x=5: f(5)=Identity(6), Identity(6).bind(g)=Identity(12) // Added move
        let rhs = m.bind(rhs_fn);           // Identity(5).bind(|x| Identity( (x+1)*2 )) -> Identity(12)
        assert_eq!(lhs, rhs);
    }
}
