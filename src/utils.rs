/// Creates a `CFn` (boxed `Fn`) from a nullary (0-argument) closure.
///
/// The resulting `CFn` will take a dummy argument (e.g., `()`) which it ignores,
/// then calls the original nullary closure.
///
/// # Examples
/// ```
/// use fp_rs::fn0; // Macro import
/// use fp_rs::function::CFn; // For type annotation if needed
///
/// let get_number = fn0!(|| 42);
/// let result: i32 = get_number.call(()); // Call with a dummy unit argument
/// assert_eq!(result, 42);
///
/// let greet = fn0!(|| "Hello".to_string());
/// assert_eq!(greet.call(()), "Hello");
/// ```
#[macro_export]
macro_rules! fn0 {
    ($closure:expr) => {
        $crate::function::CFn::new(|_: ()| $closure()) // Explicitly type dummy arg as unit
    };
}

/// Creates a `CFn` (boxed `Fn`) from a unary (1-argument) closure.
///
/// This is a convenience macro for `CFn::new(closure)`.
///
/// # Examples
/// ```
/// use fp_rs::fn1;
/// use fp_rs::function::CFn;
///
/// let add_one = fn1!(|x: i32| x + 1);
/// assert_eq!(add_one.call(5), 6);
///
/// let to_string_fn = fn1!(|x: i32| x.to_string());
/// assert_eq!(to_string_fn.call(10), "10");
/// ```
#[macro_export]
macro_rules! fn1 {
    ($closure:expr) => {
        $crate::function::CFn::new(move |x| $closure(x))
    };
}

/// Creates a curried function of two arguments, wrapped in `CFn`.
///
/// Given a closure `|x| |y| expr`, `fn2!` transforms it into
/// `move |x| CFn::new(move |y| closure(x)(y))`.
/// The outer function takes `x` and returns a `CFn` that takes `y`.
///
/// # Examples
/// ```
/// use fp_rs::fn2;
/// use fp_rs::function::CFn;
///
/// let curried_add = fn2!(|x: i32| move |y: i32| x + y);
///
/// let add_5_fn = curried_add(5); // add_5_fn is CFn<i32, i32>
/// assert_eq!(add_5_fn.call(10), 15); // Calls the inner closure with y = 10
///
/// assert_eq!(curried_add(3).call(7), 10);
/// ```
#[macro_export]
macro_rules! fn2 {
    ($closure:expr) => {
        move |x| $crate::function::CFn::new(move |y| $closure(x)(y))
    };
}

/// Creates a curried function of three arguments, wrapped in nested `CFn`s.
///
/// Given a closure `|x| |y| |z| expr`, `fn3!` transforms it into
/// `move |x| CFn::new(move |y| CFn::new(move |z| closure(x)(y)(z)))`.
///
/// # Examples
/// ```
/// use fp_rs::fn3;
/// use fp_rs::function::CFn;
///
/// let curried_add3 = fn3!(|x: i32| move |y: i32| move |z: i32| x + y + z);
///
/// let add_5_and_10_fn = curried_add3(5).call(10); // add_5_and_10_fn is CFn<i32, i32>
/// assert_eq!(add_5_and_10_fn.call(20), 35);
///
/// assert_eq!(curried_add3(1)(2).call(3), 6);
/// ```
#[macro_export]
macro_rules! fn3 {
    ($closure:expr) => {
        move |x| {
            $crate::function::CFn::new(move |y| {
                $crate::function::CFn::new(move |z| $closure(x)(y)(z))
            })
        }
    };
}

/// Creates a `BindableFn` from a nullary (0-argument) closure that returns a Monad.
///
/// The resulting `BindableFn` will take a dummy argument (e.g., `()`) which it ignores,
/// then calls the original nullary closure.
/// `BindableFn` is used for functions `A -> M<B>`. Here, `A` is `()`.
///
/// Note: `BindableFn` usage is somewhat complex and might be less common after
/// direct `Fn(A) -> Monad<B>` was adopted in the `Bind` trait.
///
/// # Examples
/// ```
/// use fp_rs::bfn0;
/// use fp_rs::function::BindableFn; // For type annotation
/// use fp_rs::monad::Bind; // For BindType alias
///
/// // Assume Option<i32> implements Bind<i32> and Bind<()>
/// // TODO: The bfn0! macro with Option leads to trait bound errors (Bind<()> for Option<T>).
/// // This example needs to be revisited or BindableFn/bfn0 needs redesign for this case.
/// // The type of bfn_opt is BindableFn<Option<()>, (), i32>
/// // It wraps CFn<(), Option<i32>>
/// // let bfn_opt = bfn0!(|| Some(42));
/// // let result: Option<i32> = bfn_opt.call(());
/// // assert_eq!(result, Some(42));
/// ```
#[macro_export]
macro_rules! bfn0 {
    ($closure:expr) => {
        $crate::function::BindableFn::new(|_: ()| $closure()) // Explicitly type dummy arg as unit
    };
}

/// Creates a `BindableFn` from a unary (1-argument) closure that returns a Monad.
///
/// This is a convenience macro for `BindableFn::new(closure)`.
/// The closure should have the signature `A -> M<B>`.
///
/// # Examples
/// ```
/// use fp_rs::bfn1;
/// use fp_rs::function::BindableFn;
/// use fp_rs::monad::Bind;
///
/// // Type of safe_divide is BindableFn<Option<f64>, f64, f64>
/// // It wraps CFn<f64, Option<f64>>
/// let safe_divide_by_2 = bfn1!(|x: f64| if x == 0.0 { None } else { Some(10.0 / x) });
/// // TODO: BindableFn does not have a direct .call() method that aligns with Fn traits.
/// // These lines would require BindableFn to implement Fn or provide a similar call interface.
/// // assert_eq!(safe_divide_by_2.call(2.0), Some(5.0));
/// // assert_eq!(safe_divide_by_2.call(0.0), None);
/// ```
#[macro_export]
macro_rules! bfn1 {
    ($closure:expr) => {
        $crate::function::BindableFn::new(move |x| $closure(x))
    };
}

/// Creates a curried `BindableFn` of two arguments.
///
/// Given a closure `|x| |y| monad_expr`, `bfn2!` transforms it into
/// `move |x| BindableFn::new(move |y| closure(x)(y))`.
/// The outer function takes `x` and returns a `BindableFn` that takes `y`
/// and returns a monad.
///
/// # Examples
/// ```
/// use fp_rs::bfn2;
/// use fp_rs::function::BindableFn;
/// use fp_rs::monad::Bind;
///
/// // Example: A function (i32 -> (String -> Option<String>))
/// let curried_combine = bfn2!(|num: i32| move |text: String| {
///     if text.is_empty() {
///         None
///     } else {
///         Some(format!("{}: {}", num, text))
///     }
/// });
///
/// let combine_with_5 = curried_combine(5); // combine_with_5 is BindableFn<Option<String>, String, String>
///                                          // It wraps CFn<String, Option<String>>
///
/// // TODO: BindableFn does not have a direct .call() method that aligns with Fn traits.
/// // These lines would require BindableFn to implement Fn or provide a similar call interface.
/// // assert_eq!(combine_with_5.call("hello".to_string()), Some("5: hello".to_string()));
/// // assert_eq!(combine_with_5.call("".to_string()), None);
/// ```
#[macro_export]
macro_rules! bfn2 {
    ($closure:expr) => {
        |x| $crate::function::BindableFn::new(move |y| $closure(x)(y))
    };
}
