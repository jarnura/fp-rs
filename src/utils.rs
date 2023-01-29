#[macro_export]
macro_rules! fn0 {
    ($closure:expr) => {
        $crate::function::CFn::new(|_| $closure())
    };
}

#[macro_export]
macro_rules! fn1 {
    ($closure:expr) => {
        $crate::function::CFn::new(move |x| $closure(x))
    };
}

#[macro_export]
macro_rules! fn2 {
    ($closure:expr) => {
        |x| $crate::function::CFn::new(move |y| $closure(x)(y))
    };
}

#[macro_export]
macro_rules! bfn0 {
    ($closure:expr) => {
        $crate::function::BindableFn::new(|_| $closure())
    };
}

#[macro_export]
macro_rules! bfn1 {
    ($closure:expr) => {
        $crate::function::BindableFn::new(move |x| $closure(x))
    };
}

#[macro_export]
macro_rules! bfn2 {
    ($closure:expr) => {
        |x| $crate::function::BindableFn::new(move |y| $closure(x)(y))
    };
}
