use crate::{applicative::Applicative, apply::Apply, function::CFn};

trait Monad<A>: Applicative<A> + Bind<A> {}

impl<A: 'static> Monad<A> for Option<A> {}

impl<A: 'static, E: 'static + Clone> Monad<A> for Result<A, E> {}

pub trait Bind<A>: Apply<A> {
    type Bind<T>;

    type BindFn<T, U>;

    fn bind<B>(self, m: Self::BindFn<A, Self::Bind<B>>) -> <Self as Bind<A>>::Bind<B>
    where
        Self: Bind<A>;
}

impl<A: 'static> Bind<A> for Option<A> {
    type Bind<T> = Option<T>;

    type BindFn<T, U> = CFn<T, U>;

    fn bind<B>(self, m: Self::BindFn<A, Self::Bind<B>>) -> <Self as Bind<A>>::Bind<B> {
        self.and_then(|a| (*m)(a))
    }
}

impl<A: 'static, E: 'static + Clone> Bind<A> for Result<A, E> {
    type Bind<T> = Result<T, E>;
    type BindFn<T, U> = CFn<T, U>; // U here will be Result<InnerB, E>

    fn bind<B>(self, m: Self::BindFn<A, Self::Bind<B>>) -> <Self as Bind<A>>::Bind<B> {
        // m is CFn<A, Result<B, E>>
        // self is Result<A, E>
        // Result::and_then does exactly this.
        self.and_then(|a| (*m)(a))
    }
}

pub fn bind<A, B, MA, MB, A2MB>(f: A2MB, ma: MA) -> MB
where
    A2MB: Fn(A) -> MB + 'static, // Removed Clone
    MA: Bind<A, Bind<B> = MB, BindFn<A, <MA as Bind<A>>::Bind<B>> = CFn<A, MB>>,
{
    let c = CFn::new(f);
    ma.bind::<B>(c)
}

pub fn join<A, M, MM>(mma: MM) -> M
where
    M: Bind<A, Bind<A> = M> + 'static, // Removed Clone bound for M
    MM: Bind<
        <M as Bind<A>>::Bind<A>,
        Bind<A> = M,
        BindFn<<M as Bind<A>>::Bind<A>, <M as Bind<A>>::Bind<A>> = CFn<M, M>,
    >,
{
    let i = CFn::new(|x: <M as Bind<A>>::Bind<A>| x);
    // Pass a closure that calls `i.call()`
    // Add type arguments: A_bind=InnerM, B_bind=A, MA_bind=MM, MB_bind=M
    bind::< <M as Bind<A>>::Bind<A>, A, MM, M, _>(move |val| i.call(val), mma) // Added move
}
