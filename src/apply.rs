use crate::{function::CFn, functor::Functor};

/// Represents a type constructor that can apply a wrapped function to a wrapped value.
///
/// `Apply` extends `Functor`. It provides an `apply` method (often denoted as `<*>`)
/// which takes a functor holding a function `F<A -> B>` and a functor holding a value `F<A>`,
/// and returns a functor holding the result `F<B>`.
///
/// This allows for applying functions that are themselves within a context to values
/// that are also within a context.
///
/// Implementors of `Apply` must satisfy the law of composition:
/// `apply(apply(map(x, compose), y), z) == apply(x, apply(y, z))`
/// where `compose` is a function that performs function composition.
/// (A more precise formulation involves curried functions and `map`).
///
/// A simpler way to think about the law is:
/// `(.) <$> u <*> v <*> w == u <*> (v <*> w)` in Haskell-like notation,
/// or `map(compose, u).apply(v).apply(w) == u.apply(v.apply(w))`
///
/// The associated type `Apply<T>` represents the generic structure of the Apply instance.
/// The associated type `Fnn<T, U>` represents the type of the (unwrapped) function `T -> U`
/// that is expected to be inside the functor context.
pub trait Apply<A>: Functor<A> {
    /// The associated type representing the structure of the Apply instance.
    /// For example, if `Self` is `Option<A>`, then `Apply<T>` would be `Option<T>`.
    type Apply<T>;

    /// The type of the (unwrapped) function `T -> U` expected within the functor context.
    /// For this library, this is typically `CFn<T, U>`, a boxed, clonable function.
    type Fnn<T, U>; // Removed AnyFunction bound

    /// Applies a wrapped function to a wrapped value.
    ///
    /// Given `self` of type `F<A>` and `i` of type `F<A -> B>` (where `F` is `Self::Apply`
    /// and `A -> B` is `Self::Fnn<A, B>`), `apply` extracts the function from `i`
    /// and the value from `self`, applies the function to the value, and returns
    /// the result wrapped in `F<B>`.
    ///
    /// # Parameters
    /// - `self`: The Apply instance containing the value(s) of type `A`.
    /// - `i`: An Apply instance (of the same type constructor) containing a function
    ///        of type `Self::Fnn<A, B>`.
    ///
    /// # Returns
    /// A new Apply instance of type `Self::Apply<B>` containing the result(s)
    /// of applying the wrapped function to the wrapped value(s).
    ///
    /// # Examples
    ///
    /// ```
    /// use fp_rs::apply::Apply;
    /// use fp_rs::functor::Functor; // Apply extends Functor
    /// use fp_rs::function::CFn;
    ///
    /// // For Option
    /// let opt_val: Option<i32> = Some(5);
    /// let opt_fn: Option<CFn<i32, i32>> = Some(CFn::new(|x| x + 1));
    /// let applied_opt = <Option<i32> as Apply<i32>>::apply(opt_val, opt_fn);
    /// assert_eq!(applied_opt, Some(6));
    ///
    /// let none_val: Option<i32> = None;
    /// let opt_fn_again: Option<CFn<i32, i32>> = Some(CFn::new(|x| x + 1));
    /// assert_eq!(<Option<i32> as Apply<i32>>::apply(none_val, opt_fn_again), None);
    ///
    /// let opt_val_again: Option<i32> = Some(5);
    /// let none_fn: Option<CFn<i32, i32>> = None;
    /// assert_eq!(<Option<i32> as Apply<i32>>::apply(opt_val_again, none_fn), None);
    /// ```
    #[allow(clippy::type_complexity)]
    fn apply<B>(
        self,
        i: <Self as Functor<A>>::Functor<Self::Fnn<A, B>>, // Simplified this type
    ) -> <Self as Apply<A>>::Apply<B>
    where
        Self: Sized,
        B: 'static, // Added B: 'static, necessary for ReaderT and consistent for others
        // Add 'static bound back to trait definition to match ReaderT impl requirement
        <Self as Functor<A>>::Functor<Self::Fnn<A, B>>: 'static;
}

