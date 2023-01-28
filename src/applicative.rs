/// A Applicative
///
/// Applicative trait supports an operation called pure.
pub trait Applicative<A> {
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

mod tests {

    #[test]
    fn check_some() {
        use super::Applicative;

        assert_eq!(Option::pure(1), Some(1))
    }
}
