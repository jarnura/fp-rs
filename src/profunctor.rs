use crate::{function::CFn, fn1};

pub trait Profunctor<B, C> {
    type Pro<T, U>;

    fn dimap<A, D, A2B, C2D>(self, a2b: A2B, c2d: C2D) ->  Self::Pro<A,D>
    where
        A2B: Fn(A) -> B + 'static,
        C2D: Fn(C) -> D + 'static,
        A: 'static,
        B: 'static,
        C: 'static,
        D: 'static,
    ;
}

impl<B,C> Profunctor<B,C> for CFn<B, C> {
    type Pro<T, U> = CFn<T, U>;
    fn dimap<A, D, A2B, C2D>(self, a2b: A2B, c2d: C2D) ->  Self::Pro<A,D>
        where
            A2B: Fn(A) -> B + 'static,
            C2D: Fn(C) -> D + 'static,
            C: 'static,
            B: 'static,
            A: 'static,
            D: 'static,
    {
        CFn::new(a2b) >> self >> CFn::new(c2d)
    }
}

fn lcmap<A, B, C, F, Pbc, Pac>(a2b: F, profunctor: Pbc) -> Pac
where
    A: 'static,
    B: 'static,
    C: 'static,
    F: Fn(A) -> B + 'static,
    Pbc: Profunctor<B, C, Pro<A,C> = Pac>
    {
        profunctor.dimap(a2b, |x| x)
    }

fn rmap<A, B, C, F, Pab, Pac>(b2c: F, profunctor: Pab) -> Pac
where
    A: 'static,
    B: 'static,
    C: 'static,
    F: Fn(B) -> C + 'static,
    Pab: Profunctor<A, B, Pro<A,C> =Pac>
    {
        profunctor.dimap(|x| x, b2c)
    }

#[cfg(test)]
mod tests {
    use crate::fn1;

    use super::*;

    #[test]
    fn test_fn_dimap() {
        let closure = fn1!(|x: i32| format!("{x}"));
        let proclosure = closure.dimap(|x: i8| (x + 1).into(), |s| vec![s]);
        let result = proclosure(1);
        assert_eq!(result, vec!["2"])
    }

    #[test]
    fn test_fn_lcmap() {
        let profunctor = fn1!(|x: i32| format!("{x}"));
        let proclosure = lcmap(|x: i8| x as i32 + 1, profunctor);
        let result = proclosure(1);
        assert_eq!(result, "2")
    }

    #[test]
    fn test_fn_rmap() {
        let profunctor = fn1!(|x: i32| format!("{x}"));
        let proclosure = rmap(|s| vec![s], profunctor);
        let result = proclosure(1);
        assert_eq!(result, vec!["1"])
    }

    #[test]
    fn test_fn_rmap_with_identity() {
        let profunctor = fn1!(|x: i32| x);
        let proclosure = rmap(|s| vec![s], profunctor);
        let result = proclosure(1);
        assert_eq!(result, vec![1])
    }
}