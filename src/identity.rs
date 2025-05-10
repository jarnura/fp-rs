//! # Identity Monad
// Classic comments remain largely applicable.

#[cfg(not(feature = "kind"))]
mod classic {
    use crate::functor::Functor;
    use crate::apply::Apply;
    use crate::applicative::Applicative;
    use crate::monad::{Bind, Monad};
    use crate::function::CFn;

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Identity<A>(pub A);

    impl<A> Functor<A> for Identity<A> {
        type Functor<BVal> = Identity<BVal>;
        fn map<B, F>(self, mut f: F) -> Self::Functor<B>
        where
            F: FnMut(A) -> B + 'static,
        {
            Identity(f(self.0))
        }
    }

    impl<A: 'static> Apply<A> for Identity<A> {
        type Apply<T> = Identity<T>;
        type Fnn<T, U> = CFn<T, U>;
        fn apply<B>(self, i: Identity<Self::Fnn<A, B>>) -> Identity<B>
        where
            Self: Sized,
        {
            Identity(i.0.call(self.0))
        }
    }

    impl<A: 'static> Applicative<A> for Identity<A> {
        type Applicative<T> = Identity<T>;
        fn pure(v: A) -> Self::Applicative<A> {
            Identity(v)
        }
    }

    impl<A: 'static> Bind<A> for Identity<A> {
        type Bind<T> = Identity<T>;
        fn bind<B, F>(self, f: F) -> Self::Bind<B>
        where
            F: Fn(A) -> Self::Bind<B> + 'static,
        {
            f(self.0)
        }
    }

    impl<A: 'static> Monad<A> for Identity<A> {}
}

#[cfg(feature = "kind")]
pub mod hkt {
    //! # Higher-Kinded Type (HKT) Identity Monad
    //!
    //! This module provides the HKT implementation of the `Identity` monad.
    //! The `Identity` monad is the simplest monad; it just wraps a value without
    //! adding any computational context beyond the wrapping itself.
    //!
    //! It's often used as a base for monad transformers or to make non-monadic
    //! code fit into a monadic interface.
    //!
    //! ## Key Components
    //! - [`Identity<A>`]: The wrapper struct holding a value of type `A`.
    //! - [`IdentityHKTMarker`]: The HKT marker for `Identity`.
    //!
    //! ## Example
    //! ```
    //! use fp_rs::identity::hkt::{Identity, IdentityHKTMarker};
    //! use fp_rs::functor::hkt::Functor;
    //! use fp_rs::applicative::hkt::Applicative;
    //! use fp_rs::monad::hkt::{Bind, Monad};
    //!
    //! // Pure (from Applicative)
    //! let id_val: Identity<i32> = IdentityHKTMarker::pure(10);
    //! assert_eq!(id_val, Identity(10));
    //!
    //! // Map (from Functor)
    //! let mapped_id: Identity<String> = IdentityHKTMarker::map(id_val, |x| (x * 2).to_string());
    //! assert_eq!(mapped_id, Identity("20".to_string()));
    //!
    //! // Bind
    //! let bound_id: Identity<f64> = IdentityHKTMarker::bind(mapped_id, |s| Identity(s.len() as f64));
    //! assert_eq!(bound_id, Identity(2.0));
    //!
    //! // Join (from Monad)
    //! let nested_id: Identity<Identity<i32>> = Identity(Identity(100));
    //! let joined_id: Identity<i32> = IdentityHKTMarker::join(nested_id);
    //! assert_eq!(joined_id, Identity(100));
    //! ```

    use crate::kind_based::kind::{HKT, HKT1}; // HKT1 for the blanket impl
    use crate::functor::hkt as functor_hkt;
    use crate::apply::hkt as apply_hkt;
    use crate::applicative::hkt as applicative_hkt;
    use crate::monad::hkt as monad_hkt;
    use crate::function::CFn; // For Apply's function container

    /// A simple wrapper struct that holds a value of type `A`.
    ///
    /// This is the core data type for the `Identity` monad. It doesn't add
    /// any special context other than simply containing the value.
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct Identity<A>(pub A);

