// Content from the original classic module in src/apply.rs
use crate::function::CFn; // CFn is not part of legacy/hkt split
use crate::legacy::functor::Functor; // Point to legacy Functor

pub trait Apply<A>: Functor<A> {
    type Apply<T>;
    type Fnn<T, U>;

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
