use std::ops::Deref;

type BFn<A, B> = Box<dyn Fn(A) -> B + 'static>;

pub struct CFn<A, B>(BFn<A, B>);

impl<A, B> CFn<A, B> {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(A) -> B + 'static,
    {
        CFn(Box::new(f))
    }
}

impl<A, B> Deref for CFn<A, B> {
    type Target = BFn<A, B>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait AnyFunction<A, B> {
    type Function: FnOnce(A) -> B;
}

impl<A, B> AnyFunction<A, B> for CFn<A, B> {
    type Function = BFn<A, B>;
}

fn compose<A: 'static, B: 'static, C: 'static>(f: BFn<A, B>, g: BFn<B, C>) -> BFn<A, C> {
    Box::new(move |x| g(f(x)))
}

impl<A: 'static, B: 'static, C: 'static> std::ops::Shr<CFn<B, C>> for CFn<A, B> {
    type Output = CFn<A, C>;
    fn shr(self, rhs: CFn<B, C>) -> Self::Output {
        CFn(compose(self.0, rhs.0))
    }
}

impl<A: 'static, B: 'static, C: 'static> std::ops::Shl<CFn<A, B>> for CFn<B, C> {
    type Output = CFn<A, C>;
    fn shl(self, rhs: CFn<A, B>) -> Self::Output {
        CFn(compose(rhs.0, self.0))
    }
}

#[macro_export]
macro_rules! fn0 {
    ($closure:expr) => {
        $crate::utils::CFn::new(|_| $closure())
    };
}

#[macro_export]
macro_rules! fn1 {
    ($closure:expr) => {
        $crate::utils::CFn::new(move |x| $closure(x))
    };
}

#[macro_export]
macro_rules! fn2 {
    ($closure:expr) => {
        |x| $crate::utils::CFn::new(move |y| $closure(x)(y))
    };
}
