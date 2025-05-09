use crate::{applicative::Applicative, apply::Apply};

/// A marker trait indicating that a type is a Monad.
///
/// `Monad` extends `Applicative` (and thus `Apply` and `Functor`) by adding
/// capabilities for sequential computation where the result of one computation
/// can determine the next computation. This is primarily achieved through the `bind`
/// (or `flat_map`) operation, defined in the `Bind` trait.
///
/// A type `M<A>` is a Monad if it implements `Applicative<A>` and `Bind<A>`.
/// This trait itself does not add new methods but serves as a way to group
/// `Applicative` and `Bind` under a single concept.
///
/// Monads must satisfy three laws:
/// 1. **Left Identity**: `pure(a).bind(f) == f(a)`
///    `Monad::pure(a).bind(f) == f(a)`
/// 2. **Right Identity**: `m.bind(pure) == m`
///    `m.bind(|x| Monad::pure(x)) == m`
/// 3. **Associativity**: `m.bind(f).bind(g) == m.bind(|x| f(x).bind(g))`
///
/// These laws ensure that monadic computations can be composed predictably.
pub trait Monad<A>: Applicative<A> + Bind<A> {}

/// `Option<A>` is a `Monad`.
/// It inherits `Applicative` and `Bind` implementations for `Option`.
impl<A: 'static> Monad<A> for Option<A> {}

/// `Result<A, E>` is a `Monad`.
/// It inherits `Applicative` and `Bind` implementations for `Result`.
impl<A: 'static, E: 'static + Clone> Monad<A> for Result<A, E> {}

/// The `Bind` trait provides the `bind` operation (often called `flat_map` or `>>=`).
///
/// `bind` is the core of a Monad. It takes a monadic value `M<A>` and a function
/// `A -> M<B>`, and returns a monadic value `M<B>`. This allows sequencing
/// computations where the next computation depends on the result of the previous one.
pub trait Bind<A>: Apply<A> {
    /// The associated type representing the structure of the Monad.
    /// For example, if `Self` is `Option<A>`, then `Bind<T>` would be `Option<T>`.
    type Bind<T>;

    /// Sequentially composes two actions, passing the result of the first to the second.
    ///
    /// Given `self` of type `M<A>` and a function `f` of type `A -> M<B>`,
    /// `bind` applies `f` to the value inside `self` (if present and valid)
    /// and returns the resulting `M<B>`.
    ///
    /// # Parameters
    /// - `self`: The monadic value `M<A>`.
    /// - `f`: A function that takes a value of type `A` and returns a new
    ///        monadic value `Self::Bind<B>` (which is `M<B>`).
    ///
    /// # Returns
    /// A new monadic value `Self::Bind<B>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fp_rs::monad::{Bind, Monad}; // Monad for pure, Bind for bind
    ///
    /// // For Option
    /// let opt_val: Option<i32> = Some(3);
    /// let f_opt = |x: i32| if x > 0 { Some(x * 2) } else { None };
    /// assert_eq!(<Option<i32> as Bind<i32>>::bind(opt_val, f_opt), Some(6));
    ///
    /// let none_val: Option<i32> = None;
    /// assert_eq!(<Option<i32> as Bind<i32>>::bind(none_val, f_opt), None);
    ///
    /// // For Result
    /// let ok_val: Result<i32, String> = Ok(5);
    /// let f_res = |x: i32| if x > 0 { Ok(x.to_string()) } else { Err("Too small".to_string()) };
    /// assert_eq!(<Result<i32, String> as Bind<i32>>::bind(ok_val, f_res), Ok("5".to_string()));
    ///
    /// let err_val: Result<i32, String> = Err("Initial error".to_string());
    /// assert_eq!(<Result<i32, String> as Bind<i32>>::bind(err_val, f_res), Err("Initial error".to_string()));
    /// ```
    fn bind<B, F>(self, f: F) -> Self::Bind<B>
    where
        F: Fn(A) -> Self::Bind<B> + Clone + 'static; // Added Clone bound
}

