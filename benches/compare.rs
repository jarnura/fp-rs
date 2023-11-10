use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use fp_rs::functor::Functor;

pub fn add_one(c: &mut Criterion) {
    let mut group = c.benchmark_group("Add_one");
    let input = Some(1);

    group.bench_with_input(BenchmarkId::new("functor_map", 1), &input, |b, &s| {
        b.iter(|| Functor::__map(s, |x| x + 1))
    });

    group.bench_with_input(BenchmarkId::new("option_map", 1), &input, |b, &s| {
        b.iter(|| Option::map(s, |x| x + 1))
    });

    group.finish();
}

criterion_group!(benches, add_one);
criterion_main!(benches);
