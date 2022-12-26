/// A Functor
/// `Functor` is a type constructor which supports a mapping operation `__map`.
/// map can be used to turn functions a -> b into functions f a -> f b whose
/// argument and return types use the type constructor f to represent some
/// computational context.
/// f a -> f b
/// <Self as Functor<A>>::Functor<B>
/// f a -> (a -> b) -> f b
trait Functor<A> {
    type Functor<T>;

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
        let closure = |x| x + 1;
        assert_eq!(super::Functor::__map(Some(1), closure), Some(2))
    }

}