use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use fp_rs::functor::Functor;

// Benchmark for Option<T>
pub fn map_option(c: &mut Criterion) {
    let mut group = c.benchmark_group("Functor_Option_map");
    let input: Option<i32> = Some(1);
    let input_ref = &input; // Criterion's bench_with_input expects a reference

    group.bench_with_input(BenchmarkId::new("functor_map_option", 1), input_ref, |b, &s| {
        b.iter(|| <Option<i32> as Functor<i32>>::map(s, |x: i32| x + 1))
    });

    group.bench_with_input(BenchmarkId::new("native_option_map", 1), input_ref, |b, &s| {
        b.iter(|| Option::map(s, |x: i32| x + 1))
    });

    group.finish();
}

// Benchmark for Result<T, E>
pub fn map_result(c: &mut Criterion) {
    let mut group = c.benchmark_group("Functor_Result_map");
    let input: Result<i32, String> = Ok(1);
    let input_ref = &input; // input_ref is &Result<i32, String>

    group.bench_with_input(BenchmarkId::new("functor_map_result", 1), input_ref, |b, s_ref: &Result<i32, String>| {
        b.iter(|| <Result<i32, String> as Functor<i32>>::map(s_ref.clone(), |x: i32| x + 1))
    });

    group.bench_with_input(BenchmarkId::new("native_result_map", 1), input_ref, |b, s_ref: &Result<i32, String>| {
        b.iter(|| Result::map(s_ref.clone(), |x: i32| x + 1))
    });

    group.finish();
}

// Benchmark for Vec<T>
pub fn map_vec(c: &mut Criterion) {
    let mut group = c.benchmark_group("Functor_Vec_map");
    let input_vec: Vec<i32> = (0..100).collect(); // A slightly larger vector
    let input_vec_ref = &input_vec;

    group.bench_with_input(BenchmarkId::new("functor_map_vec", input_vec.len()), input_vec_ref, |b, s_vec| {
        // s_vec is &Vec<i32>
        b.iter(|| <Vec<i32> as Functor<i32>>::map(s_vec.clone(), |x: i32| x + 1)) // clone s_vec as map takes self
    });

    group.bench_with_input(BenchmarkId::new("native_vec_map", input_vec.len()), input_vec_ref, |b, s_vec| {
        // s_vec is &Vec<i32>
        b.iter(|| s_vec.iter().map(|&x| x + 1).collect::<Vec<i32>>())
    });

    group.finish();
}

use fp_rs::apply::Apply;
use fp_rs::monad::Bind; // Monad import removed
use fp_rs::function::CFn; // Import CFn

// Benchmark for Apply on Option<T>
pub fn apply_option(c: &mut Criterion) {
    let mut group = c.benchmark_group("Apply_Option_ap");
    let static_val_opt: Option<i32> = Some(1);
    // For `ap(self, f: Option<F>)`, self is Option<Value>, f is Option<Function>
    // We need to pass references to these to bench_with_input.
    // The function Option will be created fresh in each iter to avoid benchmarking Box::new repeatedly outside the hot loop.

    // Parameter for bench_with_input will be just the value Option's reference.
    // The function Option will be created inside b.iter().
    let val_opt_ref = &static_val_opt;

    group.bench_with_input(BenchmarkId::new("apply_ap_option", 1), val_opt_ref, |b, &val_s| {
        // val_s is Option<i32> here because Option<i32> is Copy.
        b.iter(|| {
            // CFn::new itself does a Box::new, so we pass the closure directly.
            let func_opt: Option<CFn<i32, i32>> = Some(CFn::new(|x: i32| x + 1));
            // Call is Apply::apply(self: Option<Value>, f: Option<Function>)
            <Option<i32> as Apply<i32>>::apply(val_s, func_opt)
        })
    });

    // For manual comparison, the function Option and value Option will be created inside b.iter
    // to include allocation costs, making it comparable to apply_ap_option's CFn(Box::new) allocation.
    group.bench_with_input(BenchmarkId::new("manual_ap_option", 1), val_opt_ref, |b, &val_s| {
        // val_s is Option<i32> (due to Copy)
        b.iter(|| {
            let f_option: Option<Box<dyn Fn(i32) -> i32>> = Some(Box::new(|x: i32| x + 1));
            // val_s is already Option<i32>
            match f_option {
                Some(f) => match val_s { // Use val_s directly
                    Some(v) => Some(f(v)),
                    None => None,
                },
                None => None,
            }
        })
    });
    group.finish();
}


// Benchmark for Monad on Option<T>
pub fn bind_option(c: &mut Criterion) {
    let mut group = c.benchmark_group("Monad_Option_bind");
    let input: Option<i32> = Some(1);
    let input_ref = &input;
    // let f = |x: i32| -> Option<i32> { Some(x + 1) }; // This variable is unused


    group.bench_with_input(BenchmarkId::new("monad_bind_option", 1), input_ref, |b, &s_opt| {
        // s_opt is Option<i32> because Option<i32> is Copy
        b.iter(|| <Option<i32> as Bind<i32>>::bind(s_opt, |x: i32| Some(x+1) ))
    });

    group.bench_with_input(BenchmarkId::new("native_option_and_then", 1), input_ref, |b, &s_opt| {
        b.iter(|| s_opt.and_then(|x: i32| Some(x+1)))
    });

    group.finish();
}