/// `Option<A>` as an `Apply`.
///
/// If both the `Option` value (`self`) and the `Option` function (`i`) are `Some`,
/// the function is applied to the value. Otherwise, `None` is returned.
///
/// The `A: 'static` bound is required due to the use of `CFn` which itself has 'static bounds.
impl<A: 'static> Apply<A> for Option<A> {
    type Apply<T> = Option<T>;
    type Fnn<T, U> = CFn<T, U>;

    fn apply<B>(self, i: Option<Self::Fnn<A, B>>) -> Option<B>
    where
        Self: Sized,
    {
        // self is Option<A>, i is Option<CFn<A,B>>
        // We need to get A from self and CFn<A,B> from i.
        // If both are Some, call the function.
        match self {
            Some(val_a) => match i {
                Some(func_ab) => Some(func_ab.call(val_a)), // CFn::call
                None => None,
            },
            None => None,
        }
        // A more concise way using Option's methods:
        // self.and_then(|val_a| i.map(|func_ab| func_ab.call(val_a)))
    }
}

/// `Result<A, E>` as an `Apply`.
///
/// If both the `Result` value (`self`) and the `Result` function (`i`) are `Ok`,
/// the function is applied to the value. If either is an `Err`, the first `Err`
/// encountered is returned.
///
/// The `A: 'static` and `E: 'static + Clone` bounds are required. `Clone` for `E`
/// is needed because an error might need to be propagated if `self` is `Ok` but `i` is `Err`.
impl<A: 'static, E: 'static + Clone> Apply<A> for Result<A, E> {
    type Apply<T> = Result<T, E>;
    type Fnn<T, U> = CFn<T, U>;

    fn apply<B>(self, i: Result<Self::Fnn<A, B>, E>) -> Result<B, E>
    where
        Self: Sized,
    {
        // self is Result<A, E>, i is Result<CFn<A,B>, E>
        match self {
            Ok(val_a) => match i {
                Ok(func_ab) => Ok(func_ab.call(val_a)), // CFn::call
                Err(err_from_i) => Err(err_from_i),
            },
            Err(err_from_self) => Err(err_from_self),
        }
        // A more concise way:
        // self.and_then(|val_a| i.map(|func_ab| func_ab.call(val_a)))
        // This works because Result::and_then propagates errors correctly.
        // Result::map on `i` will transform Ok(func) to Ok(func(val_a)) or propagate Err.
    }
}

/// `Vec<A>` as an `Apply`.
///
/// Applies each function in the `Vec` of functions (`fs`) to each value in the
/// `Vec` of values (`self`). This results in a new `Vec` containing all possible
/// applications, effectively a Cartesian product of applications.
///
/// For example, if `self` is `[x, y]` and `fs` is `[f, g]`, the result is
/// `[f(x), f(y), g(x), g(y)]`. (Note: The example in the code iterates fs first,
/// so it would be `[f(x), f(y), g(x), g(y)]` if `self` is `[x,y]` and `fs` is `[f,g]`.
/// The current implementation is `[f(x), g(x), f(y), g(y)]` if `self` is `[x,y]` and `fs` is `[f,g]`.
/// Let's re-verify the order. The code is:
/// `for f_fn in fs { for val_a in self.iter() { result_vec.push((*f_fn)(val_a.clone())); } }`
/// So if `self = [v1, v2]` and `fs = [f1, f2]`, result is `[f1(v1), f1(v2), f2(v1), f2(v2)]`.
/// This is a common definition for `Apply` on collections.
///
/// The `A: 'static + Clone` bound is required. `Clone` for `A` is needed because
/// each value in `self` might be applied to multiple functions.
impl<A: 'static + Clone> Apply<A> for Vec<A> {
    type Apply<T> = Vec<T>;
    type Fnn<T, U> = CFn<T, U>;

    fn apply<B>(self, fs: Vec<Self::Fnn<A, B>>) -> Vec<B>
    where
        Self: Sized,
    {
        let mut result_vec = Vec::new();
        if self.is_empty() || fs.is_empty() {
            return result_vec; // If either is empty, the Cartesian product is empty.
        }
        // For each function in fs
        for f_fn in fs {
            // Apply it to each value in self
            for val_a in self.iter() {
                // CFn::call. val_a needs to be cloned as it's used in an inner loop.
                result_vec.push(f_fn.call(val_a.clone()));
            }
        }
        result_vec
        // A more functional way using flat_map:
        // fs.into_iter().flat_map(|f_fn| {
        //     self.iter().map(move |val_a| f_fn.call(val_a.clone()))
        // }).collect()
    }
}

