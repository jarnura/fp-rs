/// A Functor
///
/// Functor trait supports an operation called __map.
pub trait Functor<A> {
    /// The Associative type which acts a `* -> *`.
    ///  `*(Functor)` -> `*(T)`   
    type Functor<T>;

    /// Assume F is a `Functor`, then __map can be used to apply a function `A -> B` on
    /// that `Functor<A>` or `F A`,  which produces `Functor<B>` or `F B`.
    fn __map<B, Func>(self, f: Func) -> <Self as Functor<A>>::Functor<B>
    where
        Func: FnOnce(A) -> B;
}

impl<A> Functor<A> for Option<A> {
    type Functor<T> = Option<T>;

    fn __map<B, Func>(self, f: Func) -> <Self as Functor<A>>::Functor<B>
    where
        Func: FnOnce(A) -> B,
    {
        self.map(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_one() {
        let closure = |x| x + 1;
        assert_eq!(Some(1).__map(closure), Some(2))
    }
}
