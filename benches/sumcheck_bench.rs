use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use Benchmark_Sumcheck::{MultilinearPoly, run_sumcheck};
use rand::Rng;

fn bench_sumcheck(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sumcheck Protocol");

    for v in [2, 4, 6, 8].iter() {
        // 生成随机多项式求值表
        let evals: Vec<u64> = (0..(1 << v))
            .map(|_| rand::thread_rng().gen_range(0..100))
            .collect();
        let poly = MultilinearPoly {
            num_vars: *v,
            evaluations: evals.clone(),
        };
        let claimed_sum = evals.iter().sum();

        group.bench_with_input(BenchmarkId::from_parameter(v), &poly, |b, poly| {
            b.iter(|| {
                let _ = run_sumcheck(poly, claimed_sum);
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_sumcheck);
criterion_main!(benches);