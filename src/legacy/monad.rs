// Content from the original classic module in src/monad.rs
use crate::legacy::applicative::Applicative; // Point to legacy Applicative
use crate::legacy::apply::Apply;             // Point to legacy Apply

/// Legacy version of the `Monad` trait.
///
/// A `Monad` is a type that implements `Applicative` and `Bind`.
/// It represents computations that can be sequenced.
/// This version uses associated types.
pub trait Monad<A>: Applicative<A> + Bind<A> {}

impl<A: 'static> Monad<A> for Option<A> {}
impl<A: 'static, E: 'static + Clone> Monad<A> for Result<A, E> {}
impl<A: 'static + Clone> Monad<A> for Vec<A> {}

/// Legacy version of the `Bind` trait (often `flatMap` or `>>=`).
///
/// `Bind` extends `Apply` and allows sequencing operations where each operation
/// takes a normal value and returns a value wrapped in the monadic context.
/// This version uses associated types.
pub trait Bind<A>: Apply<A> {
    /// The type constructor for this `Bind` instance.
    /// E.g., if `Self` is `Option<A>`, then `Bind<T>` would be `Option<T>`.
    type Bind<T>;
    /// Sequentially composes computations within the monadic context.
    ///
    /// Takes a value in context (`self`) and a function `f: A -> F<B>`
    /// (where `F<B>` is `Self::Bind<B>`). It applies `f` to the unwrapped
    /// value and returns the resulting context.
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

/// Legacy helper free function for `Bind::bind`.
///
/// Allows calling `bind(f, ma)` instead of `ma.bind(f)`.
/// This version is for the legacy `Bind` trait.
pub fn bind<A, B, MA, MB, F>(f: F, ma: MA) -> MB
where
    F: Fn(A) -> MB + Clone + 'static,
    MA: Bind<A, Bind<B> = MB>, // MA implements Bind<A> and its associated Bind<B> type is MB
{
    ma.bind::<B, F>(f)
}

/// Legacy helper free function for `Monad::join` (or flattening).
///
/// Given `mma: F<F<A>>`, produces `F<A>`.
/// Implemented using `bind` and an identity function.
/// This version is for the legacy `Bind` trait.
pub fn join<A, M, MM>(mma: MM) -> M
where
    M: Bind<A, Bind<A> = M> + 'static, // M is the inner monad type, e.g. Option<A>
    MM: Bind<M, Bind<A> = M>,        // MM is the outer monad type, e.g. Option<Option<A>>
                                     // and its bind operation for M results in M.
{
    mma.bind::<A, _>(|x: M| x) // The function for bind is id: M -> M
}
