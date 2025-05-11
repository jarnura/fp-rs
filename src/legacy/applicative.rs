// Content from the original classic module in src/applicative.rs
use crate::legacy::apply::Apply; // Point to legacy Apply

/// Legacy version of the `Applicative` trait.
///
/// An `Applicative` functor is a type that can lift a value into a context (`pure`)
/// and apply a wrapped function to a wrapped value (via `apply` from the `Apply` supertrait).
/// This version uses associated types rather than HKTs.
pub trait Applicative<A>: Apply<A> {
    /// The type constructor for this `Applicative`.
    /// For example, if `Self` is `Option<A>`, then `Applicative<T>` would be `Option<T>`.
    type Applicative<T>;
    /// Lifts a value `v` of type `A` into the applicative context.
    ///
    /// # Example
    /// ```
    /// use monadify::legacy::applicative::Applicative; // Assuming legacy feature is enabled
    ///
    /// let val_opt: Option<i32> = <Option<i32> as Applicative<i32>>::pure(10);
    /// assert_eq!(val_opt, Some(10));
    ///
    /// let val_vec: Vec<String> = <Vec<String> as Applicative<String>>::pure("hello".to_string());
    /// assert_eq!(val_vec, vec!["hello".to_string()]);
    /// ```
    fn pure(v: A) -> Self::Applicative<A>;
}

impl<A: 'static> Applicative<A> for Option<A> {
    type Applicative<T> = Option<T>;
    fn pure(v: A) -> Self::Applicative<A> {
        Some(v)
    }
}

impl<A: 'static, E: 'static + Clone> Applicative<A> for Result<A, E> {
    type Applicative<T> = Result<T, E>;
    fn pure(v: A) -> Self::Applicative<A> {
        Ok(v)
    }
}

impl<A: 'static + Clone> Applicative<A> for Vec<A> {
    type Applicative<T> = Vec<T>;
    fn pure(v: A) -> Self::Applicative<A> {
        vec![v]
    }
}
