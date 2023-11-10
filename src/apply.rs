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
    #[allow(clippy::type_complexity)]
    fn apply<B>(
        self,
        i: <Self as Functor<A>>::Functor<<Self::Fnn<A, B> as AnyFunction<A, B>>::Function>,
    ) -> <Self as Apply<A>>::Apply<B>
    where
        Self: Sized;
}

impl<A> Apply<A> for Option<A> {
    type Apply<T> = Option<T>;

    type Fnn<T, U> = CFn<T, U>;

    fn apply<B>(self, i: Option<<Self::Fnn<A, B> as AnyFunction<A, B>>::Function>) -> Option<B>
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
pub fn lift2<A, B, C, A2B2C, FB2C, FA, FB, FC>(func: A2B2C, fa: FA, fb: FB) -> FC
where
    A2B2C: Fn(A) -> CFn<B, C>,
    FA: Functor<A, Functor<CFn<B, C>> = FB2C>,
    FB: Apply<
        B,
        Functor<<<FB as Apply<B>>::Fnn<B, C> as AnyFunction<B, C>>::Function> = FB2C,
        Apply<C> = FC,
    >,
{
    fb.apply(fa.__map(func))
}

pub fn lift3<A, B, C, D, A2B2C2D, FB2C2D, FC2D, FA, FB, FC, FD>(
    func: A2B2C2D,
    fa: FA,
    fb: FB,
    fc: FC,
) -> FD
where
    A2B2C2D: Fn(A) -> CFn<B, CFn<C, D>>,
    FA: Functor<A, Functor<CFn<B, CFn<C, D>>> = FB2C2D>,
    FB: Apply<
        B,
        Functor< <<FB as Apply<B>>::Fnn<B, CFn<C,D>> as AnyFunction<B, CFn<C,D>>>::Function> = FB2C2D,
        Apply<CFn<C,D>> = FC2D,
        >,
    FC: Apply<
        C,
        Functor< <<FC as Apply<C>>::Fnn<C, D> as AnyFunction<C, D>>::Function> = FC2D,
        Apply<D> = FD,
    >,
{
    fc.apply(fb.apply(fa.__map(func)))
}

/// Combine two `Applyable` actions, keeping only the result of the first.
/// ```
/// use fp_rs::apply::apply_first;
///
/// assert_eq!(apply_first(Some(1), Some(2)), Some(1));
/// assert_eq!(apply_first(None::<i32>, Some(1)), None);
/// assert_eq!(apply_first(Some(1), None::<i32>), None);
/// assert_eq!(apply_first(Option::<i32>::None, None::<i32>), None);
/// ```
pub fn apply_first<A, B, FA, FB, FB2A>(fa: FA, fb: FB) -> <FB as Apply<B>>::Apply<A>
where
    A: Copy + 'static,
    FA: Functor<A, Functor<CFn<B, A>> = FB2A>,
    FB: Apply<B, Functor<<<FB as Apply<B>>::Fnn<B, A> as AnyFunction<B, A>>::Function> = FB2A>,
{
    lift2(fn2!(|x| move |_y| x), fa, fb)
}

/// Combine `two Applyable` actions, keeping only the result of the second.
/// ```
/// use fp_rs::apply::apply_second;
///
/// assert_eq!(apply_second(Some(1), Some(2)), Some(2));
/// assert_eq!(apply_second(None::<&str>, Some(1)), None);
/// assert_eq!(apply_second(Some(1), None::<i8>), None);
/// assert_eq!(apply_second(Option::<i32>::None, None::<i8>), None);
/// ```
pub fn apply_second<A, B, FA, FB, FB2B>(fa: FA, fb: FB) -> <FB as Apply<B>>::Apply<B>
where
    A: Copy + 'static,
    FA: Functor<A, Functor<CFn<B, B>> = FB2B>,
    FB: Apply<B, Functor<<<FB as Apply<B>>::Fnn<B, B> as AnyFunction<B, B>>::Function> = FB2B>,
{
    lift2(fn2!(|_x| move |y| y), fa, fb)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{fn2, fn3, functor::Functor};

    #[test]
    fn apply_on_option() {
        let closure = fn2!(|x: i32| move |y: i8| format!("{x}{y}"));
        let some_closure = Some(1).__map(closure);
        let none_closure = None.__map(closure);
        assert_eq!(Some(2).apply(some_closure), Some("12".to_string()));
        assert_eq!(Some(2).apply(none_closure), None);

        let closure = fn2!(|x: i32| move |y: i8| format!("{x}{y}"));
        assert_eq!(lift2(closure, Some(1), Some(2)), Some("12".to_string()));
        assert_eq!(lift2(closure, None, Some(2)), None);

        let closure = fn3!(|x: i32| move |y: i8| move |z: i32| x + y as i32 + z);

        assert_eq!(lift3(closure, Some(1), Some(2), Some(3)), Some(6));
    }
}
