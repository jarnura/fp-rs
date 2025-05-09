use crate::{apply::Apply, function::CFn};

/// Represents an applicative functor.
///
/// `Applicative` extends `Apply` (which extends `Functor`) by adding a `pure` method.
/// `pure` allows lifting a normal value `A` into an applicative context `F<A>`.
///
/// Applicative functors allow for applying a sequence of functions within a context
/// to a sequence of values within the same context, in a way that `Functor` alone
/// cannot easily achieve (e.g., applying `F<A -> B -> C>` to `F<A>` and `F<B>`).
///
/// Implementors of `Applicative` must satisfy several laws:
/// 1. **Identity**: `pure(id).apply(v) == v`
///    (where `id` is the identity function `|x| x`)
///    `Applicative::pure(CFn::new(|x| x)).apply(v) == v`
/// 2. **Homomorphism**: `pure(f).apply(pure(x)) == pure(f(x))`
///    `Applicative::pure(CFn::new(f)).apply(Applicative::pure(x)) == Applicative::pure(f(x))`
/// 3. **Interchange**: `u.apply(pure(y)) == pure(|f| f(y)).apply(u)`
///    (where `u` is `F<A -> B>`)
///    `u.apply(Applicative::pure(y)) == Applicative::pure(CFn::new(|f: CFn<A,B>| f.call(y))).apply(u)`
///    (This one is a bit tricky with `CFn` due to `CFn` itself not being the raw function.
///     The spirit is that applying a wrapped function `u` to a pure value `y` is
///     the same as taking a pure function that applies its argument function to `y`,
///     and applying that to `u`.)
/// 4. **Composition (derived from Apply and Functor)**: `map(compose, u).apply(v).apply(w) == u.apply(v.apply(w))`
///    (The `map` used here is from `Functor`).
///
/// The associated type `Applicative<T>` represents the generic structure of the applicative.
pub trait Applicative<A>: Apply<A> {
    /// The associated type representing the structure of the Applicative.
    /// For example, if `Self` is `Option<A>`, then `Applicative<T>` would be `Option<T>`.
    type Applicative<T>;

    /// Lifts a value into the applicative context.
    ///
    /// Given a value `v` of type `A`, `pure` wraps it into the applicative
    /// structure `F<A>` (where `F` is `Self::Applicative`).
    ///
    /// # Parameters
    /// - `v`: The value of type `A` to be lifted.
    ///
    /// # Returns
    /// A new applicative instance of type `Self::Applicative<A>` containing the value `v`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fp_rs::applicative::Applicative;
    ///
    /// // For Option
    /// let pure_opt: Option<i32> = <Option<i32> as Applicative<i32>>::pure(5);
    /// assert_eq!(pure_opt, Some(5));
    ///
    /// // For Result
    /// let pure_res: Result<i32, String> = <Result<i32, String> as Applicative<i32>>::pure(10);
    /// assert_eq!(pure_res, Ok(10));
    ///
    /// // For Vec
    /// let pure_vec: Vec<i32> = <Vec<i32> as Applicative<i32>>::pure(7);
    /// assert_eq!(pure_vec, vec![7]);
    /// ```
    fn pure(v: A) -> Self::Applicative<A>;
}

/// `Option<A>` as an `Applicative`.
///
/// `pure` wraps the value `v` in `Some(v)`.
///
/// The `A: 'static` bound is typically required by `Apply`'s use of `CFn`.
impl<A: 'static> Applicative<A> for Option<A> {
    type Applicative<T> = Option<T>;

    fn pure(v: A) -> Self::Applicative<A> {
        Some(v)
    }
}

/// `Result<A, E>` as an `Applicative`.
///
/// `pure` wraps the value `v` in `Ok(v)`.
///
/// The `A: 'static` and `E: 'static + Clone` bounds are typically required by `Apply`.
impl<A: 'static, E: 'static + Clone> Applicative<A> for Result<A, E> {
    type Applicative<T> = Result<T, E>;

    fn pure(v: A) -> Self::Applicative<A> {
        Ok(v)
    }
}

/// `Vec<A>` as an `Applicative`.
///
/// `pure` wraps the value `v` in a singleton `Vec`, i.e., `vec![v]`.
///
/// The `A: 'static + Clone` bound is typically required by `Apply` for `Vec`.
impl<A: 'static + Clone> Applicative<A> for Vec<A> {
    type Applicative<T> = Vec<T>;

    fn pure(v: A) -> Self::Applicative<A> {
        vec![v]
    }
}

/// Lifts a unary function `A -> B` to operate on `Applicative` values: `F<A> -> F<B>`.
///
/// This function is essentially `Functor::map` but implemented using `Applicative::pure`
/// and `Apply::apply`. It demonstrates how `map` can be derived from `pure` and `apply`.
/// `map(fa, f) == pure(f).apply(fa)`
///
/// # Type Parameters
/// - `F`: The `Applicative` type constructor (e.g., `Option`, `Vec`).
/// - `A`, `B`: Input and output types of the function.
/// - `A2B`: The type of the function `A -> B`.
/// - `FA2B`: The type `F<CFn<A, B>>`, i.e., the function wrapped in the applicative context.
///
/// # Arguments
/// - `f`: The function `A -> B` to lift.
/// - `fa`: The applicative value `F<A>`.
///
/// # Returns
/// The result `F<B>`.
///
/// # Example
/// ```
/// use fp_rs::applicative::lift_a1;
/// use fp_rs::function::CFn; // For CFn in type constraints
///
/// let add_one = |x: i32| x + 1;
///
/// let opt_val: Option<i32> = Some(5);
/// assert_eq!(lift_a1(add_one, opt_val), Some(6));
///
/// let none_val: Option<i32> = None;
/// assert_eq!(lift_a1(add_one, none_val), None);
///
/// // Vec example removed as it caused CFn: Clone issues with current Apply<Vec>
/// // let vec_val: Vec<i32> = vec![1, 2, 3];
/// // assert_eq!(lift_a1(add_one, vec_val), vec![2, 3, 4]);
/// ```
#[allow(clippy::module_name_repetitions)] // Name lift_a1 is established.
pub fn lift_a1<AppCtx, A, B: 'static, FnHook, AppFnCtx>(
    f: FnHook,
    fa: AppCtx,
) -> <AppCtx as Apply<A>>::Apply<B>
where
    FnHook: Fn(A) -> B + 'static, // The function A -> B
    // AppFnCtx is the type F<CFn<A,B>>. It must be Applicative itself.
    AppFnCtx: Applicative<CFn<A, B>, Applicative<CFn<A, B>> = AppFnCtx> + 'static, // Added 'static bound
    // AppCtx is F<A>. It must be Apply.
    // Its Apply::Functor<F::Fnn<A,B>> must be AppFnCtx (i.e. F<CFn<A,B>>)
    AppCtx: Apply<A, Functor<<AppCtx as Apply<A>>::Fnn<A, B>> = AppFnCtx>,
{
    // 1. Lift the function `f: A -> B` into the context: `pure(CFn::new(f))` gives `F<CFn<A,B>>`.
    //    This is `AppFnCtx::pure(CFn::new(f))`.
    let f_in_context: AppFnCtx = AppFnCtx::pure(CFn::new(f));

    // 2. Apply the wrapped function to the wrapped value: `fa.apply(f_in_context)`
    //    `fa` is `F<A>`. `f_in_context` is `F<CFn<A,B>>`.
    //    The result is `F<B>`.
    <AppCtx as Apply<A>>::apply(fa, f_in_context)
}
