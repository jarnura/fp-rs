use crate::{applicative::Applicative, apply::Apply, function::CFn};

trait Monad<A>: Applicative<A> + Bind<A> {}

impl<A> Monad<A> for Option<A> {}

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
        self.and_then(|a| (*m)(a))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{bfn1, fn1};

    #[test]
    fn bind_option() {
        let add_one = fn1!(|x: i32| Some(x + 1));
        let add_two = fn1!(|x: i32| Some(x + 2));
        let add_three = fn1!(|x: i32| Some(x + 3));
        let result = Some(1).bind(add_one).bind(add_two).bind(add_three);
        assert_eq!(result, Some(7))
    }

    #[test]
    fn bind_option_with_composing() {
        let add_one = fn1!(|x: i32| Some(x + 1));
        let add_two = fn1!(|x: i32| x + 2);
        let add_three = fn1!(|x: i32| x + 3);
        let composed = add_one << add_two << add_three;
        let result = Some(1).bind(composed);
        assert_eq!(result, Some(7))
    }

    #[test]
    fn bind_option_with_bind_composing() {
        let add_one = bfn1!(|x: i32| Some(x + 1));
        let add_two = bfn1!(|x: i32| Some(x + 2));
        let add_three = bfn1!(|x: i32| Some(x + 3));
        let result = Some(1) | add_one | add_two | add_three;
        assert_eq!(result, Some(7))
    }
}
