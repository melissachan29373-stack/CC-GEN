use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ccgen_core::card::*;
use ccgen_core::generator::CardGenerator;

fn bench_generate_100(c: &mut Criterion) {
    let mut cgen = CardGenerator::new();
    let req = GenerateRequest {
        bin_pattern: "4xxxxxxxxxxxxx".to_string(),
        count: 100,
        include_expiry: true,
        include_cvv: true,
        format: OutputFormat::Pipe,
        unique: true,
        ..Default::default()
    };
    c.bench_function("generate_100_visa", |b| {
        b.iter(|| {
            black_box(cgen.generate(&req).unwrap());
        })
    });
}

fn bench_generate_1000(c: &mut Criterion) {
    let mut cgen = CardGenerator::new();
    let req = GenerateRequest {
        bin_pattern: "4xxxxxxxxxxxxx".to_string(),
        count: 1000,
        include_expiry: true,
        include_cvv: true,
        format: OutputFormat::CardOnly,
        unique: true,
        ..Default::default()
    };
    c.bench_function("generate_1000_visa", |b| {
        b.iter(|| {
            black_box(cgen.generate(&req).unwrap());
        })
    });
}

fn bench_generate_10000(c: &mut Criterion) {
    let mut cgen = CardGenerator::new();
    let req = GenerateRequest {
        bin_pattern: "4xxxxxxxxxxxxx".to_string(),
        count: 10000,
        include_expiry: false,
        include_cvv: false,
        format: OutputFormat::CardOnly,
        unique: false,
        ..Default::default()
    };
    c.bench_function("generate_10000_visa_no_extras", |b| {
        b.iter(|| {
            black_box(cgen.generate(&req).unwrap());
        })
    });
}

criterion_group!(benches, bench_generate_100, bench_generate_1000, bench_generate_10000);
criterion_main!(benches);
