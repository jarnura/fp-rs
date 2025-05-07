use crate::{
    apply::Apply,
    function::{CFn},
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

pub fn lift_a1<F, A, B, A2B, FA2B>(f: A2B, fa: F) -> <F as Apply<A>>::Apply<B>
where
    A2B: Fn(A) -> B + 'static, // Removed Clone
    FA2B: Applicative<CFn<A, B>, Applicative<CFn<A, B>> = FA2B>,
    F: Apply<A, Functor<<F as Apply<A>>::Fnn<A, B>> = FA2B>, // Simplified this bound
{
    fa.apply(FA2B::pure(CFn::new(f)))
}
