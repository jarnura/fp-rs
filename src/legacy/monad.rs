// Content from the original classic module in src/monad.rs
use crate::legacy::applicative::Applicative; // Point to legacy Applicative
use crate::legacy::apply::Apply;             // Point to legacy Apply

pub trait Monad<A>: Applicative<A> + Bind<A> {}

impl<A: 'static> Monad<A> for Option<A> {}
impl<A: 'static, E: 'static + Clone> Monad<A> for Result<A, E> {}
impl<A: 'static + Clone> Monad<A> for Vec<A> {}

pub trait Bind<A>: Apply<A> {
    type Bind<T>;
    fn bind<B, F>(self, f: F) -> Self::Bind<B>
    where
        F: Fn(A) -> Self::Bind<B> + Clone + 'static;
}

impl<A: 'static> Bind<A> for Option<A> {
    type Bind<T> = Option<T>;
    fn bind<B, F>(self, f: F) -> Self::Bind<B>
    where
        F: Fn(A) -> Self::Bind<B> + 'static,
    {
        self.and_then(f)
    }
}

impl<A: 'static, E: 'static + Clone> Bind<A> for Result<A, E> {
    type Bind<T> = Result<T, E>;
    fn bind<B, F>(self, f: F) -> Self::Bind<B>
    where
        F: Fn(A) -> Self::Bind<B> + 'static,
    {
        self.and_then(f)
    }
}

impl<A: 'static + Clone> Bind<A> for Vec<A> {
    type Bind<T> = Vec<T>;
    fn bind<B, F>(self, f: F) -> Self::Bind<B>
    where
        F: Fn(A) -> Self::Bind<B> + 'static,
    {
        self.into_iter().flat_map(f).collect()
    }
}

pub fn bind<A, B, MA, MB, F>(f: F, ma: MA) -> MB
where
    F: Fn(A) -> MB + Clone + 'static,
    MA: Bind<A, Bind<B> = MB>,
{
    ma.bind::<B, F>(f)
}

pub fn join<A, M, MM>(mma: MM) -> M
where
    M: Bind<A, Bind<A> = M> + 'static,
    MM: Bind<M, Bind<A> = M>,
{
    mma.bind::<A, _>(|x: M| x)
}
