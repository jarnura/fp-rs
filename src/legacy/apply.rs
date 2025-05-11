// Content from the original classic module in src/apply.rs
use crate::function::CFn; // CFn is not part of legacy/hkt split
use crate::legacy::functor::Functor; // Point to legacy Functor

/// Legacy version of the `Apply` trait.
///
/// `Apply` extends `Functor` by providing the `apply` method, which allows
/// applying a wrapped function to a wrapped value.
/// This version uses associated types.
pub trait Apply<A>: Functor<A> {
    /// The type constructor for this `Apply` instance.
    /// E.g., if `Self` is `Option<A>`, then `Apply<T>` would be `Option<T>`.
    type Apply<T>;
    /// The type of the (wrapped) function used by `apply`.
    /// Typically `CFn<T, U>` for function `T -> U`.
    type Fnn<T, U>;

    /// Applies a wrapped function to a wrapped value.
    ///
    /// Given `self` (e.g., `Option<A>`) and `i` (e.g., `Option<CFn<A,B>>`),
    /// produces a result (e.g., `Option<B>`).
    #[allow(clippy::type_complexity)]
    fn apply<B>(
        self,
        i: <Self as Functor<A>>::Functor<Self::Fnn<A, B>>,
    ) -> <Self as Apply<A>>::Apply<B>
    where
        Self: Sized,
        B: 'static,
        <Self as Functor<A>>::Functor<Self::Fnn<A, B>>: 'static;
}

impl<A: 'static> Apply<A> for Option<A> {
    type Apply<T> = Option<T>;
    type Fnn<T, U> = CFn<T, U>;

    fn apply<B>(self, i: Option<Self::Fnn<A, B>>) -> Option<B>
    where
        Self: Sized,
    {
        self.and_then(|val_a| i.map(|func_ab| func_ab.call(val_a)))
    }
}

impl<A: 'static, E: 'static + Clone> Apply<A> for Result<A, E> {
    type Apply<T> = Result<T, E>;
    type Fnn<T, U> = CFn<T, U>;

    fn apply<B>(self, i: Result<Self::Fnn<A, B>, E>) -> Result<B, E>
    where
        Self: Sized,
    {
        self.and_then(|val_a| i.map(|func_ab| func_ab.call(val_a)))
    }
}

impl<A: 'static + Clone> Apply<A> for Vec<A> {
    type Apply<T> = Vec<T>;
    type Fnn<T, U> = CFn<T, U>;

    fn apply<B>(self, fs: Vec<Self::Fnn<A, B>>) -> Vec<B>
    where
        Self: Sized,
    {
        fs.into_iter()
            .flat_map(|f_fn| {
                self.iter()
                    .map(move |val_a| f_fn.call(val_a.clone()))
            })
            .collect()
    }
}

/// Lifts a binary function to operate on two `Apply` contexts.
///
/// Given `func: A -> (B -> C)`, `fa: F<A>`, `fb: F<B>`, produces `F<C>`.
/// This is a common helper for `Apply` types.
pub fn lift2<A, B, C: 'static, A2B2C, FB2C: 'static, FA, FB, FC>(
    func: A2B2C,
    fa: FA,
    fb: FB,
) -> FC
where
    A2B2C: Fn(A) -> CFn<B, C> + Clone + 'static,
    FA: Functor<A, Functor<CFn<B, C>> = FB2C>,
    FB: Apply<B, Functor<<FB as Apply<B>>::Fnn<B, C>> = FB2C, Apply<C> = FC>,
{
    let f_b_to_c_in_fa = <FA as Functor<A>>::map(fa, func);
    <FB as Apply<B>>::apply(fb, f_b_to_c_in_fa)
}

/// Lifts a ternary function to operate on three `Apply` contexts.
///
/// Given `func: A -> (B -> (C -> D))`, `fa: F<A>`, `fb: F<B>`, `fc: F<C>`,
/// produces `F<D>`.
pub fn lift3<
    A,
    B,
    C: 'static,
    D: 'static,
    A2B2C2D,
    FB2C2D: 'static,
    FC2D: 'static,
    FA,
    FB,
    FC,
    FD,
>(
    func: A2B2C2D,
    fa: FA,
    fb: FB,
    fc: FC,
) -> FD
where
    A2B2C2D: Fn(A) -> CFn<B, CFn<C, D>> + Clone + 'static,
    FA: Functor<A, Functor<CFn<B, CFn<C, D>>> = FB2C2D>,
    FB: Apply<B, Functor<<FB as Apply<B>>::Fnn<B, CFn<C, D>>> = FB2C2D, Apply<CFn<C, D>> = FC2D>,
    FC: Apply<C, Functor<<FC as Apply<C>>::Fnn<C, D>> = FC2D, Apply<D> = FD>,
{
    let f_b_to_c_to_d_in_fa = <FA as Functor<A>>::map(fa, func);
    let f_c_to_d_in_fb = <FB as Apply<B>>::apply(fb, f_b_to_c_to_d_in_fa);
    <FC as Apply<C>>::apply(fc, f_c_to_d_in_fb)
}

/// Applies the function in the first context to the value in the second,
/// but returns the result as if applied to the first context's original value.
/// Essentially, `fa *> fb` (sequence `fb` after `fa`, keeping `fa`'s original value type).
/// This is often called "apply first" or "followed by".
///
/// `apply_first(fa, fb)` is equivalent to `lift2(|a| |_b| a, fa, fb)`.
pub fn apply_first<A, B, FA, FB, FB2A: 'static>(fa: FA, fb: FB) -> <FB as Apply<B>>::Apply<A>
where
    A: Copy + 'static,
    B: 'static,
    FA: Functor<A, Functor<CFn<B, A>> = FB2A>,
    FB: Apply<B, Functor<<FB as Apply<B>>::Fnn<B, A>> = FB2A>,
{
    let map_fn = |x: A| CFn::new(move |_y: B| x);
    lift2(map_fn, fa, fb)
}

/// Applies the function in the first context to the value in the second,
/// but returns the result as if applied to the second context's original value.
/// Essentially, `fa <* fb` (sequence `fb` after `fa`, keeping `fb`'s original value type).
/// This is often called "apply second" or "preceded by".
///
/// `apply_second(fa, fb)` is equivalent to `lift2(|_a| |b| b, fa, fb)`.
pub fn apply_second<A, B, FA, FB, FMapResult, ResultApplyB>(
    fa: FA,
    fb: FB,
) -> ResultApplyB
where
    A: 'static,
    B: 'static,
    FA: Functor<A, Functor<CFn<B, B>> = FMapResult>,
    FB: Apply<B, Functor<<FB as Apply<B>>::Fnn<B, B>> = FMapResult, Apply<B> = ResultApplyB>,
    FMapResult: Functor<<FB as Apply<B>>::Fnn<B, B>> + 'static,
{
    let map_fn = |_: A| CFn::new(|y: B| y);
    let mapped_fa: FMapResult = <FA as Functor<A>>::map(fa, map_fn);
    <FB as Apply<B>>::apply(fb, mapped_fa)
}