/// Lifts a binary function to operate on values within an `Apply` context.
///
/// Given a curried function `A -> (B -> C)` and two `Apply` instances `F<A>` and `F<B>`,
/// `lift2` applies the function to the wrapped values, returning `F<C>`.
///
/// This is equivalent to `f <$> fa <*> fb` in Haskell-like notation, or
/// `fa.map(f).apply(fb)` if `f` is curried.
///
/// # Type Parameters
/// - `A`, `B`, `C`: The types of the arguments and result of the binary function.
/// - `A2B2C`: The type of the curried binary function `A -> CFn<B, C>`.
/// - `FB2C`: The intermediate type `F<CFn<B, C>>` after mapping the first argument.
/// - `FA`, `FB`, `FC`: The types of the `Apply` instances for `A`, `B`, and `C`.
///
/// # Arguments
/// - `func`: The curried binary function.
/// - `fa`: An `Apply` instance containing a value of type `A`.
/// - `fb`: An `Apply` instance containing a value of type `B`.
///
/// # Returns
/// An `Apply` instance `FC` containing the result of type `C`.
///
/// # Example
/// ```
/// use fp_rs::fn2;
/// use fp_rs::apply::lift2;
/// use fp_rs::function::CFn; // Required for CFn in function signature
///
/// // A curried function: (i32) -> CFn<i32, i32>
/// let curried_add = |x: i32| CFn::new(move |y: i32| x + y);
///
/// assert_eq!(lift2(curried_add, Some(1), Some(2)), Some(3));
/// assert_eq!(lift2(curried_add, None, Some(2)), None);
/// assert_eq!(lift2(curried_add, Some(1), None), None);
///
/// let vec_a = vec![1, 2];
/// let vec_b = vec![10, 20];
/// // Expected: [1+10, 1+20, 2+10, 2+20] = [11, 21, 12, 22]
/// assert_eq!(lift2(curried_add, vec_a, vec_b), vec![11, 21, 12, 22]);
/// ```
pub fn lift2<A, B, C: 'static, A2B2C, FB2C: 'static, FA, FB, FC>(func: A2B2C, fa: FA, fb: FB) -> FC
where
    A2B2C: Fn(A) -> CFn<B, C> + Clone + 'static, // func: A -> (B -> C), Added Clone
    FA: Functor<A, Functor<CFn<B, C>> = FB2C>, // fa.map(func) results in F<B -> C>
    FB: Apply<B, Functor<<FB as Apply<B>>::Fnn<B, C>> = FB2C, Apply<C> = FC>, // F<B -> C>.apply(fb) results in F<C>
{
    // Step 1: fa.map(func)
    //   `fa` is F<A>. `func` is A -> (B->C).
    //   `fa.map(func)` results in F<(B->C)>, which is of type FB2C.
    let f_b_to_c_in_fa = <FA as Functor<A>>::map(fa, func);

    // Step 2: fb.apply(f_b_to_c_in_fa)
    //   `fb` is F<B>. `f_b_to_c_in_fa` is F<(B->C)>.
    //   Applying F<(B->C)> to F<B> gives F<C>.
    //   Note: The Apply trait's apply method is defined as self.apply(f_self),
    //   so it should be `f_b_to_c_in_fa.apply(fb)` if FB2C implemented Apply.
    //   However, the standard definition is `F<A -> B> <*> F<A>`.
    //   The current trait is `F<A>.apply(F<A -> B>)`.
    //   So, it should be `fb.apply(f_b_to_c_in_fa)`.
    <FB as Apply<B>>::apply(fb, f_b_to_c_in_fa)
}

