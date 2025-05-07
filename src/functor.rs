use crate::function::{CFn, CFnOnce};

/// A Functor
///
/// Functor trait supports an operation called map.
pub trait Functor<A> {
    /// The Associative type which acts a `* -> *`.
    ///  `*(Functor)` -> `*(T)`   
    type Functor<T>;

    /// Assume F is a `Functor`, then map can be used to apply a function `A -> B` on
    /// that `Functor<A>` or `F A`,  which produces `Functor<B>` or `F B`.
    fn map<B, Func>(self, f: Func) -> <Self as Functor<A>>::Functor<B>
    where
        Func: FnOnce(A) -> B + 'static; // Added + 'static back here
}

impl<A: 'static> Functor<A> for Option<A> { // Added 'static bound to A here as well, often needed when dealing with 'static closures
    type Functor<T> = Option<T>;

    fn map<B, Func>(self, f: Func) -> <Self as Functor<A>>::Functor<B>
    where
        Func: FnOnce(A) -> B + 'static, // Added + 'static to match trait
    {
        // Directly using Option's own map method.
        // This still works as non-'static closures can satisfy 'static if they don't capture non-'static data.
        // And Option::map itself doesn't impose 'static.
        self.map(f)
    }
}

impl<A: 'static, E: 'static> Functor<A> for Result<A, E> {
    type Functor<T> = Result<T, E>;

    fn map<B, Func>(self, f: Func) -> <Self as Functor<A>>::Functor<B>
    where
        Func: FnOnce(A) -> B + 'static,
    {
        self.map(f)
    }
}

impl<B: 'static, C: 'static> Functor<C> for CFn<B, C> {
    type Functor<T> = CFnOnce<B, T>;

    fn map<D, Func>(self, g: Func) -> Self::Functor<D>
    where
        Func: FnOnce(C) -> D + 'static,
    {
        CFnOnce::new(move |x| g(self.call(x)))
    }
}

impl<B: 'static, C: 'static> Functor<C> for CFnOnce<B, C> {
    type Functor<T> = CFnOnce<B, T>;

    fn map<D, Func>(self, g: Func) -> Self::Functor<D>
    where
        Func: FnOnce(C) -> D + 'static,
    {
        CFnOnce::new(move |x| g(self.call_once(x)))
    }
}