// Benchmark for Apply on Result<T, E>
pub fn apply_result(c: &mut Criterion) {
    let mut group = c.benchmark_group("Apply_Result_ap");
    let static_val_res: Result<i32, String> = Ok(1);
    let val_res_ref = &static_val_res;

    group.bench_with_input(BenchmarkId::new("apply_ap_result", 1), val_res_ref, |b, val_s_ref: &Result<i32, String>| {
        b.iter(|| {
            let func_res: Result<CFn<i32, i32>, String> = Ok(CFn::new(|x: i32| x + 1));
            <Result<i32, String> as Apply<i32>>::apply(val_s_ref.clone(), func_res)
        })
    });

    group.bench_with_input(BenchmarkId::new("manual_ap_result", 1), val_res_ref, |b, val_s_ref: &Result<i32, String>| {
        b.iter(|| {
            let f_res: Result<Box<dyn Fn(i32) -> i32>, String> = Ok(Box::new(|x: i32| x + 1));
            match f_res {
                Ok(f) => match val_s_ref.clone() {
                    Ok(v) => Ok(f(v)),
                    Err(e) => Err(e),
                },
                Err(e) => Err(e),
            }
        })
    });
    group.finish();
}

// Benchmark for Bind on Result<T, E>
pub fn bind_result(c: &mut Criterion) {
    let mut group = c.benchmark_group("Monad_Result_bind");
    let input: Result<i32, String> = Ok(1);
    let input_ref = &input;

    group.bench_with_input(BenchmarkId::new("monad_bind_result", 1), input_ref, |b, s_res_ref: &Result<i32, String>| {
        b.iter(|| <Result<i32, String> as Bind<i32>>::bind(s_res_ref.clone(), |x: i32| Ok(x + 1)))
    });

    group.bench_with_input(BenchmarkId::new("native_result_and_then", 1), input_ref, |b, s_res_ref: &Result<i32, String>| {
        b.iter(|| s_res_ref.clone().and_then(|x: i32| Ok(x + 1)))
    });
    group.finish();
}

// Benchmark for Apply on Vec<T>
pub fn apply_vec(c: &mut Criterion) {
    let mut group = c.benchmark_group("Apply_Vec_ap");
    let static_val_vec: Vec<i32> = (0..10).collect(); // Smaller vec for ap
    let val_vec_ref = &static_val_vec;

    group.bench_with_input(BenchmarkId::new("apply_ap_vec", static_val_vec.len()), val_vec_ref, |b, val_s_vec_ref: &Vec<i32>| {
        b.iter(|| {
            // Create a Vec of functions for ap. For simplicity, one function.
            let func_vec: Vec<CFn<i32, i32>> = vec![CFn::new(|x: i32| x + 1)];
            <Vec<i32> as Apply<i32>>::apply(val_s_vec_ref.clone(), func_vec)
        })
    });

    group.bench_with_input(BenchmarkId::new("manual_ap_vec", static_val_vec.len()), val_vec_ref, |b, val_s_vec_ref: &Vec<i32>| {
        b.iter(|| {
            let fs_vec: Vec<Box<dyn Fn(i32) -> i32>> = vec![Box::new(|x: i32| x + 1)];
            let mut result_vec = Vec::new();
            if val_s_vec_ref.is_empty() || fs_vec.is_empty() {
                // return result_vec; // cannot return inside iter
            } else {
                for f in &fs_vec {
                    for val_a in val_s_vec_ref.iter() {
                        result_vec.push(f(*val_a));
                    }
                }
            }
            result_vec
        })
    });
    group.finish();
}

// Benchmark for Bind on Vec<T>
pub fn bind_vec(c: &mut Criterion) {
    let mut group = c.benchmark_group("Monad_Vec_bind");
    let input_vec: Vec<i32> = (0..10).collect(); // Smaller vec for bind
    let input_vec_ref = &input_vec;

    group.bench_with_input(BenchmarkId::new("monad_bind_vec", input_vec.len()), input_vec_ref, |b, s_vec_ref: &Vec<i32>| {
        b.iter(|| <Vec<i32> as Bind<i32>>::bind(s_vec_ref.clone(), |x: i32| vec![x + 1, x + 2]))
    });

    group.bench_with_input(BenchmarkId::new("native_vec_flat_map", input_vec.len()), input_vec_ref, |b, s_vec_ref: &Vec<i32>| {
        b.iter(|| s_vec_ref.iter().flat_map(|&x| vec![x + 1, x + 2].into_iter()).collect::<Vec<i32>>())
    });
    group.finish();
}

criterion_group!(benches,
    map_option, map_result, map_vec,
    apply_option, apply_result, apply_vec,
    bind_option, bind_result, bind_vec
);
criterion_main!(benches);
