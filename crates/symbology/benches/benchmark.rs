use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::str::FromStr;
use symbology::figi::Figi; // Ensure this path correctly points to the Figi type
use symbology::figi_imperative::Figi as FigiImperative;
use symbology::ibrk_figi::Figi as FigiIbrk;

fn bench_figi_ibrk(c: &mut Criterion) {
    c.bench_function("figi_ibrk", |b| {
        // Use a representative FIGI value for benchmarking
        // This value should ideally cover typical use cases
        b.iter(|| {
            let figi = FigiIbrk::from_str(black_box("BBG000BLNNH6")).unwrap();
            let figi_string = black_box(String::from(&figi));
            criterion::black_box(figi_string); // Prevent optimization
        })
    });
}

fn bench_figi_parse(c: &mut Criterion) {
    c.bench_function("figi_parse", |b| {
        // Use a representative FIGI value for benchmarking
        // This value should ideally cover typical use cases
        b.iter(|| {
            let figi = Figi::from_str(black_box("BBG000BLNNH6")).unwrap();
            criterion::black_box(figi.0); // Prevent optimization
        })
    });
}

fn bench_figi_imperative(c: &mut Criterion) {
    c.bench_function("figi_imperative", |b| {
        // Use a representative FIGI value for benchmarking
        // This value should ideally cover typical use cases
        b.iter(|| {
            let figi = FigiImperative::from_str(black_box("BBG000BLNNH6")).unwrap();
            criterion::black_box(figi.0); // Prevent optimization
        })
    });
}

criterion_group!(
    benches,
    bench_figi_parse,
    bench_figi_ibrk,
    bench_figi_imperative
);
criterion_main!(benches);
