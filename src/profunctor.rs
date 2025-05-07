use std::{marker::PhantomData, ops::Deref}; // Removed DerefMut

use crate::{
    // fn1, // Removed
    function::{CFn, CFnOnce},
    // functor::Functor, // Removed
};

pub trait Profunctor<B, C> {
    type Pro<T, U>;

    fn dimap<A, D, A2B, C2D>(self, a2b: A2B, c2d: C2D) -> Self::Pro<A, D>
    where
        A2B: Fn(A) -> B + 'static, // Removed Clone
        C2D: Fn(C) -> D + 'static, // Removed Clone
        A: 'static,
        B: 'static,
        C: 'static,
        D: 'static;

    fn dimap_<A, D>(self, a2b: CFn<A, B>, c2d: CFn<C, D>) -> Self::Pro<A, D>
    where
        // A2B: CFn<A,B>,
        // C2D: Fn(C) -> D + 'static,
        A: 'static,
        B: 'static,
        C: 'static,
        D: 'static;
}

impl<B, C> Profunctor<B, C> for CFn<B, C> {
    type Pro<T, U> = CFn<T, U>;
    fn dimap<A, D, A2B, C2D>(self, a2b: A2B, c2d: C2D) -> Self::Pro<A, D>
    where
        A2B: Fn(A) -> B + 'static, // Removed Clone
        C2D: Fn(C) -> D + 'static, // Removed Clone
        C: 'static,
        B: 'static,
        A: 'static,
        D: 'static,
    {
        CFn::new(a2b) >> self >> CFn::new(c2d)
    }

    fn dimap_<A, D>(self, a2b: CFn<A, B>, c2d: CFn<C, D>) -> Self::Pro<A, D>
    where
        C: 'static,
        B: 'static,
        A: 'static,
        D: 'static,
    {
        a2b >> self >> c2d
    }
}

impl<B, C> Profunctor<B, C> for CFnOnce<B, C> {
    type Pro<T, U> = CFnOnce<T, U>;
    fn dimap<A, D, A2B, C2D>(self, a2b: A2B, c2d: C2D) -> Self::Pro<A, D>
    where
        A2B: Fn(A) -> B + 'static, // Added 'static back
        C2D: Fn(C) -> D + 'static, // Added 'static back
        C: 'static,
        B: 'static,
        A: 'static,
        D: 'static,
    {
        CFnOnce::new(a2b) >> self >> CFnOnce::new(c2d)
    }

    fn dimap_<A, D>(self, a2b: CFn<A, B>, c2d: CFn<C, D>) -> Self::Pro<A, D>
    where
        C: 'static,
        B: 'static,
        A: 'static,
        D: 'static,
    {
        CFnOnce::new(move |x| a2b.call(x)) >> self >> CFnOnce::new(move |x| c2d.call(x))
    }
}

pub trait Strong<A, B>: Profunctor<A, B> {
    fn first<C: 'static>(self) -> Self::Pro<(A, C), (B, C)>;
    fn second<C: 'static>(self) -> Self::Pro<(C, A), (C, B)>;
}

impl<A: 'static, B: 'static> Strong<A, B> for CFn<A, B> {
    fn first<C: 'static>(self) -> Self::Pro<(A, C), (B, C)> {
        CFn::new(move |(a, c)| (self(a), c)) // self: a2b function
    }

    fn second<C: 'static>(self) -> Self::Pro<(C, A), (C, B)> {
        CFn::new(move |(c, a)| (c, self(a)))
    }
}

pub trait Choice<A, B>: Profunctor<A, B> {
    fn left<C>(self) -> Self::Pro<Result<C, A>, Result<C, B>>;
    fn right<C>(self) -> Self::Pro<Result<A, C>, Result<B, C>>;
}

impl<A: 'static, B: 'static> Choice<A, B> for CFn<A, B> {
    fn left<C>(self) -> Self::Pro<Result<C, A>, Result<C, B>> {
        CFn::new(move |r| match r {
            Ok(c) => Ok(c),
            Err(a) => Err(self(a)),
        })
    }
    fn right<C>(self) -> Self::Pro<Result<A, C>, Result<B, C>> {
        CFn::new(move |r| match r {
            Ok(a) => Ok(self(a)),
            Err(c) => Err(c),
        })
    }
}