    /// The HKT marker for the `Identity` monad.
    ///
    /// This unit struct is used to implement the HKT traits (`Functor`, `Apply`,
    /// `Applicative`, `Monad`, `Bind`) for `Identity`.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct IdentityHKTMarker;

    impl HKT for IdentityHKTMarker {
        type Applied<T> = Identity<T>;
    }
    // HKT1 is implemented by the blanket impl in kind.rs for types that impl HKT.

    // HKT Functor for IdentityHKTMarker
    impl<A, B> functor_hkt::Functor<A, B> for IdentityHKTMarker
    where
        // A: 'static, B: 'static are not strictly needed here if func is FnMut
        // but the trait itself might impose them for broader compatibility.
        // The functor::Functor trait itself does not require 'static for A, B.
    {
        /// Applies a function to the value inside `Identity`.
        /// `Identity(a)` becomes `Identity(func(a))`.
        fn map(input: Identity<A>, mut func: impl FnMut(A) -> B) -> Identity<B> {
            Identity(func(input.0))
        }
    }

    // HKT Apply for IdentityHKTMarker
    impl<A, B> apply_hkt::Apply<A, B> for IdentityHKTMarker
    where
        A: 'static, // Required by Apply trait definition
        B: 'static, // Required by Apply trait definition
    {
        /// Applies a wrapped function `Identity<CFn<A, B>>` to a wrapped value `Identity<A>`.
        fn apply(
            value_container: Identity<A>,
            function_container: Identity<CFn<A, B>>,
        ) -> Identity<B> {
            Identity(function_container.0.call(value_container.0))
        }
    }

    // HKT Applicative for IdentityHKTMarker
    impl<T: 'static> applicative_hkt::Applicative<T> for IdentityHKTMarker
    // T: 'static is already on the Applicative trait
    {
        /// Lifts a value `T` into `Identity<T>`.
        fn pure(value: T) -> Identity<T> {
            Identity(value)
        }
    }

    // HKT Bind for IdentityHKTMarker
    impl<A, B> monad_hkt::Bind<A, B> for IdentityHKTMarker
    where
        A: 'static, // Required by Bind trait definition
        B: 'static, // Required by Bind trait definition
    {
        /// Applies a function `A -> Identity<B>` to the value inside `Identity<A>`.
        /// Effectively, it unwraps the value, applies the function, and returns the new `Identity<B>`.
        fn bind(input: Identity<A>, mut func: impl FnMut(A) -> Identity<B>) -> Identity<B> {
            func(input.0)
        }
    }

    // HKT Monad for IdentityHKTMarker
    impl<A: 'static> monad_hkt::Monad<A> for IdentityHKTMarker {
        /// Flattens `Identity<Identity<A>>` to `Identity<A>`.
        /// This simply unwraps the outer `Identity`.
        fn join(mma: Identity<Identity<A>>) -> Identity<A> {
            // mma is Identity(Identity(value))
            // mma.0 is Identity(value)
            // So, we return mma.0 directly.
            // However, the original code `Identity(mma.0.0)` is also correct
            // as it constructs a new Identity from the innermost value.
            // Let's stick to the simpler `mma.0` if it means the same type.
            // `mma.0` is `Identity<A>`, which is `Self::Applied<A>`.
            // `Identity(mma.0.0)` is also `Identity<A>`. Both are fine.
            // The original `Identity(mma.0.0)` is perhaps more explicit about reconstruction.
            mma.0 // This is Identity<A>
            // Or, more explicitly: Identity(mma.0.0)
        }
    }
}

// Re-export based on feature flag
#[cfg(not(feature = "kind"))]
pub use classic::Identity;

#[cfg(feature = "kind")]
pub use hkt::{Identity, IdentityHKTMarker};

// Ensure HKT1 is in scope for the blanket impl.
// This is usually handled by `use crate::kind_based::kind::HKT1;`
// in modules that define HKT markers, but good to be mindful.
// The `hkt` module itself imports HKT1.
