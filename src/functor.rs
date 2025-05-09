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
    ///
    /// This type constructor `F<_>` allows `Functor` to be generic over the
    /// type it contains. For example, if `Self` is `Option<A>`, then
    /// `Functor<T>` would be `Option<T>`. It essentially specifies "what kind of box"
    /// this functor is, independent of the type of value inside the box.
    type Functor<T>;

    /// Applies a function to a value within a context.
    ///
    /// Given a value `self` of type `F<A>` (where `F` is `Self::Functor`),
    /// and a function `f` from `A` to `B`, `map` applies `f` to the inner value(s)
    /// of `self` to produce a new value of type `F<B>`.
    ///
    /// # Parameters
    /// - `self`: The functor instance containing the value(s) of type `A`.
    /// - `f`: A function that takes a value of type `A` and returns a value of type `B`.
    ///        This function must be callable multiple times if the functor contains multiple values (e.g., `Vec`).
    ///
    /// # Returns
    /// A new functor instance of type `Self::Functor<B>` containing the result(s)
    /// of applying `f`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fp_rs::functor::Functor;
    ///
    /// // For Option
    /// let opt_val: Option<i32> = Some(5);
    /// // UFCS (Universal Function Call Syntax) is used here to be explicit about calling the trait method.
    /// // Method syntax `opt_val.map(...)` might call Option's inherent `map` method.
    /// let mapped_opt = <Option<i32> as Functor<i32>>::map(opt_val, |x| x + 1);
    /// assert_eq!(mapped_opt, Some(6));
    ///
    /// let none_val: Option<i32> = None;
    /// let mapped_none = <Option<i32> as Functor<i32>>::map(none_val, |x| x + 1);
    /// assert_eq!(mapped_none, None);
    ///
    /// // For Vec
    /// let vec_val: Vec<i32> = vec![1, 2, 3];
    /// let mapped_vec = <Vec<i32> as Functor<i32>>::map(vec_val, |x| x * 2);
    /// assert_eq!(mapped_vec, vec![2, 4, 6]);
    /// ```
    fn map<B, Func>(self, f: Func) -> <Self as Functor<A>>::Functor<B>
    where
        Func: Fn(A) -> B + Clone + 'static; // Changed FnMut to Fn, added Clone
}

/// `Option<A>` as a `Functor`.
///
/// `map` applies the function `f` to the value inside `Some`,
/// or does nothing if it's `None`.
///
/// The `A: 'static` bound is required due to the `Func: ... + 'static` bound in the trait.
///
/// # Examples
///
/// ```
/// use fp_rs::functor::Functor;
///
/// let some_value: Option<i32> = Some(10);
/// assert_eq!(<Option<i32> as Functor<i32>>::map(some_value, |x| x.to_string()), Some("10".to_string()));
///
/// let no_value: Option<i32> = None;
/// assert_eq!(<Option<i32> as Functor<i32>>::map(no_value, |x| x.to_string()), None);
/// ```
impl<A: 'static> Functor<A> for Option<A> {
    // Added 'static bound to A here as well, often needed when dealing with 'static closures
    type Functor<T> = Option<T>;

    fn map<B, Func>(self, f: Func) -> <Self as Functor<A>>::Functor<B>
    where
        Func: FnMut(A) -> B + 'static, // Changed FnOnce to FnMut
    {
        // Directly using Option's own map method.
        // This works because an FnMut closure can be passed where FnOnce is expected.
        self.map(f)
    }
}

/// `Result<A, E>` as a `Functor`.
///
/// `map` applies the function `f` to the value inside `Ok`,
/// or leaves the `Err` untouched.
///
/// The `A: 'static` and `E: 'static` bounds are required due to the `Func: ... + 'static`
/// bound in the trait.
///
/// # Examples
///
/// ```
/// use fp_rs::functor::Functor;
///
/// let ok_value: Result<i32, String> = Ok(10);
/// assert_eq!(<Result<i32, String> as Functor<i32>>::map(ok_value, |x| x + 5), Ok(15));
///
/// let err_value: Result<i32, String> = Err("An error".to_string());
/// assert_eq!(<Result<i32, String> as Functor<i32>>::map(err_value, |x| x + 5), Err("An error".to_string()));
/// ```
impl<A: 'static, E: 'static> Functor<A> for Result<A, E> {
    type Functor<T> = Result<T, E>;

    fn map<B, Func>(self, f: Func) -> <Self as Functor<A>>::Functor<B>
    where
        Func: FnMut(A) -> B + 'static, // Changed FnOnce to FnMut
    {
        // Directly using Result's own map method.
        self.map(f)
    }
}

