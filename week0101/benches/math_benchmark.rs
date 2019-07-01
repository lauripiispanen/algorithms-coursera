extern crate week0101;

#[macro_use]
extern crate criterion;

use criterion::Criterion;
use criterion::black_box;

fn bitmath_benchmark(c: &mut Criterion) {
    c.bench_function("bitmath add", 
                     |b| b.iter(|| black_box(week0101::bitmath::add(
                         &vec![true, false, false, true, true, true, false, false, false, true, false, false],
                         &vec![true, true, true, false, true, false, true, false, false, true, true, false, false, false, false]))));
}

fn stringmath_benchmark(c: &mut Criterion) {
    c.bench_function("stringmath add", 
                     |b| b.iter(|| black_box(week0101::stringmath::add("2500", "30000"))));
}

fn plain_benchmark(c: &mut Criterion) {
    c.bench_function("u32 add", 
                     |b| b.iter(|| black_box(2500u32 + 30000u32)));
}

fn parse_benchmark(c: &mut Criterion) {
    c.bench_function("u32 parse", 
                     |b| b.iter(|| black_box("3250014512".parse::<u32>())));
}

criterion_group!(benches, bitmath_benchmark, stringmath_benchmark, plain_benchmark, parse_benchmark);
criterion_main!(benches);