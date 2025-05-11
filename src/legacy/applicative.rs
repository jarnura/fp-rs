// Content from the original classic module in src/applicative.rs
use crate::legacy::apply::Apply; // Point to legacy Apply

pub trait Applicative<A>: Apply<A> {
    type Applicative<T>;
    fn pure(v: A) -> Self::Applicative<A>;
}

impl<A: 'static> Applicative<A> for Option<A> {
    type Applicative<T> = Option<T>;
    fn pure(v: A) -> Self::Applicative<A> {
        Some(v)
    }
}

impl<A: 'static, E: 'static + Clone> Applicative<A> for Result<A, E> {
    type Applicative<T> = Result<T, E>;
    fn pure(v: A) -> Self::Applicative<A> {
        Ok(v)
    }
}

impl<A: 'static + Clone> Applicative<A> for Vec<A> {
    type Applicative<T> = Vec<T>;
    fn pure(v: A) -> Self::Applicative<A> {
        vec![v]
    }
}

// #[allow(clippy::module_name_repetitions)]
// pub fn lift_a1<AppCtx, A, B: 'static, FnHook>(
//     f: FnHook,
//     fa: AppCtx,
// ) -> <AppCtx as Apply<A>>::Apply<B>
// where
//     A: 'static, // Added: Apply<A> impls often require A: 'static
//     FnHook: Fn(A) -> B + 'static,
//     AppCtx: Apply<A>, // AppCtx is, e.g., Option<A>, Vec<A>

//     // This complex type is "AppCtx's structure but holding a function CFn<A,B>"
//     // e.g., if AppCtx is Option<A>, this type is Option<CFn<A,B>>.
//     // This type must implement Applicative for CFn<A,B>.
//     <AppCtx as crate::legacy::functor::Functor<A>>::Functor<<AppCtx as Apply<A>>::Fnn<A, B>>:
//         Applicative<CFn<A, B>> + 'static,
// {
//     let f_cfn = CFn::new(f);

//     // Call pure on the type "AppCtx's structure holding CFn<A,B>".
//     // For example, if AppCtx is Option<A>, this is effectively:
//     // let f_in_context: Option<CFn<A,B>> = <Option<CFn<A,B>> as Applicative<CFn<A,B>>>::pure(f_cfn);
//     let f_in_context: <AppCtx as crate::legacy::functor::Functor<A>>::Functor<<AppCtx as Apply<A>>::Fnn<A, B>> =
//         <<AppCtx as crate::legacy::functor::Functor<A>>::Functor<<AppCtx as Apply<A>>::Fnn<A, B>>
//             as Applicative<CFn<A, B>>>::pure(f_cfn);

//     // Pass this f_in_context to AppCtx's apply method.
//     // AppCtx::apply expects its second argument to be of type:
//     // <AppCtx as Functor<A>>::Functor<<AppCtx as Apply<A>>::Fnn<A,B>>
//     // which is exactly the type of f_in_context.
//     <AppCtx as Apply<A>>::apply(fa, f_in_context)
// }
