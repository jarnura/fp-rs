/*
use crate::{
    function::CFn, // Note: AnyFunction is not needed here
    functor::Functor,
};

pub trait MyApply<A>: Functor<A> {
    /// The Associative type which acts a `* -> *`.
    ///  `*(Apply)` -> `*(T)`
    type MyApply<T>;

    /// Apply used to lift a normal function to `Apply<Function>`.
    ///  `Fnn<T,U>` is the normal function
    type Fnn<T, U>: MyFancyFunc<T, U>;

    /// Assume F is a `Apply`, then apply can be used to apply a wrapped function `Apply<(A -> B)>` on
    /// that `Apply<A>` or `F A`,  which produces `Apply<B>` or `F B`.
    #[allow(clippy::type_complexity)]
    fn apply<B>(
        self,
        i: <Self as Functor<A>>::Functor<Self::Fnn<A, B>>,
    ) -> <Self as MyApply<A>>::MyApply<B>
    where
        Self: Sized;
}

impl<A: 'static> MyApply<A> for Option<A> {
    type MyApply<T> = Option<T>;

    type Fnn<T, U> = CFn<T, U>;

    fn apply<B>(self, i: Option<Self::Fnn<A, B>>) -> Option<B>
    where
        Self: Sized,
    {
        // Updated to use .map as Functor trait is being refactored.
        self.and_then(|v| i.map(|f| (f.func())(v)))
    }
}

pub trait MyFancyFunc<A, B> {
    // type Function: FnOnce(A) -> B;

    fn func(self) -> impl Fn(A) -> B;
}

impl<A, B> MyFancyFunc<A, B> for CFn<A, B> {
    // type Function = CFn<A, B>;

    fn func(self) -> impl Fn(A) -> B {
        self.0 // Return the inner Box<dyn Fn(A) -> B + 'static>
    }
}
*/