/// Lifts a ternary function to operate on values within an `Apply` context.
///
/// Given a curried function `A -> (B -> (C -> D))` and three `Apply` instances
/// `F<A>`, `F<B>`, and `F<C>`, `lift3` applies the function to the wrapped values,
/// returning `F<D>`.
///
/// This is equivalent to `f <$> fa <*> fb <*> fc` in Haskell-like notation.
///
/// # Example
/// ```
/// use fp_rs::apply::lift3;
/// use fp_rs::function::CFn;
///
/// let curried_add3 = |a: i32| CFn::new(move |b: i32| CFn::new(move |c: i32| a + b + c));
///
/// assert_eq!(lift3(curried_add3, Some(1), Some(2), Some(3)), Some(6));
/// assert_eq!(lift3(curried_add3, Some(1), None, Some(3)), None);
/// ```
pub fn lift3<A, B, C: 'static, D: 'static, A2B2C2D, FB2C2D: 'static, FC2D: 'static, FA, FB, FC, FD>(
    func: A2B2C2D,
    fa: FA,
    fb: FB,
    fc: FC,
) -> FD
where
    A2B2C2D: Fn(A) -> CFn<B, CFn<C, D>> + Clone + 'static, // A -> (B -> (C -> D)), Added Clone
    FA: Functor<A, Functor<CFn<B, CFn<C, D>>> = FB2C2D>,
    FB: Apply<B, Functor<<FB as Apply<B>>::Fnn<B, CFn<C, D>>> = FB2C2D, Apply<CFn<C, D>> = FC2D>,
    FC: Apply<C, Functor<<FC as Apply<C>>::Fnn<C, D>> = FC2D, Apply<D> = FD>,
{
    // fa.map(func) -> F<B -> (C -> D)>
    // fb.apply(F<B -> (C -> D)>) -> F<C -> D>
    // fc.apply(F<C -> D>) -> F<D>
    let f_b_to_c_to_d_in_fa = <FA as Functor<A>>::map(fa, func);
    let f_c_to_d_in_fb = <FB as Apply<B>>::apply(fb, f_b_to_c_to_d_in_fa);
    <FC as Apply<C>>::apply(fc, f_c_to_d_in_fb)
}

/// Combines two `Apply` actions, keeping only the result of the first.
///
/// This is often denoted as `<*` in Haskell. `fa <* fb` evaluates both `fa` and `fb`,
/// but discards the result of `fb`, yielding the result of `fa`.
/// If either action results in a "failure" (e.g., `None` for `Option`, `Err` for `Result`),
/// the failure is propagated.
///
/// # Arguments
/// - `fa`: The first `Apply` action, whose result is kept.
/// - `fb`: The second `Apply` action, whose result is discarded (but is still evaluated).
///
/// # Returns
/// An `Apply` instance containing the result of `fa` if both actions succeed.
///
/// # Example
/// ```
/// use fp_rs::apply::apply_first;
///
/// assert_eq!(apply_first(Some(1), Some(2)), Some(1));
/// assert_eq!(apply_first(None::<i32>, Some(1)), None);
/// assert_eq!(apply_first(Some(1), None::<i32>), None);
/// assert_eq!(apply_first(Option::<i32>::None, None::<i32>), None);
///
/// // For Vec:
/// // apply_first(vec![1,2], vec![10,20])
/// // = lift2(|x| |_y| x, vec![1,2], vec![10,20])
/// // = vec![1,2].map(|x| |_y| x) -> vec![ fn(_){1}, fn(_){2} ]
/// //   vec![10,20].apply( vec![ fn(_){1}, fn(_){2} ] )
/// //   apply for Vec: fs.flat_map(|f| self.map(|x| f(x)))
/// //   vec![ fn(_){1}, fn(_){2} ].flat_map(|curried_fn| vec![10,20].map(|val_b| curried_fn.call(val_b)))
/// //   curried_fn = fn(_){1} (i.e. |_y| 1)
/// //     vec![10,20].map(|val_b| (|_y|1)(val_b)) -> vec![1,1]
/// //   curried_fn = fn(_){2} (i.e. |_y| 2)
/// //     vec![10,20].map(|val_b| (|_y|2)(val_b)) -> vec![2,2]
/// //   flat_map combines these: vec![1,1,2,2]
/// // This matches Haskell's (<*) behavior for lists: [1,2] <* [10,20] == [1,1,2,2]
/// assert_eq!(apply_first(vec![1,2], vec![10,20]), vec![1,1,2,2]);
/// ```
pub fn apply_first<A, B, FA, FB, FB2A: 'static>(fa: FA, fb: FB) -> <FB as Apply<B>>::Apply<A>
where
    A: Copy + 'static, 
    B: 'static, // For CFn type in the closure
    FA: Functor<A, Functor<CFn<B, A>> = FB2A>,
    FB: Apply<B, Functor<<FB as Apply<B>>::Fnn<B, A>> = FB2A>, // Ensure Apply<A> is the output type
{
    // The function required by lift2 is `A -> CFn<B, C>`. Here C is A.
    // So, `A -> CFn<B, A>`.
    // This is `|x: A| CFn::new(move |_y: B| x)`
    let map_fn = |x: A| CFn::new(move |_y: B| x);
    lift2(map_fn, fa, fb)
}

