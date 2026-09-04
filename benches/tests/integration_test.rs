use Benchmark_Sumcheck::{MultilinearPoly, run_sumcheck};

#[test]
fn test_sumcheck_v3() {
    let evals = vec![0, 1, 2, 3, 4, 5, 6, 7];
    let poly = MultilinearPoly {
        num_vars: 3,
        evaluations: evals.clone(),
    };
    let claimed_sum = evals.iter().sum();
    assert!(run_sumcheck(&poly, claimed_sum));
}

#[test]
fn test_sumcheck_v4() {
    let evals: Vec<u64> = (0..16).collect();
    let poly = MultilinearPoly {
        num_vars: 4,
        evaluations: evals.clone(),
    };
    let claimed_sum = evals.iter().sum();
    assert!(run_sumcheck(&poly, claimed_sum));
}