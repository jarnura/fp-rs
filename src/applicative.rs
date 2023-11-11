use crate::{
    apply::Apply,
    function::{AnyFunction, CFn},
};

/// A Applicative
///
/// Applicative trait supports an operation called pure.
pub trait Applicative<A>: Apply<A> {
    // Me: Apply<A>;
    /// The Associative type which acts a `* -> *`
    /// `*(Applicative)` -> `*(Any type)`   
    type Applicative<T>;

    /// Assume F is a `Applicative`, then pure can be use to lift a value
    /// A to `Applicative<A>` or `F A`
    fn pure(v: A) -> Self::Applicative<A>;
}

impl<A> Applicative<A> for Option<A> {
    type Applicative<T> = Option<T>;

    fn pure(v: A) -> Self::Applicative<A> {
        Some(v)
    }
}

pub fn lift_a1<F, A, B, A2B, FA2B>(f: A2B, fa: F) -> <F as Apply<A>>::Apply<B>
where
    A2B: Fn(A) -> B + 'static,
    FA2B: Applicative<CFn<A, B>, Applicative<CFn<A, B>> = FA2B>,
    F: Apply<A, Functor<<<F as Apply<A>>::Fnn<A, B> as AnyFunction<A, B>>::Function> = FA2B>,
{
    fa.apply(FA2B::pure(CFn::new(f)))
}

#[cfg(test)]
mod tests {

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