pub struct Optic<POuter: Profunctor<S, T>, PInner: Profunctor<A, B>, S, T, A, B> {
    pub optic: Box<dyn FnOnce(PInner) -> POuter>, // Made public
    _s: PhantomData<S>,
    _t: PhantomData<T>,
    _a: PhantomData<A>,
    _b: PhantomData<B>,
}

// Optic_ is not used in tests, keeping it private for now. If needed, will make pub.
struct Optic_<P: Profunctor<A, B, Pro<S, T> = P>, S, T, A, B> {
    optic: Box<dyn FnOnce(P) -> P>,
    _s: PhantomData<S>,
    _t: PhantomData<T>,
    _a: PhantomData<A>,
    _b: PhantomData<B>,
}

pub struct Lens<PO: Strong<S, T>, PI: Strong<A, B>, S, T, A, B>(pub Optic<PO, PI, S, T, A, B>); // Made tuple field public

// Lens_ is not used in tests, keeping it private for now.
struct Lens_<P: Strong<A, B, Pro<S, T> = P>, S, T, A, B>(Optic_<P, S, T, A, B>);

impl<PO: Strong<S, T>, PI: Strong<A, B>, S, T, A, B> Deref for Lens<PO, PI, S, T, A, B> {
    type Target = Optic<PO, PI, S, T, A, B>;
    fn deref(&self) -> &Optic<PO, PI, S, T, A, B> {
        &self.0
    }
}

pub type Fold<R, S, T, A, B> = Optic<Forget<R, S, T>, Forget<R, A, B>, S, T, A, B>; // Made public

pub type AGetter<S, T, A, B> = Fold<A, S, T, A, B>; // Made public

impl<PA: Strong<S, T>, PB: Strong<A, B>, S, T, A, B> From<Lens<PA, PB, S, T, A, B>>
    for Optic<PA, PB, S, T, A, B>
{
    fn from(value: Lens<PA, PB, S, T, A, B>) -> Self {
        value.0
    }
}

pub fn view<S, T, A: 'static, B>(lens: AGetter<S, T, A, B>, s: S) -> A { // Made public
    let forget = Forget {
        inner: CFn::new(|x| x),
        _forget: PhantomData,
    };
    ((lens.optic)(forget).inner)(s)
}

// type Forget<R, A, B> = Box<dyn FnOnce(A) -> R>;
pub struct Forget<R, A, B> { // Made public
    inner: CFn<A, R>,
    _forget: PhantomData<B>,
}

impl<R: 'static, B, C> Profunctor<B, C> for Forget<R, B, C> {
    type Pro<T, U> = Forget<R, T, U>;
    fn dimap<A, D, A2B, C2D>(self, a2b: A2B, _c2d: C2D) -> Self::Pro<A, D>
    where
        A2B: Fn(A) -> B + 'static, // Removed Clone
        C2D: Fn(C) -> D + 'static, // Removed Clone
        A: 'static,
        B: 'static,
        D: 'static,
    {
        Forget {
            inner: CFn::new(a2b) >> self.inner,
            _forget: PhantomData,
        }
    }

    fn dimap_<A, D>(self, a2b: CFn<A, B>, _c2d: CFn<C, D>) -> Self::Pro<A, D> // Prefixed c2d with _
    where
        A: 'static,
        B: 'static,
        D: 'static,
    {
        Forget {
            inner: a2b >> self.inner, // _c2d is not used here
            _forget: PhantomData,
        }
    }
}

impl<R: 'static, B: 'static, C> Strong<B, C> for Forget<R, B, C> {
    fn first<D: 'static>(self) -> Self::Pro<(B, D), (C, D)> {
        Forget {
            inner: CFn::new(|(b, _c)| b) >> self.inner,
            _forget: PhantomData,
        }
    }

    fn second<D: 'static>(self) -> Self::Pro<(D, B), (D, C)> {
        Forget {
            inner: CFn::new(|(_b, c)| c) >> self.inner,
            _forget: PhantomData,
        }
    }
}

