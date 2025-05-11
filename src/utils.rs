/// Creates a `CFn` (boxed `Fn`) from a nullary (0-argument) closure.
///
/// The resulting `CFn` will take a dummy argument (e.g., `()`) which it ignores,
/// then calls the original nullary closure.
///
/// # Examples
/// ```
/// use monadify::fn0; // Macro import
/// use monadify::function::CFn; // For type annotation if needed
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
/// use monadify::fn1;
/// use monadify::function::CFn;
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
/// use monadify::fn2;
/// use monadify::function::CFn;
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
/// use monadify::fn3;
/// use monadify::function::CFn;
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
