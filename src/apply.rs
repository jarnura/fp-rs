use crate::{
    functor::Functor,
    utils::{AnyFunction, CFn},
};

pub trait Apply<A>: Functor<A> {
    type Apply<T>;

    type Fnn<T, U>: AnyFunction<T, U>;

    fn apply<B>(
        self,
        i: <Self as Functor<A>>::Functor<Self::Fnn<A, B>>,
    ) -> <Self as Apply<A>>::Apply<B>
    where
        Self: Sized;
}

impl<A> Apply<A> for Option<A> {
    type Apply<T> = Option<T>;

    type Fnn<T, U> = CFn<T, U>;

    fn apply<B>(self, i: Option<Self::Fnn<A, B>>) -> Option<B>
    where
        Self: Sized,
    {
        self.and_then(|v| i.__map(|f| f.run(v)))
    }
}

mod tests {

    #[test]
    fn apply_on_option() {
        use super::Apply;
        use crate::{closure2, functor::Functor};

        let closure = closure2!(|x: i32| move |y: i32| x + y);
        let some_closure = Some(1).__map(closure);
        let none_closure = None.__map(closure);
        assert_eq!(Some(2).apply(some_closure), Some(3));
        assert_eq!(Some(2).apply(none_closure), None)
    }
}
