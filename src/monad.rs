use crate::{apply::Apply, utils::CFn};
pub trait Bind<A>: Apply<A> {
    type Bind<T>;

    type BindFn<T, U>;

    fn bind<B>(self, m: Self::BindFn<A, B>) -> <Self as Bind<A>>::Bind<B>
    where
        Self: Bind<A>;
}

impl<A> Bind<A> for Option<A> {
    type Bind<T> = Option<T>;

    type BindFn<T, U> = CFn<T, Self::Bind<U>>;

    fn bind<B>(self, m: Self::BindFn<A, B>) -> <Self as Bind<A>>::Bind<B> {
        self.and_then(|a| m.run(a))
    }
}

mod tests {

    #[test]
    fn bind_option() {
        use super::Bind;
        use crate::closure1;

        let add_one = closure1!(|x: i32| Some(x + 1));
        let add_two = closure1!(|x: i32| Some(x + 2));
        let add_three = closure1!(|x: i32| Some(x + 3));
        let result = Some(1).bind(add_one).bind(add_two).bind(add_three);
        assert_eq!(result, Some(7))
    }
}
