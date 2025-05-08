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
        Func: FnMut(A) -> B + 'static; // Changed FnOnce to FnMut
}

impl<A: 'static> Functor<A> for Option<A> {
    // Added 'static bound to A here as well, often needed when dealing with 'static closures
    type Functor<T> = Option<T>;

    fn map<B, Func>(self, f: Func) -> <Self as Functor<A>>::Functor<B>
    where
        Func: FnMut(A) -> B + 'static, // Changed FnOnce to FnMut
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
        Func: FnMut(A) -> B + 'static, // Changed FnOnce to FnMut
    {
        self.map(f)
    }
}

impl<A: 'static> Functor<A> for Vec<A> {
    type Functor<T> = Vec<T>;

    fn map<B, Func>(self, f: Func) -> <Self as Functor<A>>::Functor<B>
    where
        Func: FnMut(A) -> B + 'static, // Changed FnOnce to FnMut
    {
        // Vec's iterator has a map method, which can be collected into a new Vec.
        // However, the function `f` is `FnOnce`. If we want to apply it to multiple elements,
        // it must be `FnMut` or `Fn`.
        // For `Functor`, `map` takes `self`, consuming the functor.
        // If `f` is `FnOnce`, it can be called once for each element if we can clone `f`.
        // But `FnOnce` is not necessarily `Clone`.
        // A standard `map` on a collection usually requires `FnMut` for the mapping function.
        // Let's re-evaluate the `FnOnce` constraint on `Func` for collections.
        // For `Option` and `Result`, `FnOnce` is fine because `f` is called at most once.
        // For `Vec`, `f` would be called for each element.
        // The standard library `Iterator::map` takes `F: FnMut(Self::Item) -> B`.
        //
        // Given the current trait definition `Func: FnOnce(A) -> B + 'static`,
        // to map over a `Vec<A>`, `f` would need to be `Clone` to be called for each element,
        // or the trait definition needs to be adjusted (e.g. `F: FnMut(A) -> B`).
        //
        // Let's assume the intention is to map `f` over each element.
        // If `f` is `FnOnce` and we want to map it over multiple elements, this implies
        // that `f` must be `Copy` or `Clone` to be called multiple times.
        // However, the `FnOnce` trait itself doesn't guarantee `Clone`.
        //
        // A common approach for `Vec`'s `map` (if we were defining it from scratch)
        // would be to take `FnMut`.
        //
        // Given the existing trait `Functor<A>::map<B, Func>(self, f: Func) where Func: FnOnce(A) -> B`,
        // the most straightforward way to implement this for `Vec<A>` is to use `into_iter()`
        // and then `map`. The closure passed to `iterator.map()` must be `FnMut`.
        // If our `f` is `FnOnce`, we can't directly pass it to `iterator.map()` if the iterator
        // produces more than one item.
        //
        // This suggests a potential mismatch in the `FnOnce` constraint for collection-like Functors.
        //
        // Let's look at other functional libraries for inspiration (e.g., Haskell, Scala).
        // Haskell's fmap: `fmap :: Functor f => (a -> b) -> f a -> f b` (no FnOnce/FnMut distinction)
        // Scala's map: `def map[B](f: A => B): F[B]`
        //
        // If `Func` must be `FnOnce(A) -> B + 'static`:
        // This means `f` can only be *called* once. This is problematic for `Vec`.
        //
        // Re-checking the `Functor` laws:
        // 1. Identity: `map id == id`
        // 2. Composition: `map (g . f) == map g . map f`
        //
        // If `f` is `FnOnce`, then `self.into_iter().map(f).collect()` is not directly possible
        // unless `f` is also `Copy` or `Clone`.
        //
        // The current `Functor` trait definition with `FnOnce` is more aligned with types
        // that are "consumed" and where the function is applied at most once to an inner value.
        //
        // Let's consider the implications. If `Vec<A>` is a `Functor`, and `map` takes `FnOnce`,
        // then `f` can only be called once. This would mean `Vec<A>` could only be mapped if it has
        // zero or one element, or if `f` is `Copy`. This seems too restrictive.
        //
        // Perhaps the `Functor` trait itself needs a different bound for `Func` for broader applicability,
        // e.g., `F: FnMut(A) -> B`.
        // Or, we can assume that for `Vec`, the `FnOnce` is meant to be callable for each element,
        // which implies it must be `Copy` or `Clone`.
        //
        // If we stick to `FnOnce` strictly, the implementation would be:
        // ```rust
        // self.into_iter().map(|a| f(a)).collect() // This won't compile if f is not Copy/Clone
        // ```
        //
        // Let's assume for now that the user of `Functor::map` on a `Vec` provides an `f` that
        // *can* be called multiple times, e.g., it's a simple closure that captures by reference
        // or is a function pointer, making it `Copy`.
        // Or, the `FnOnce` is a bit of a misnomer in the general trait and for collections,
        // it behaves like `FnMut`.
        //
        // The standard library `Iterator::map` takes `F: FnMut(Self::Item) -> B`.
        // If we want to use that, our `f` needs to be `FnMut`.
        //
        // Given the constraint `Func: FnOnce(A) -> B + 'static`, and `map` takes `self`,
        // the most idiomatic Rust way to map over a `Vec` is `self.into_iter().map(f).collect()`.
        // This requires `f` to be `FnMut` for the `Iterator::map` method.
        // If `Func` is `FnOnce`, it can be adapted to `FnMut` if `Func: FnMut`, but `FnOnce` is a supertrait of `FnMut`.
        // No, `Fn` is a subtrait of `FnMut`, which is a subtrait of `FnOnce`.
        // So, an `Fn` can be used where `FnMut` or `FnOnce` is expected.
        // An `FnMut` can be used where `FnOnce` is expected.
        // An `FnOnce` cannot necessarily be used where `FnMut` is expected if it consumes its environment on first call.
        //
        // This is a classic issue with HKT emulation and function traits in Rust.
        //
        // For now, let's proceed with the most direct translation, assuming `f` can be called multiple times.
        // This usually means `f` would need to be `Copy` or the `FnOnce` is effectively `FnMut`.
        // The `map` method on `Vec`'s iterator takes `FnMut`.
        // If `f` is truly `FnOnce` and not `FnMut`, this implementation is problematic for `Vec`s with >1 element.
        //
        // Let's assume the `Functor` trait's `FnOnce` is a general bound, and for collections,
        // the provided function `f` will typically be `FnMut` or `Fn` (which satisfy `FnOnce`).
        self.into_iter().map(f).collect()
    }
}

impl<B: 'static, C: 'static> Functor<C> for CFn<B, C> {
    type Functor<T> = CFnOnce<B, T>;

    fn map<D, Func>(self, mut g: Func) -> Self::Functor<D>
    // Removed mut self, g needs to be mut
    where
        Func: FnMut(C) -> D + 'static, // Changed FnOnce to FnMut
    {
        CFnOnce::new(move |x| g(self.call(x)))
    }
}

impl<B: 'static, C: 'static> Functor<C> for CFnOnce<B, C> {
    type Functor<T> = CFnOnce<B, T>;

    fn map<D, Func>(self, mut g: Func) -> Self::Functor<D>
    // g needs to be mut
    where
        Func: FnMut(C) -> D + 'static, // Changed FnOnce to FnMut
    {
        CFnOnce::new(move |x| g(self.call_once(x)))
    }
}
