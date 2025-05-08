use std::ops::Deref;

use crate::monad::Bind;

type BFn<A, B> = Box<dyn Fn(A) -> B + 'static>;

type BFnOnce<A, B> = Box<dyn FnOnce(A) -> B + 'static>;

pub struct CFn<A, B>(pub BFn<A, B>);
pub struct CFnOnce<A, B>(pub BFnOnce<A, B>);

impl<A, B> CFn<A, B> {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(A) -> B + 'static,
    {
        CFn(Box::new(f))
    }

    pub fn call(&self, arg: A) -> B {
        (self.0)(arg)
    }
}

impl<A, B> CFnOnce<A, B> {
    pub fn new<F>(f: F) -> Self
    where
        F: FnOnce(A) -> B + 'static,
    {
        CFnOnce(Box::new(f))
    }

    pub fn call_once(self, arg: A) -> B {
        (self.0)(arg)
    }
}

impl<A, B> Deref for CFn<A, B> {
    type Target = BFn<A, B>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<A, B> Deref for CFnOnce<A, B> {
    type Target = BFnOnce<A, B>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn compose<A: 'static, B: 'static, C: 'static>(f: BFn<A, B>, g: BFn<B, C>) -> BFn<A, C> {
    Box::new(move |x| g(f(x)))
}

fn compose_fn_once<A: 'static, B: 'static, C: 'static>(
    f: BFnOnce<A, B>,
    g: BFnOnce<B, C>,
) -> BFnOnce<A, C> {
    Box::new(move |x| g(f(x)))
}

impl<A: 'static, B: 'static, C: 'static> std::ops::Shr<CFn<B, C>> for CFn<A, B> {
    type Output = CFn<A, C>;
    fn shr(self, rhs: CFn<B, C>) -> Self::Output {
        CFn(compose(self.0, rhs.0))
    }
}

impl<A: 'static, B: 'static, C: 'static> std::ops::Shl<CFn<A, B>> for CFn<B, C> {
    type Output = CFn<A, C>;
    fn shl(self, rhs: CFn<A, B>) -> Self::Output {
        CFn(compose(rhs.0, self.0))
    }
}

impl<A: 'static, B: 'static, C: 'static> std::ops::Shr<CFnOnce<B, C>> for CFnOnce<A, B> {
    type Output = CFnOnce<A, C>;
    fn shr(self, rhs: CFnOnce<B, C>) -> Self::Output {
        CFnOnce(compose_fn_once(self.0, rhs.0))
    }
}

impl<A: 'static, B: 'static, C: 'static> std::ops::Shl<CFnOnce<A, B>> for CFnOnce<B, C> {
    type Output = CFnOnce<A, C>;
    fn shl(self, rhs: CFnOnce<A, B>) -> Self::Output {
        CFnOnce(compose_fn_once(rhs.0, self.0))
    }
}

pub struct BindableFn<M: Bind<A> + Bind<B>, A, B>(CFn<A, BindType<M, A, B>>);

type BindType<M, A, B> = <M as Bind<A>>::Bind<B>;

impl<M: Bind<A> + Bind<B>, A, B> Deref for BindableFn<M, A, B> {
    type Target = BFn<A, BindType<M, A, B>>;

    fn deref(&self) -> &Self::Target {
        &(self.0)
    }
}

impl<M, A, B> BindableFn<M, A, B>
where
    M: Bind<A> + Bind<B> + Bind<A, Bind<B> = M>,
{
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(A) -> M + 'static,
    {
        BindableFn(CFn::new(f))
    }
}
