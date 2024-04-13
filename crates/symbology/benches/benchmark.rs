use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::Rng;
use regex::Regex;
use std::str::FromStr;
use symbology::figi::Figi; // Ensure this path correctly points to the Figi type

fn is_valid_figi_regex(figi: &str) -> bool {
    let pattern = r"^[B-DF-HJ-NP-TV-Z]{2}G[B-DF-HJ-NP-TV-Z0-9]{8}\d$";
    let re = Regex::new(pattern).unwrap();

    if re.is_match(figi) {
        let first_two = &figi[0..2];
        !(first_two == "BS"
            || first_two == "BM"
            || first_two == "GG"
            || first_two == "GB"
            || first_two == "GH"
            || first_two == "KY"
            || first_two == "VG")
    } else {
        false
    }
}

fn is_valid_figi(figi: &str) -> bool {
    if figi.len() != 12 {
        return false;
    }

    let consonants = "BCDFGHJKLMNPQRSTVWXYZ";
    let digits = "0123456789";

    // Check characters 1 and 2
    let first_two = &figi[0..2];
    if first_two == "BS"
        || first_two == "BM"
        || first_two == "GG"
        || first_two == "GB"
        || first_two == "GH"
        || first_two == "KY"
        || first_two == "VG"
    {
        return false;
    }
    if !first_two.chars().all(|c| consonants.contains(c)) {
        return false;
    }

    // Check character 3
    if figi.chars().nth(2).unwrap() != 'G' {
        return false;
    }

    // Check characters 4-11
    if !figi[3..11]
        .chars()
        .all(|c| consonants.contains(c) || digits.contains(c))
    {
        return false;
    }

    // Check character 12 (check digit)
    if !figi.chars().last().unwrap().is_ascii_digit() {
        return false;
    }

    true
}

// Generate a random valid FIGI
fn generate_random_figi() -> String {
    let consonants = "BCDFGHJKLMNPQRSTVWXYZ";
    let digits = "0123456789";
    let mut rng = rand::thread_rng();

    let mut figi = String::new();

    // Generate the first two characters (excluding restricted combinations)
    loop {
        let first_two: String = (0..2)
            .map(|_| {
                consonants
                    .chars()
                    .nth(rng.gen_range(0..consonants.len()))
                    .unwrap()
            })
            .collect();
        if !["BS", "BM", "GG", "GB", "GH", "KY", "VG"].contains(&first_two.as_str()) {
            figi.push_str(&first_two);
            break;
        }
    }

    // Add the third character 'G'
    figi.push('G');

    // Generate characters 4-11 (consonants and digits)
    for _ in 0..8 {
        if rng.gen_bool(0.5) {
            figi.push(
                consonants
                    .chars()
                    .nth(rng.gen_range(0..consonants.len()))
                    .unwrap(),
            );
        } else {
            figi.push(digits.chars().nth(rng.gen_range(0..digits.len())).unwrap());
        }
    }

    // Generate the check digit
    figi.push(digits.chars().nth(rng.gen_range(0..digits.len())).unwrap());

    figi
}

fn bench_figi_group(c: &mut Criterion) {
    let mut group = c.benchmark_group("figi");

    // Generate random FIGIs for benchmarking
    let figi = "BBG000BLNNH6";

    group.bench_with_input("imperative", figi, |b, figi| b.iter(|| is_valid_figi(figi)));
    group.bench_with_input("regex", figi, |b, figi| {
        b.iter(|| is_valid_figi_regex(figi))
    });
    group.bench_with_input("parse", figi, |b, figi| b.iter(|| Figi::from_str(figi)));

    group.finish();
}

// You can define more specific benchmarks here, targeting different aspects or functions related to FIGIs

criterion_group!(benches, bench_figi_group);
criterion_main!(benches);