/// `Option<A>` as `Bind`.
///
/// `bind` (equivalent to `Option::and_then`) applies the function `f` if `self` is `Some`,
/// otherwise returns `None`.
impl<A: 'static> Bind<A> for Option<A> {
    type Bind<T> = Option<T>;

    fn bind<B, F>(self, f: F) -> Self::Bind<B>
    where
        F: Fn(A) -> Self::Bind<B> + 'static,
    {
        self.and_then(f) // Option's and_then is exactly bind.
    }
}

/// `Result<A, E>` as `Bind`.
///
/// `bind` (equivalent to `Result::and_then`) applies the function `f` if `self` is `Ok`,
/// otherwise propagates the `Err`.
impl<A: 'static, E: 'static + Clone> Bind<A> for Result<A, E> {
    type Bind<T> = Result<T, E>;

    fn bind<B, F>(self, f: F) -> Self::Bind<B>
    where
        F: Fn(A) -> Self::Bind<B> + 'static,
    {
        self.and_then(f) // Result's and_then is exactly bind.
    }
}

/// `Vec<A>` as `Bind`.
///
/// `bind` applies the function `f` (which returns a `Vec<B>`) to each element of `self` (`Vec<A>`),
/// and concatenates (flattens) all the resulting `Vec<B>`s into a single `Vec<B>`.
/// This is equivalent to `flat_map` on iterators.
impl<A: 'static + Clone> Bind<A> for Vec<A> {
    type Bind<T> = Vec<T>;

    fn bind<B, F>(self, f: F) -> Self::Bind<B>
    where
        F: Fn(A) -> Self::Bind<B> + 'static, // Self::Bind<B> is Vec<B>
    {
        let mut result_vec = Vec::new();
        for a_val in self {
            // f(a_val) produces a Vec<B>
            let vec_b_for_a: Self::Bind<B> = f(a_val);
            result_vec.extend(vec_b_for_a);
        }
        result_vec
        // A more functional way:
        // self.into_iter().flat_map(f).collect()
    }
}

/// `Vec<A>` is a `Monad`.
/// It inherits `Applicative` and `Bind` implementations for `Vec`.
/// `A` needs to be `Clone` because `Applicative<A>` for `Vec<A>` requires it (due to `Apply` for `Vec`).
impl<A: 'static + Clone> Monad<A> for Vec<A> {}


/// A helper function to call `Bind::bind` with a slightly simpler signature for the caller.
///
/// This function takes an unboxed function `f: A -> MB` and a monadic value `ma: MA`,
/// and applies `ma.bind(f)`.
///
/// # Type Parameters
/// - `A`: The input type of the function `f`.
/// - `B`: The type parameter for the output monad `MB` (e.g., if `MB` is `Option<String>`, `B` is `String`).
/// - `MA`: The type of the input monadic value (e.g., `Option<A>`). Must implement `Bind<A>`.
/// - `MB`: The type of the output monadic value (e.g., `Option<B>`). This is `MA::Bind<B>`.
/// - `F`: The type of the function `A -> MB`.
///
/// # Arguments
/// - `f`: The function to apply within the bind operation.
/// - `ma`: The input monadic value.
///
/// # Returns
/// The result of `ma.bind(f)`.
///
/// # Example
/// ```
/// use fp_rs::monad; // Using the module to access the bind function
///
/// let opt_val: Option<i32> = Some(5);
/// let f = |x: i32| if x > 0 { Some((x * 10).to_string()) } else { None };
///
/// // monad::bind is used here, not the trait method directly.
/// let result: Option<String> = monad::bind(f, opt_val);
/// assert_eq!(result, Some("50".to_string()));
///
/// let none_val: Option<i32> = None;
/// let result_none: Option<String> = monad::bind(f, none_val);
/// assert_eq!(result_none, None);
/// ```
pub fn bind<A, B, MA, MB, F>(f: F, ma: MA) -> MB
where
    F: Fn(A) -> MB + Clone + 'static, // Added Clone
    MA: Bind<A, Bind<B> = MB>, // MA is M<A>, MB is M<B>
{
    // Call the trait method.
    // The type B for bind::<B, _> is inferred from MB (MA::Bind<B> = MB).
    // The type F for bind::<_, F> is inferred from the argument f.
    ma.bind::<B, F>(f)
}


