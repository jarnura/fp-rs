use std::ops::Deref;

use crate::monad::Bind;

type BFn<A, B> = Box<dyn Fn(A) -> B + 'static>;

pub struct CFn<A, B>(BFn<A, B>);

impl<A, B> CFn<A, B> {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(A) -> B + 'static,
    {
        CFn(Box::new(f))
    }
}

impl<A, B> Deref for CFn<A, B> {
    type Target = BFn<A, B>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait AnyFunction<A, B> {
    type Function: FnOnce(A) -> B;
}

impl<A, B> AnyFunction<A, B> for CFn<A, B> {
    type Function = CFn<A, B>;
}

impl<A, B> FnOnce<(A,)> for CFn<A, B> {
    type Output = B;
    extern "rust-call" fn call_once(self, b: (A,)) -> Self::Output {
        self.0.call_once(b)
    }
}

impl<A, B> Fn<(A,)> for CFn<A, B> {
    extern "rust-call" fn call(&self, b: (A,)) -> Self::Output {
        self.0.call(b)
    }
}

impl<A, B> FnMut<(A,)> for CFn<A, B> {
    extern "rust-call" fn call_mut(&mut self, b: (A,)) -> Self::Output {
        self.0.call_mut(b)
    }
}

fn compose<A: 'static, B: 'static, C: 'static>(f: BFn<A, B>, g: BFn<B, C>) -> BFn<A, C> {
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

impl<M, A, B> std::ops::BitOr<BindableFn<M, A, B>> for BindType<M, A, A>
where
    M: Bind<B>
        + Bind<A, Bind<A> = M>
        + Bind<A, Bind<B> = M, BindFn<A, B> = CFn<A, <M as Bind<A>>::Bind<B>>>,
{
    type Output = BindType<M, A, B>;

    fn bitor(self, rhs: BindableFn<M, A, B>) -> Self::Output {
        <M as Bind<A>>::bind::<B>(self, rhs.0)
    }
}