pub fn lens_<PO, PI, PBC, S: 'static, T: 'static, A: 'static, B: 'static>( // Made public
    to: CFn<S, (A, CFn<B, T>)>,
) -> Lens<PO, PBC, S, T, A, B>
where
    // PO: Strong<S, T, Pro<S, T> = <CFn<S, (A, CFn<B, T>)> as Profunctor<(B, CFn<B, S>), T>>::Pro<(B, CFn<B, S>), T>> + 'static,
    // // CFn<(B, CFn<B, S>), T>: Strong<S,T>,
    // PI: Strong<A, B, Pro<(A, S), (B, S)> = CFn<(A, CFn<B, T>), T>>
    //     + Strong<A, B, Pro<S, T> = PI>
    //     + 'static,
    // F: Profunctor<S, (A, CFn<B, T>), Pro<(B, CFn<B, S>), T> = PO> + 'static,
    PO: Strong<S, T>,
    PI: Profunctor<(A, CFn<B, T>), (B, CFn<B, T>), Pro<S, T> = PO>,
    PBC: Strong<A, B, Pro<(A, CFn<B, T>), (B, CFn<B, T>)> = PI>,
{
    let optics = |pbc: PBC| PI::dimap(PBC::first(pbc), move |val| to.call(val), |(b, f)| f(b)); // Added move here
    Lens(Optic {
        optic: Box::new(optics),
        _s: PhantomData,
        _t: PhantomData,
        _a: PhantomData,
        _b: PhantomData,
    })
}

pub fn lens<PO, PI, S: Copy + 'static, T: 'static, A: 'static, B: 'static>( // Already public
    s2a: CFn<S, A>, // CFn is already Clone
    s2b2t: CFn<S, CFn<B, T>>, // CFn is already Clone
) -> Lens<PO, PI, S, T, A, B>
where
    // PI: Strong<A, B, Pro<S, T> = PI>
    //     + Strong<A, B, Pro<(A, S), (B, S)> = CFn<(A, CFn<B, T>), T>>
    //     + 'static,
    // PO: Strong<S, T, Pro<S, T> = CFn<(B, CFn<B, S>), T>>,
    // // CFn<(B, CFn<B, S>), T>: Strong<S, T, Pro<S, T> = CFn<(B, CFn<B, S>), T>>,
    // <PI as Profunctor<A, B>>::Pro<(A, S), (B, S)>: Strong<A, B>,
    PO: Strong<S, T>,
    PI: Strong<A, B>,
    <PI as Profunctor<A, B>>::Pro<(A, CFn<B, T>), (B, CFn<B, T>)>:
        Profunctor<(A, CFn<B, T>), (B, CFn<B, T>), Pro<S, T> = PO>,
{
    let dimap_profunctor = CFn::new(move |s: S| {
        let get = s2a(s);
        let set = s2b2t(s);
        (get, set)
    });
    lens_(dimap_profunctor)
}

// fn lens<PO, PI, S: Clone + Copy + 'static, T: 'static, A: 'static, B: 'static>(
//     s2a: CFn<S, A>,
//     s2b2t: CFn<S, CFn<B, T>>,
// ) -> Lens<<CFn<(B, CFn<B, S>), T> as Profunctor<S, T>>::Pro<S, T>, PI, S, T, A, B>
// where
//     PI: Strong<A, B, Pro<S, T> = PI>
//         + Strong<A, B, Pro<(A, S), (B, S)> = CFn<(A, CFn<B, T>), T>>
//         + 'static,
//     // PO: Strong<S, T, Pro<S, T> = PO>,
//     CFn<(B, CFn<B, S>), T>: Strong<S, T, Pro<S, T> = CFn<(B, CFn<B, S>), T>>,
//     <PI as Profunctor<A, B>>::Pro<(A, S), (B, S)>: Strong<A, B>,
// {
//     let dimap_profunctor = CFn::new(move |s: S| {
//         let state = s;
//         let get = s2a(s.clone());
//         (get, s2b2t(state))
//     });
//     lens_(dimap_profunctor)
// }

