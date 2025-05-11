// HKT tests are now default
mod hkt_tests {
    use fp_rs::identity::Identity;
    use fp_rs::kind_based::markers::{IdentityHKTMarker, OptionHKTMarker};
    use fp_rs::transformers::reader::hkt::ReaderT;
    // HKT traits
    use fp_rs::applicative::hkt::Applicative;
    use fp_rs::monad::hkt::{Bind, Monad}; // Monad trait itself might not be used directly in laws if bind/pure are sufficient

    // Helper to run ReaderT with Identity inner monad and compare
    fn run_hkt_reader_t_identity<R: Clone + 'static, A: PartialEq + std::fmt::Debug + Clone + 'static>(
        reader: ReaderT<R, IdentityHKTMarker, A>,
        env: R,
    ) -> Identity<A> {
        // Assuming ReaderT is Clone because it uses Rc internally
        (reader.run_reader_t)(env)
    }

    // Helper to run ReaderT with Option inner monad and compare
    fn run_hkt_reader_t_option<R: Clone + 'static, A: PartialEq + std::fmt::Debug + Clone + 'static>(
        reader: ReaderT<R, OptionHKTMarker, A>,
        env: R,
    ) -> Option<A> {
        (reader.run_reader_t)(env)
    }

    #[test]
    fn test_hkt_reader_t_monad_laws_identity_inner() {
        type Env = i32;
        type A = i32;
        type B = String;
        type C = usize;

        let env_val: Env = 10;
        let a_val: A = 5;

        // f: A -> ReaderT<Env, IdentityHKTMarker, B>
        let f = move |x: A| -> ReaderT<Env, IdentityHKTMarker, B> {
            ReaderT::new(move |env: Env| -> Identity<B> {
                Identity(format!("f: val={}, env={}", x, env))
            })
        };

        // g: B -> ReaderT<Env, IdentityHKTMarker, C>
        let g = move |s: B| -> ReaderT<Env, IdentityHKTMarker, C> {
            ReaderT::new(move |env: Env| -> Identity<C> {
                Identity(s.len() + env as usize)
            })
        };

        // m: ReaderT<Env, IdentityHKTMarker, A>
        let m: ReaderT<Env, IdentityHKTMarker, A> =
            ReaderT::new(move |env: Env| Identity(a_val + env));

        // Law 1: pure(a).bind(f) == f(a)
        let pure_a: ReaderT<Env, IdentityHKTMarker, A> =
            ReaderT::<Env, IdentityHKTMarker, A>::pure(a_val);
        let left_law1 = pure_a.bind(f);
        let right_law1 = f(a_val);
        assert_eq!(
            run_hkt_reader_t_identity(left_law1.clone(), env_val),
            run_hkt_reader_t_identity(right_law1.clone(), env_val)
        );

        // Law 2: m.bind(pure) == m
        let left_law2 = m
            .clone()
            .bind(|x: A| ReaderT::<Env, IdentityHKTMarker, A>::pure(x));
        let right_law2 = m.clone();
        assert_eq!(
            run_hkt_reader_t_identity(left_law2, env_val),
            run_hkt_reader_t_identity(right_law2, env_val)
        );

        // Law 3: m.bind(f).bind(g) == m.bind(|x| f(x).bind(g))
        let left_law3 = m.clone().bind(f).bind(g);
        let right_law3 = m.bind(move |x: A| f(x).bind(g)); // f, g are Fn, so they are Copy/Clone if they capture Copy/Clone types
        assert_eq!(
            run_hkt_reader_t_identity(left_law3.clone(), env_val),
            run_hkt_reader_t_identity(right_law3.clone(), env_val)
        );
    }

