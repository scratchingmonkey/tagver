use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tagver::{calculate_version, CalculationResult, Config, Version};

fn benchmark_version_calculation(c: &mut Criterion) {
    c.bench_function("version_calculation_no_tags", |b| {
        b.iter(|| {
            let config = Config::default();
            let work_dir = ".";
            let _result = calculate_version(work_dir, &config);
        })
    });

    c.bench_function("version_calculation_with_mock", |b| {
        b.iter(|| {
            // This is a mock benchmark - in a real scenario you'd have a test repo
            let result = CalculationResult {
                version: Version::new(1, 0, 0),
                height: 0,
                is_from_tag: false,
                work_dir: ".".into(),
            };
            black_box(result);
        })
    });
}

criterion_group!(benches, benchmark_version_calculation);
criterion_main!(benches);
