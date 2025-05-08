use crate::{fn2, function::CFn, functor::Functor};

/// A Apply
///
/// Apply trait supports an operation called apply.
pub trait Apply<A>: Functor<A> {
    /// The Associative type which acts a `* -> *`.
    ///  `*(Apply)` -> `*(T)`
    type Apply<T>;

    /// Apply used to lift a normal function to `Apply<Function>`.
    ///  `Fnn<T,U>` is the normal function
    type Fnn<T, U>; // Removed AnyFunction bound

    /// Assume F is a `Apply`, then apply can be used to apply a wrapped function `Apply<(A -> B)>` on
    /// that `Apply<A>` or `F A`,  which produces `Apply<B>` or `F B`.
    #[allow(clippy::type_complexity)]
    fn apply<B>(
        self,
        i: <Self as Functor<A>>::Functor<Self::Fnn<A, B>>, // Simplified this type
    ) -> <Self as Apply<A>>::Apply<B>
    where
        Self: Sized;
}

impl<A: 'static> Apply<A> for Option<A> {
    type Apply<T> = Option<T>;

    type Fnn<T, U> = CFn<T, U>;

    fn apply<B>(self, i: Option<Self::Fnn<A, B>>) -> Option<B>
    // Simplified this type
    where
        Self: Sized,
    {
        self.and_then(|v| i.map(|f| (*f)(v)))
    }
}

impl<A: 'static, E: 'static + Clone> Apply<A> for Result<A, E> {
    type Apply<T> = Result<T, E>;
    type Fnn<T, U> = CFn<T, U>;

    fn apply<B>(self, i: Result<Self::Fnn<A, B>, E>) -> Result<B, E>
    where
        Self: Sized,
    {
        match self {
            Ok(v) => match i {
                Ok(f) => Ok((*f)(v)),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }
}

impl<A: 'static + Clone> Apply<A> for Vec<A> {
    // Added Clone bound for A
    type Apply<T> = Vec<T>;
    type Fnn<T, U> = CFn<T, U>; // Using CFn to represent the function type

    fn apply<B>(self, fs: Vec<Self::Fnn<A, B>>) -> Vec<B>
    where
        Self: Sized,
    {
        let mut result_vec = Vec::new();
        if self.is_empty() || fs.is_empty() {
            return result_vec;
        }
        for f_fn in fs {
            for val_a in self.iter() {
                result_vec.push((*f_fn)(val_a.clone()));
            }
        }
        result_vec
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
    A2B2C: Fn(A) -> CFn<B, C> + 'static,
    FA: Functor<A, Functor<CFn<B, C>> = FB2C>,
    FB: Apply<B, Functor<<FB as Apply<B>>::Fnn<B, C>> = FB2C, Apply<C> = FC>,
{
    fb.apply(fa.map(func))
}

pub fn lift3<A, B, C, D, A2B2C2D, FB2C2D, FC2D, FA, FB, FC, FD>(
    func: A2B2C2D,
    fa: FA,
    fb: FB,
    fc: FC,
) -> FD
where
    A2B2C2D: Fn(A) -> CFn<B, CFn<C, D>> + 'static,
    FA: Functor<A, Functor<CFn<B, CFn<C, D>>> = FB2C2D>,
    FB: Apply<B, Functor<<FB as Apply<B>>::Fnn<B, CFn<C, D>>> = FB2C2D, Apply<CFn<C, D>> = FC2D>,
    FC: Apply<C, Functor<<FC as Apply<C>>::Fnn<C, D>> = FC2D, Apply<D> = FD>,
{
    fc.apply(fb.apply(fa.map(func)))
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
    FB: Apply<B, Functor<<FB as Apply<B>>::Fnn<B, A>> = FB2A>,
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
    FB: Apply<B, Functor<<FB as Apply<B>>::Fnn<B, B>> = FB2B>,
{
    lift2(fn2!(|_x| move |y| y), fa, fb)
}