    #[test]
    fn test_hkt_reader_t_monad_laws_option_inner() {
        type Env = i32;
        type A = i32;
        type B = String;
        type C = usize;

        let env_val: Env = 10;
        let a_val: A = 5;

        // f_opt: A -> ReaderT<Env, OptionHKTMarker, B>
        let f_opt = move |x: A| -> ReaderT<Env, OptionHKTMarker, B> {
            ReaderT::new(move |env: Env| -> Option<B> {
                if x > 0 && env > 0 {
                    Some(format!("f_opt: val={}, env={}", x, env))
                } else {
                    None
                }
            })
        };

        // g_opt: B -> ReaderT<Env, OptionHKTMarker, C>
        let g_opt = move |s: B| -> ReaderT<Env, OptionHKTMarker, C> {
            ReaderT::new(move |env: Env| -> Option<C> {
                if !s.is_empty() && env > 0 {
                    Some(s.len() + env as usize)
                } else {
                    None
                }
            })
        };

        // m_opt_some: ReaderT<Env, OptionHKTMarker, A> (yields Some)
        let m_opt_some: ReaderT<Env, OptionHKTMarker, A> =
            ReaderT::new(move |env: Env| if env > 0 { Some(a_val + env) } else { None });

        // m_opt_none: ReaderT<Env, OptionHKTMarker, A> (yields None)
        let m_opt_none: ReaderT<Env, OptionHKTMarker, A> =
            ReaderT::new(move |_env: Env| None::<A>);

        // --- Test with m_opt_some ---
        // Law 1: pure(a).bind(f) == f(a)
        let pure_a_opt: ReaderT<Env, OptionHKTMarker, A> =
            ReaderT::<Env, OptionHKTMarker, A>::pure(a_val);
        let left_law1_opt = pure_a_opt.bind(f_opt);
        let right_law1_opt = f_opt(a_val);
        assert_eq!(
            run_hkt_reader_t_option(left_law1_opt.clone(), env_val),
            run_hkt_reader_t_option(right_law1_opt.clone(), env_val)
        );

        // Law 2: m.bind(pure) == m
        let left_law2_opt = m_opt_some
            .clone()
            .bind(|x: A| ReaderT::<Env, OptionHKTMarker, A>::pure(x));
        let right_law2_opt = m_opt_some.clone();
        assert_eq!(
            run_hkt_reader_t_option(left_law2_opt, env_val),
            run_hkt_reader_t_option(right_law2_opt, env_val)
        );

        // Law 3: m.bind(f).bind(g) == m.bind(|x| f(x).bind(g))
        let left_law3_opt = m_opt_some.clone().bind(f_opt).bind(g_opt);
        let right_law3_opt = m_opt_some.bind(move |x: A| f_opt(x).bind(g_opt));
        assert_eq!(
            run_hkt_reader_t_option(left_law3_opt.clone(), env_val),
            run_hkt_reader_t_option(right_law3_opt.clone(), env_val)
        );

        // --- Test with m_opt_none (should propagate None) ---
        // Law 1: Test f_opt with a value that makes it None (doesn't involve m_opt_none directly for this form)
        let val_for_f_none: A = -1;
        let pure_val_for_f_none: ReaderT<Env, OptionHKTMarker, A> =
            ReaderT::<Env, OptionHKTMarker, A>::pure(val_for_f_none);
        let left_law1_f_none = pure_val_for_f_none.bind(f_opt);
        let right_law1_f_none = f_opt(val_for_f_none);
         assert_eq!(
            run_hkt_reader_t_option(left_law1_f_none.clone(), env_val),
            run_hkt_reader_t_option(right_law1_f_none.clone(), env_val)
        );
        assert_eq!(run_hkt_reader_t_option(right_law1_f_none.clone(), env_val), None);


        // Law 2: m.bind(pure) == m (with m_opt_none)
        let left_law2_none = m_opt_none
            .clone()
            .bind(|x: A| ReaderT::<Env, OptionHKTMarker, A>::pure(x));
        let right_law2_none = m_opt_none.clone();
        assert_eq!(
            run_hkt_reader_t_option(left_law2_none.clone(), env_val),
            run_hkt_reader_t_option(right_law2_none.clone(), env_val)
        );
        assert_eq!(run_hkt_reader_t_option(right_law2_none.clone(), env_val), None);

        // Law 3: m.bind(f).bind(g) == m.bind(|x| f(x).bind(g)) (with m_opt_none)
        let left_law3_none = m_opt_none.clone().bind(f_opt).bind(g_opt);
        let right_law3_none = m_opt_none.bind(move |x: A| f_opt(x).bind(g_opt));
        assert_eq!(
            run_hkt_reader_t_option(left_law3_none.clone(), env_val),
            run_hkt_reader_t_option(right_law3_none.clone(), env_val)
        );
        assert_eq!(run_hkt_reader_t_option(right_law3_none.clone(), env_val), None);
    }
}
