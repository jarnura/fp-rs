// Imports needed for the tests, adjusted from src/kind_based/kind.rs context
use monadify::kind_based::kind::*; // Imports Kind, OptionKind, ResultKind, etc.

// The example module content
// Copied from src/kind_based/kind.rs

/// Example `Functor` trait using the `Kind` system.
/// `Self` is the Kind constructor (e.g., `OptionKind`).
/// `A` is the input type of the contained value.
/// `B` is the output type of the contained value after mapping.
pub trait FunctorExample<A, B>: Kind1 {
    // Kind1 implies Self: Kind
    fn map_example(input: Self::Of<A>, func: impl Fn(A) -> B) -> Self::Of<B>; // Changed Applied to Of
}

// Example implementation for OptionKind
impl<A, B> FunctorExample<A, B> for OptionKind {
    // Changed OptionHKTMarker to OptionKind
    fn map_example(input: Option<A>, func: impl Fn(A) -> B) -> Option<B> {
        input.map(func)
    }
}

// Example implementation for ResultKind<E>
impl<A, B, E> FunctorExample<A, B> for ResultKind<E> {
    // Changed ResultHKTMarker to ResultKind
    fn map_example(input: Result<A, E>, func: impl Fn(A) -> B) -> Result<B, E> {
        input.map(func)
    }
}

#[test]
fn test_functor_example_option_kind() {
    // Renamed test
    let option_a: Option<i32> = Some(5);
    let option_b = OptionKind::map_example(option_a, |x| x * 2); // Renamed Marker
    assert_eq!(option_b, Some(10));

    let option_n: Option<i32> = None;
    let option_n_mapped = OptionKind::map_example(option_n, |x: i32| x * 2); // Renamed Marker
    assert_eq!(option_n_mapped, None);
}

#[test]
fn test_functor_example_result_kind() {
    // Renamed test
    type MyResult<T> = Result<T, String>;
    type MyResultKind = ResultKind<String>; // Renamed HKT to Kind

    let result_a: MyResult<i32> = Ok(5);
    let result_b = MyResultKind::map_example(result_a, |x| x * 2); // Renamed Marker
    assert_eq!(result_b, Ok(10));

    let result_e: MyResult<i32> = Err("error".to_string());
    let result_e_mapped = MyResultKind::map_example(result_e, |x: i32| x * 2); // Renamed Marker
    assert_eq!(result_e_mapped, Err("error".to_string()));
}