/// Combines two `Apply` actions, keeping only the result of the second.
///
/// This is often denoted as `*>` in Haskell. `fa *> fb` evaluates both `fa` and `fb`,
/// but discards the result of `fa`, yielding the result of `fb`.
/// If either action results in a "failure" (e.g., `None` for `Option`, `Err` for `Result`),
/// the failure is propagated.
///
/// # Arguments
/// - `fa`: The first `Apply` action, whose result is discarded (but is still evaluated).
/// - `fb`: The second `Apply` action, whose result is kept.
///
/// # Returns
/// An `Apply` instance containing the result of `fb` if both actions succeed.
///
/// # Example
/// ```
/// use fp_rs::apply::apply_second;
///
/// assert_eq!(apply_second(Some(1), Some(2)), Some(2));
/// assert_eq!(apply_second(None::<&str>, Some(1)), None);
/// assert_eq!(apply_second(Some(1), None::<i8>), None);
/// assert_eq!(apply_second(Option::<i32>::None, None::<i8>), None);
///
/// // For Vec:
/// // apply_second(vec![1,2], vec![10,20])
/// // = lift2(|_x| |y| y, vec![1,2], vec![10,20])
/// // = vec![1,2].map(|_x| |y| y) -> vec![ fn(y){y}, fn(y){y} ] (two identical id functions for B)
/// //   vec![10,20].apply( vec![ fn(y){y}, fn(y){y} ] )
/// //   curried_fn = fn(y){y}
/// //     vec![10,20].map(|val_b| (fn(y){y})(val_b)) -> vec![10,20]
/// //   curried_fn = fn(y){y}
/// //     vec![10,20].map(|val_b| (fn(y){y})(val_b)) -> vec![10,20]
/// //   flat_map combines these: vec![10,20,10,20]
/// // This matches Haskell's (*>) behavior for lists: [1,2] *> [10,20] == [10,20,10,20]
/// assert_eq!(apply_second(vec![1,2], vec![10,20]), vec![10,20,10,20]);
/// ```
pub fn apply_second<A, B, FA, FB, FMapResult, ResultApplyB>(fa: FA, fb: FB) -> ResultApplyB
where
    A: 'static, // For the CFn argument type
    B: 'static, // For the CFn argument and return type
    FA: Functor<A, Functor<CFn<B, B>> = FMapResult>, 
    FB: Apply<B, Functor<<FB as Apply<B>>::Fnn<B, B>> = FMapResult, Apply<B> = ResultApplyB>, 
    FMapResult: Functor<<FB as Apply<B>>::Fnn<B, B>> + 'static, // Added 'static bound
{
    // The function to pass to map should be `A -> CFn<B,B>`.
    // This function is `|_ignored_a: A| CFn::new(|b_val: B| b_val)`.
    let map_fn = |_: A| CFn::new(|y: B| y);
    let mapped_fa: FMapResult = <FA as Functor<A>>::map(fa, map_fn);
    <FB as Apply<B>>::apply(fb, mapped_fa)
}