pub fn _1< // Made public
    A: 'static,
    B: 'static,
    C: 'static,
    PA: Strong<A, B, Pro<(A, C), (B, C)> = PS> + 'static,
    PS: Strong<(A, C), (B, C)>,
>() -> Lens<PS, PA, (A, C), (B, C), A, B> {
    Lens(Optic {
        optic: Box::new(Strong::first),
        _s: PhantomData,
        _t: PhantomData,
        _a: PhantomData,
        _b: PhantomData,
    })
}

pub fn _2< // Made public
    A: 'static,
    B: 'static,
    C: 'static,
    PA: Strong<A, B, Pro<(C, A), (C, B)> = PS> + 'static,
    PS: Strong<(C, A), (C, B)>,
>() -> Lens<PS, PA, (C, A), (C, B), A, B> {
    Lens(Optic {
        optic: Box::new(Strong::second),
        _s: PhantomData,
        _t: PhantomData,
        _a: PhantomData,
        _b: PhantomData,
    })
}

fn _1_new<A: Copy + 'static, B: Copy + 'static, PO, PI>() -> Lens<PO, PI, (A, B), (A, B), A, B> // Removed A: Clone
where
    PI: Strong<A, B>,
    PO: Strong<(A, B), (A, B)>,
    <PI as Profunctor<A, B>>::Pro<(A, CFn<B, (A, B)>), (B, CFn<B, (A, B)>)>:
        Profunctor<(A, CFn<B, (A, B)>), (B, CFn<B, (A, B)>), Pro<(A, B), (A, B)> = PO>,
{
    lens(
        CFn::new(|(f, _s): (A, B)| f),
        CFn::new(|(keep, _)| CFn::new(move |new| (keep, new))),
    )
}

#[derive(Clone, Copy)]
pub struct Check { // Made public
    pub key: i8, // Made public
    pub other: i8 // Made public
}

pub fn _key<PO, PI>() -> Lens<PO, PI, Check, Check, i8, i8> // Already public
where
    PO: Strong<Check, Check>,
    PI: Strong<i8, i8>,
    <PI as Profunctor<i8, i8>>::Pro<(i8, CFn<i8, Check>), (i8, CFn<i8, Check>)>:
        Profunctor<(i8, CFn<i8, Check>), (i8, CFn<i8, Check>), Pro<Check, Check> = PO>,
{
    lens(
        CFn::new(|c: Check| c.key),
        CFn::new(|c: Check| CFn::new(move |key| Check { key, ..c })),
    )
}

// fn _1_new<A: Copy + 'static, B: Copy + 'static, PO, PI>() -> Lens<PO, PI, (A, B), (A, B), A, B>
// where
//     PI: Strong<A, B>,
//     PO: Strong<(A, B), (A, B)>,
//     <PI as Profunctor<A, B>>::Pro<(A, CFn<B, (A, B)>), (B, CFn<B, (A, B)>)>:
//         Profunctor<(A, CFn<B, (A, B)>), (B, CFn<B, (A, B)>), Pro<(A, B), (A, B)> = PO>,
// {
//     lens(
//         CFn::new(|(f, s): (A, B)| f),
//         CFn::new(|(keep, _)| CFn::new(move |new| (keep, new))),
//     )
// }

pub fn lcmap<A, B, C, F, Pbc, Pac>(a2b: F, profunctor: Pbc) -> Pac // Made public
where
    A: 'static,
    B: 'static,
    C: 'static,
    F: Fn(A) -> B + 'static, // Removed Clone
    Pbc: Profunctor<B, C, Pro<A, C> = Pac>,
{
    profunctor.dimap(a2b, |x| x)
}

pub fn rmap<A, B, C, F, Pab, Pac>(b2c: F, profunctor: Pab) -> Pac // Already public
where
    A: 'static,
    B: 'static,
    C: 'static,
    F: Fn(B) -> C + 'static, // Removed Clone
    Pab: Profunctor<A, B, Pro<A, C> = Pac>,
{
    profunctor.dimap(|x| x, b2c)
}
