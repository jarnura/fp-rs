use crate::{
    fn2,
    function::{AnyFunction, CFn},
    functor::Functor,
};

/// A Apply
///
/// Apply trait supports an operation called apply.
pub trait Apply<A>: Functor<A> {
    /// The Associative type which acts a `* -> *`.
    ///  `*(Apply)` -> `*(T)`
    type Apply<T>;

    /// Apply used to lift a normal function to `Apply<Function>`.
    ///  `Fnn<T,U>` is the normal function
    type Fnn<T, U>: AnyFunction<T, U>;

    /// Assume F is a `Apply`, then apply can be used to apply a wrapped function `Apply<(A -> B)>` on
    /// that `Apply<A>` or `F A`,  which produces `Apply<B>` or `F B`.
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
        self.and_then(|v| i.__map(|f| (*f)(v)))
    }
}

/// Lift a function of two arguments to a function which accepts and returns
/// values wrapped with the type constructor `F`.
///
/// ```
/// use fp_rs::fn2;
/// use fp_rs::apply::lift2;
///
/// let closure = fn2!(|x: i32| move |y: i32| x + y);
/// assert_eq!(lift2(closure, Some(1), Some(2)), Some(3));
/// assert_eq!(lift2(closure, None, Some(2)), None)
///```
///
pub fn lift2<A, B, C, F, Fnn>(func: Fnn, fa: F, fb: F) -> F
where
    Fnn: Fn(A) -> CFn<B, C>,
    F: Apply<A, Apply<B> = F>
        + Functor<A, Functor<A> = F>
        + Functor<A, Functor<<F as Apply<A>>::Fnn<A, B>> = <F as Functor<A>>::Functor<CFn<B, C>>>,
{
    fb.apply(fa.__map(func))
}

/// Combine two `Applyable` actions, keeping only the result of the first.
/// ```
/// use fp_rs::apply::apply_first;
///
/// assert_eq!(apply_first(Some(1), Some(2)), Some(1));
/// assert_eq!(apply_first(None, Some(1)), None);
/// assert_eq!(apply_first(Some(1), None), None);
/// assert_eq!(apply_first(Option::<i32>::None, None), None);
/// ```
pub fn apply_first<A, B, F>(fa: F, fb: F) -> <F as Apply<A>>::Apply<A>
where
    A: Copy + 'static,
    F: Apply<A, Apply<A> = F>
        + Apply<A, Apply<B> = F>
        + Functor<A, Functor<A> = F>
        + Functor<A, Functor<<F as Apply<A>>::Fnn<A, B>> = <F as Functor<A>>::Functor<CFn<B, A>>>,
{
    lift2(fn2!(|x| move |_y| x), fa, fb)
}

/// Combine `two Applyable` actions, keeping only the result of the second.
/// ```
/// use fp_rs::apply::apply_second;
///
/// assert_eq!(apply_second(Some(1), Some(2)), Some(2));
/// assert_eq!(apply_second(None, Some(1)), None);
/// assert_eq!(apply_second(Some(1), None), None);
/// assert_eq!(apply_second(Option::<i32>::None, None), None);
/// ```
pub fn apply_second<A, B, F>(fa: F, fb: F) -> <F as Apply<A>>::Apply<B>
where
    A: Copy + 'static,
    F: Apply<A, Apply<A> = F>
        + Apply<A, Apply<B> = F>
        + Functor<A, Functor<A> = F>
        + Functor<A, Functor<<F as Apply<A>>::Fnn<A, A>> = <F as Functor<A>>::Functor<CFn<A, A>>>,
{
    lift2(fn2!(|_x| move |y| y), fa, fb)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{fn2, functor::Functor};

    #[test]
    fn apply_on_option() {
        let closure = fn2!(|x: i32| move |y: i32| x + y);
        let some_closure = Some(1).__map(closure);
        let none_closure = None.__map(closure);
        assert_eq!(Some(2).apply(some_closure), Some(3));
        assert_eq!(Some(2).apply(none_closure), None)
    }
}
