// Imports needed for the tests, adjusted from src/kind_based/kind.rs context
use fp_rs::kind_based::kind::*; // Imports HKT, OptionHKTMarker, ResultHKTMarker, etc.

// The example module content
// Copied from src/kind_based/kind.rs

/// Example `Functor` trait using the `HKT` system.
/// `Self` is the HKT constructor (e.g., `OptionHKTMarker`).
/// `A` is the input type of the contained value.
/// `B` is the output type of the contained value after mapping.
pub trait FunctorExample<A, B>: HKT1 { // HKT1 implies Self: HKT
    fn map_example(input: Self::Applied<A>, func: impl Fn(A) -> B) -> Self::Applied<B>;
}

// Example implementation for OptionHKTMarker
impl<A, B> FunctorExample<A, B> for OptionHKTMarker {
    fn map_example(input: Option<A>, func: impl Fn(A) -> B) -> Option<B> {
        input.map(func)
    }
}

// Example implementation for ResultHKTMarker<E>
impl<A, B, E> FunctorExample<A, B> for ResultHKTMarker<E> {
    fn map_example(input: Result<A, E>, func: impl Fn(A) -> B) -> Result<B, E> {
        input.map(func)
    }
}

#[test]
fn test_functor_example_option() {
    let option_a: Option<i32> = Some(5);
    let option_b = OptionHKTMarker::map_example(option_a, |x| x * 2);
    assert_eq!(option_b, Some(10));

    let option_n: Option<i32> = None;
    let option_n_mapped = OptionHKTMarker::map_example(option_n, |x: i32| x * 2);
    assert_eq!(option_n_mapped, None);
}

#[test]
fn test_functor_example_result() {
    type MyResult<T> = Result<T, String>;
    type MyResultHKT = ResultHKTMarker<String>;

    let result_a: MyResult<i32> = Ok(5);
    let result_b = MyResultHKT::map_example(result_a, |x| x * 2);
    assert_eq!(result_b, Ok(10));

    let result_e: MyResult<i32> = Err("error".to_string());
    let result_e_mapped = MyResultHKT::map_example(result_e, |x: i32| x * 2);
    assert_eq!(result_e_mapped, Err("error".to_string()));
}