/// Flattens a nested monadic value. `join(M<M<A>>)` becomes `M<A>`.
///
/// `join` is a fundamental monadic operation that can be defined in terms of `bind`:
/// `join(mma) == mma.bind(|ma| ma)` (i.e., `mma.bind(identity_function)`).
/// Conversely, `bind` can be defined in terms of `join` and `map`:
/// `ma.bind(f) == join(ma.map(f))`.
///
/// # Type Parameters
/// - `A`: The inner type of the monad (e.g., if `M` is `Option<String>`, `A` is `String`).
/// - `M`: The type of the "inner" and "outer" monad (e.g., `Option<A>`).
///        It must implement `Bind<A>` such that its `Bind<A>` associated type is itself (`M`).
/// - `MM`: The type of the nested monadic value (e.g., `Option<Option<A>>`).
///         It must implement `Bind<M>` such that its `Bind<A>` associated type is `M`.
///
/// # Arguments
/// - `mma`: The nested monadic value `M<M<A>>`.
///
/// # Returns
/// The flattened monadic value `M<A>`.
///
/// # Example
/// ```
/// use fp_rs::monad::join;
///
/// let nested_opt: Option<Option<i32>> = Some(Some(5));
/// assert_eq!(join(nested_opt), Some(5));
///
/// let nested_opt_inner_none: Option<Option<i32>> = Some(None);
/// assert_eq!(join(nested_opt_inner_none), None);
///
/// let nested_opt_outer_none: Option<Option<i32>> = None;
/// assert_eq!(join(nested_opt_outer_none), None);
///
/// let nested_vec: Vec<Vec<i32>> = vec![vec![1, 2], vec![3, 4]];
/// // join for Vec is concatenation:
/// assert_eq!(join(nested_vec), vec![1, 2, 3, 4]);
///
/// let nested_res: Result<Result<i32, String>, String> = Ok(Ok(10));
/// assert_eq!(join(nested_res), Ok(10));
///
/// let nested_res_inner_err: Result<Result<i32, String>, String> = Ok(Err("Inner error".to_string()));
/// assert_eq!(join(nested_res_inner_err), Err("Inner error".to_string()));
///
/// let nested_res_outer_err: Result<Result<i32, String>, String> = Err("Outer error".to_string());
/// assert_eq!(join(nested_res_outer_err), Err("Outer error".to_string()));
/// ```
pub fn join<A, M, MM>(mma: MM) -> M
where
    // M is the "inner" monad type, M<A>.
    // It must be Bind<A> and its associated Bind<A> type must be M itself.
    // e.g., M = Option<String>, A = String. Option<String> is Bind<String> and Bind<String>::Bind<String> is Option<String>.
    M: Bind<A, Bind<A> = M> + 'static, // M is M<A>

    // MM is the "outer" monad type, M<M<A>>.
    // It must be Bind over the type M (which is M<A>).
    // Its associated Bind<A> (for the *flattened* result) must be M.
    // e.g., MM = Option<Option<String>>. It is Bind<Option<String>>.
    // And Bind<Option<String>>::Bind<String> (the result type after flattening) is Option<String>.
    MM: Bind<M, Bind<A> = M>, // MM is M<M<A>>, its Bind<A> (after flattening) is M<A>
{
    // The function passed to bind is `id: M -> M`.
    // `mma` is `M<M<A>>`.
    // `bind` expects a function `F: Fn(InnerType) -> ResultMonad`.
    // Here, `InnerType` of `mma` is `M` (which is `M<A>`).
    // The `ResultMonad` should be `M` (which is `M<A>`).
    // So, the function is `Fn(M) -> M`. The identity function `|x: M| x` fits.
    // The `B` in `mma.bind::<B, _>` refers to the `A` in the *final* `M<A>`.
    mma.bind::<A, _>(|x: M| x)
}