/// `Vec<A>` as a `Functor`.
///
/// `map` applies the function `f` to each element in the `Vec`,
/// producing a new `Vec` with the results.
///
/// The `A: 'static` bound is required due to the `Func: ... + 'static` bound in the trait.
///
/// # Examples
///
/// ```
/// use fp_rs::functor::Functor;
///
/// let values: Vec<i32> = vec![1, 2, 3];
/// assert_eq!(<Vec<i32> as Functor<i32>>::map(values, |x| x * x), vec![1, 4, 9]);
///
/// let empty_vec: Vec<i32> = vec![];
/// assert_eq!(<Vec<i32> as Functor<i32>>::map(empty_vec, |x| x * x), vec![]);
/// ```
impl<A: 'static> Functor<A> for Vec<A> {
    type Functor<T> = Vec<T>;

    fn map<B, Func>(self, f: Func) -> <Self as Functor<A>>::Functor<B>
    where
        Func: FnMut(A) -> B + 'static,
    {
        // Vec's iterator has a map method, which can be collected into a new Vec.
        // The standard library `Iterator::map` takes `F: FnMut(Self::Item) -> B`.
        // Our `Func` is `FnMut`, so this is compatible.
        self.into_iter().map(f).collect()
    }
}

/// `CFn<B, C>` (a reusable, boxed function `B -> C`) as a `Functor` over its output type `C`.
///
/// `map` composes the `CFn` instance (let's call it `f_bc: B -> C`) with the
/// provided function `g_cd: C -> D`.
/// The result is a new function `h_bd: B -> D` such that `h_bd(x) = g_cd(f_bc(x))`.
/// This is equivalent to function composition `g_cd . f_bc`.
///
/// The resulting composed function is returned as a `CFnOnce<B, D>`.
/// The `B: 'static` and `C: 'static` bounds are typically required for `CFn` and `CFnOnce`
/// due to their use of `Box<dyn Fn... + 'static>`.
///
/// # Examples
///
/// ```
/// use fp_rs::functor::Functor;
/// use fp_rs::function::CFn;
///
/// let add_one = CFn::new(|x: i32| x + 1);      // Represents f: i32 -> i32
/// let to_string_fn = |y: i32| y.to_string(); // Represents g: i32 -> String
///
/// // Map 'to_string_fn' over the output of 'add_one' using method syntax.
/// // `add_one.map(...)` is equivalent to `<CFn<_,_> as Functor<_>>::map(add_one, ...)`
/// let composed_fn = add_one.map(to_string_fn); // Results in h: CFnOnce<i32, String>
///
/// assert_eq!(composed_fn.call_once(5), "6".to_string()); // h(5) = to_string_fn(add_one(5)) = to_string_fn(6) = "6"
/// ```
impl<B: 'static, C: 'static> Functor<C> for CFn<B, C> {
    type Functor<T> = CFnOnce<B, T>; // Note: Result is CFnOnce, not CFn

    fn map<D, Func>(self, mut g: Func) -> Self::Functor<D>
    // g needs to be mut as it's FnMut
    where
        Func: FnMut(C) -> D + 'static,
    {
        // self is CFn<B, C>, self.call takes B returns C
        // g takes C returns D
        // result should be CFnOnce<B, D>
        CFnOnce::new(move |x: B| g(self.call(x)))
    }
}

/// `CFnOnce<B, C>` (a once-callable, boxed function `B -> C`) as a `Functor` over its output type `C`.
///
/// `map` composes the `CFnOnce` instance (let's call it `f_bc: B -> C`) with the
/// provided function `g_cd: C -> D`.
/// The result is a new function `h_bd: B -> D` such that `h_bd(x) = g_cd(f_bc(x))`.
/// This is equivalent to function composition `g_cd . f_bc`.
///
/// Since the original function `f_bc` is `FnOnce`, the resulting composed function
/// `h_bd` is also `FnOnce` (and returned as `CFnOnce<B, D>`).
/// The `B: 'static` and `C: 'static` bounds are typically required.
///
/// # Examples
///
/// ```
/// use fp_rs::functor::Functor;
/// use fp_rs::function::CFnOnce;
///
/// let add_one = CFnOnce::new(|x: i32| x + 1);      // Represents f: i32 -> i32 (callable once)
/// let to_string_fn = |y: i32| y.to_string();    // Represents g: i32 -> String
///
/// // Map 'to_string_fn' over the output of 'add_one' using method syntax.
/// let composed_fn = add_one.map(to_string_fn); // Results in h: CFnOnce<i32, String> (callable once)
///
/// assert_eq!(composed_fn.call_once(5), "6".to_string()); // h(5) = to_string_fn(add_one(5)) = to_string_fn(6) = "6"
/// ```
impl<B: 'static, C: 'static> Functor<C> for CFnOnce<B, C> {
    type Functor<T> = CFnOnce<B, T>;

    fn map<D, Func>(self, mut g: Func) -> Self::Functor<D>
    // g needs to be mut as it's FnMut
    where
        Func: FnMut(C) -> D + 'static,
    {
        // self is CFnOnce<B, C>, self.call_once takes B returns C
        // g takes C returns D
        // result should be CFnOnce<B, D>
        CFnOnce::new(move |x: B| g(self.call_once(x)))
    }
}
