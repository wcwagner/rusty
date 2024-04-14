use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::str::FromStr;
use symbology::figi::Figi; // Ensure this path correctly points to the Figi type
use symbology::ibrk_figi::Figi as FigiIbrk;

fn bench_figi_ibrk(c: &mut Criterion) {
    c.bench_function("figi_ibrk", |b| {
        // Use a representative FIGI value for benchmarking
        // This value should ideally cover typical use cases
        b.iter(|| FigiIbrk::from_str(black_box("BBG000BLNNH6")))
    });
}

// Here we define a function to benchmark the Figi parsing functionality
fn bench_figi_parse(c: &mut Criterion) {
    c.bench_function("figi_parse", |b| {
        // Use a representative FIGI value for benchmarking
        // This value should ideally cover typical use cases
        b.iter(|| Figi::from_str(black_box("BBG000BLNNH6")))
    });
}

criterion_group!(benches, bench_figi_parse, bench_figi_ibrk);
criterion_main!(benches);
