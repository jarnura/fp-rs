// Content from the original classic module in src/functor.rs
use crate::function::{CFn, CFnOnce};

/// Represents a type constructor that can be mapped over.
///
/// The `Functor` trait provides a `map` operation that allows a function `A -> B`
/// to be applied to a value within a context `F<A>`, producing a value in a
/// context `F<B>`. This is a fundamental concept in functional programming
/// for working with "boxed" or "contextual" values.
///
/// Implementors of `Functor` must satisfy two laws:
/// 1. **Identity**: `map(x, |a| a) == x` (more precisely, `<Type<A> as Functor<A>>::map(x, |a| a) == x`)
/// 2. **Composition**: `<Type<B> as Functor<B>>::map(<Type<A> as Functor<A>>::map(x, f), g) == <Type<A> as Functor<A>>::map(x, |a| g(f(a)))`
///
/// The associated type `Functor<T>` represents the generic structure of the functor
/// itself, allowing `map` to transform the inner type while preserving the structure.
pub trait Functor<A> {
    /// The associated type representing the structure of the Functor.
    type Functor<T>;

    /// Applies a function to a value within a context.
    fn map<B, Func>(self, f: Func) -> <Self as Functor<A>>::Functor<B>
    where
        Func: Fn(A) -> B + Clone + 'static;
}

impl<A: 'static> Functor<A> for Option<A> {
    type Functor<T> = Option<T>;

    fn map<B, Func>(self, f: Func) -> <Self as Functor<A>>::Functor<B>
    where
        Func: FnMut(A) -> B + 'static,
    {
        self.map(f)
    }
}

impl<A: 'static, E: 'static> Functor<A> for Result<A, E> {
    type Functor<T> = Result<T, E>;

    fn map<B, Func>(self, f: Func) -> <Self as Functor<A>>::Functor<B>
    where
        Func: FnMut(A) -> B + 'static,
    {
        self.map(f)
    }
}

impl<A: 'static> Functor<A> for Vec<A> {
    type Functor<T> = Vec<T>;

    fn map<B, Func>(self, f: Func) -> <Self as Functor<A>>::Functor<B>
    where
        Func: FnMut(A) -> B + 'static,
    {
        self.into_iter().map(f).collect()
    }
}

impl<X: 'static, A: 'static> Functor<A> for CFn<X, A> {
    type Functor<T> = CFnOnce<X, T>;

    fn map<B, Func>(self, mut f: Func) -> Self::Functor<B>
    where
        Func: FnMut(A) -> B + 'static,
    {
        CFnOnce::new(move |x: X| f(self.call(x)))
    }
}

impl<X: 'static, A: 'static> Functor<A> for CFnOnce<X, A> {
    type Functor<T> = CFnOnce<X, T>;

    fn map<B, Func>(self, mut f: Func) -> Self::Functor<B>
    where
        Func: FnMut(A) -> B + 'static,
    {
        CFnOnce::new(move |x: X| f(self.call_once(x)))
    }
} // Closing for impl Functor for CFnOnce
