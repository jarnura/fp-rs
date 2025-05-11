// Content from the original classic module in src/transformers/reader.rs
use std::marker::PhantomData;
use std::rc::Rc;
use crate::legacy::applicative::Applicative;
use crate::legacy::apply::Apply;
use crate::legacy::functor::Functor;
use crate::legacy::identity::Identity; 
use crate::legacy::monad::{Bind, Monad};

pub struct ReaderT<R, M, A> {
    pub run_reader_t: Rc<dyn Fn(R) -> M + 'static>,
    _phantom_a: PhantomData<A>,
}

impl<R, M, A> ReaderT<R, M, A> {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(R) -> M + 'static,
    {
        ReaderT {
            run_reader_t: Rc::new(f),
            _phantom_a: PhantomData,
        }
    }
}

pub type Reader<R, A> = ReaderT<R, Identity<A>, A>;

impl<R, M, A> Functor<A> for ReaderT<R, M, A>
where
    R: Clone + 'static,
    M: Functor<A> + 'static,
    A: 'static,
{
    type Functor<BVal> = ReaderT<R, <M as Functor<A>>::Functor<BVal>, BVal>;
    fn map<B, Func>(self, f: Func) -> Self::Functor<B>
    where
        Func: Fn(A) -> B + Clone + 'static,
    {
        let run_reader_t_clone = self.run_reader_t.clone();
        ReaderT::new(move |env: R| {
            let m_val = run_reader_t_clone(env);
            m_val.map(f.clone())
        })
    }
}

impl<R, M, A> Apply<A> for ReaderT<R, M, A>
where
    R: Clone + 'static,
    M: Apply<A> + 'static,
    A: 'static,
{
    type Apply<TVal> = ReaderT<R, <M as Apply<A>>::Apply<TVal>, TVal>;
    type Fnn<TArg, TRes> = <M as Apply<A>>::Fnn<TArg, TRes>;
    fn apply<B>(self, i: <Self as Functor<A>>::Functor<Self::Fnn<A, B>>) -> Self::Apply<B>
    where
        Self: Sized,
        B: 'static,
        <Self as Functor<A>>::Functor<<Self as Apply<A>>::Fnn<A, B>>: 'static,
    {
        let self_run = self.run_reader_t.clone();
        let i_run = i.run_reader_t.clone();
        ReaderT::new(move |env: R| {
            let m_val = self_run(env.clone());
            let m_func = i_run(env);
            m_val.apply(m_func)
        })
    }
}

impl<R, M, A> Applicative<A> for ReaderT<R, M, A>
where
    R: Clone + 'static,
    M: Applicative<A> + 'static,
    A: Clone + 'static,
    <M as Applicative<A>>::Applicative<A>: 'static,
{
    type Applicative<TVal> = ReaderT<R, <M as Applicative<A>>::Applicative<TVal>, TVal>;
    fn pure(v: A) -> Self::Applicative<A>
    where
        <M as Applicative<A>>::Applicative<A>: 'static,
    {
        ReaderT::new(move |_env: R| M::pure(v.clone()))
    }
}

impl<R, M, A> Bind<A> for ReaderT<R, M, A>
where
    R: Clone + 'static,
    M: Bind<A> + 'static,
    A: 'static,
{
    type Bind<TVal> = ReaderT<R, <M as Bind<A>>::Bind<TVal>, TVal>;
    fn bind<B, F>(self, f: F) -> Self::Bind<B>
    where
        F: Fn(A) -> Self::Bind<B> + Clone + 'static,
    {
        let self_run = self.run_reader_t.clone();
        ReaderT::new(move |env: R| {
            let m_a_val = self_run(env.clone());
            let f_clone = f.clone();
            m_a_val.bind(move |a_val: A| {
                let next_reader_t: Self::Bind<B> = f_clone(a_val);
                (next_reader_t.run_reader_t)(env.clone())
            })
        })
    }
}

impl<R, M, A> Monad<A> for ReaderT<R, M, A>
where
    R: Clone + 'static,
    M: Monad<A> + 'static,
    A: Clone + 'static,
    <M as Applicative<A>>::Applicative<A>: 'static,
{}

pub trait MonadReader<REnv, AVal> where Self: Sized {
    type SelfWithEnvAsValue;
    fn ask() -> Self::SelfWithEnvAsValue where REnv: Clone + 'static, Self::SelfWithEnvAsValue: Sized;
    fn local<FMapEnv>(map_env_fn: FMapEnv, computation: Self) -> Self
    where REnv: 'static, AVal: 'static, FMapEnv: Fn(REnv) -> REnv + 'static;
}

impl<R, M, A> MonadReader<R, A> for ReaderT<R, M, A>
where
    R: 'static,
    A: 'static,
    M: 'static,
    M: Applicative<R>,
    <M as Applicative<R>>::Applicative<R>: 'static,
{
    type SelfWithEnvAsValue = ReaderT<R, <M as Applicative<R>>::Applicative<R>, R>;
    fn ask() -> Self::SelfWithEnvAsValue where R: Clone + 'static {
        ReaderT::new(move |env: R| M::pure(env.clone()))
    }
    fn local<FMapEnv>(map_env_fn: FMapEnv, computation: Self) -> Self
    where FMapEnv: Fn(R) -> R + 'static {
        let computation_run = computation.run_reader_t.clone();
        ReaderT::new(move |current_env: R| {
            let modified_env = map_env_fn(current_env);
            computation_run(modified_env)
        })
    }
}
