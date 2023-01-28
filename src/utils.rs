pub trait AnyFunction<A, B> {
    type Function: FnOnce(A) -> B;
}

type BFn<A, B> = Box<dyn Fn(A) -> B + 'static>;

pub struct CFn<A, B>(BFn<A, B>);

impl<A, B> CFn<A, B> {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(A) -> B + 'static,
    {
        CFn(Box::new(f))
    }

    pub fn run(self, x: A) -> B {
        self.0(x)
    }
}

impl<A, B> AnyFunction<A, B> for CFn<A, B> {
    type Function = BFn<A, B>;
}

#[macro_export]
macro_rules! closure2 {
    ($closure:expr) => {
        |x| $crate::utils::CFn::new(move |y| $closure(x)(y))
    };
}

#[macro_export]
macro_rules! closure1 {
    ($closure:expr) => {
        $crate::utils::CFn::new(move |x| $closure(x))
    };
}
