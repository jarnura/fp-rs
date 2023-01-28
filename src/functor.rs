/// A Functor
///
/// Functor trait supports an operation called __map.
pub trait Functor<A> {
    /// The Associative type which acts a `* -> *`.
    ///  `*(Functor)` -> `*(Any type)`   
    type Functor<T>;

    /// Assume F is a `Functor`, then __map can be used to apply a function `A -> B` on
    /// that `Functor<A>` or `F A`,  which produces `Functor<B>` or `F B`.
    fn __map<B, Func>(self, f: Func) -> <Self as Functor<A>>::Functor<B>
    where
        Func: FnOnce(A) -> B;
}

impl<A> Functor<A> for Option<A> {
    type Functor<T> = Option<T>;

    fn __map<B, Func>(self, f: Func) -> Self::Functor<B>
    where
        Func: FnOnce(A) -> B,
    {
        self.map(f)
    }
}

mod tests {

    #[test]
    fn add_one() {
        use super::Functor;

        let closure = |x| x + 1;
        assert_eq!(Some(1).__map(closure), Some(2))
    }
}
