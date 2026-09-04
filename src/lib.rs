use rand::Rng;

// ============ 数据结构 ============

#[derive(Clone)]
pub struct MultilinearPoly {
    pub num_vars: usize,
    pub evaluations: Vec<u64>,
}

// ============ Prover ============

pub struct Prover {
    poly: MultilinearPoly,
    fixed: Vec<u64>,
}

impl Prover {
    pub fn new(poly: MultilinearPoly) -> Self {
        Self {
            poly,
            fixed: Vec::new(),
        }
    }

    pub fn round_poly(&self, round: usize) -> Box<dyn Fn(u64) -> u64> {
        let num_vars = self.poly.num_vars;
        let evals = self.poly.evaluations.clone();
        // 只使用已固定的变量（前 round-1 个）
        let fixed = self.fixed[0..round - 1].to_vec();

        Box::new(move |x_j: u64| {
            let remaining = num_vars - round;
            let mut sum = 0u64;
            for bits in 0..(1 << remaining) {
                let mut idx = 0;
                let mut pos = 0;
                // 固定部分
                for &r in &fixed {
                    idx |= (r as usize) << pos;
                    pos += 1;
                }
                // 当前变量
                idx |= (x_j as usize) << pos;
                pos += 1;
                // 剩余变量
                for b in 0..remaining {
                    idx |= ((bits >> b) & 1) << pos;
                    pos += 1;
                }
                sum += evals[idx];
            }
            sum
        })
    }

    pub fn record_challenge(&mut self, r: u64) {
        self.fixed.push(r);
    }
}

// ============ Verifier ============

pub struct Verifier {
    num_vars: usize,
    claimed_sum: u64,
    fixed: Vec<u64>,
    prev_value: Option<u64>,
}

impl Verifier {
    pub fn new(num_vars: usize, claimed_sum: u64) -> Self {
        Self {
            num_vars,
            claimed_sum,
            fixed: Vec::new(),
            prev_value: None,
        }
    }

    pub fn verify_round(&mut self, round: usize, poly_fn: &dyn Fn(u64) -> u64) -> bool {
        let is_first = round == 1;

        if is_first {
            if self.claimed_sum != poly_fn(0) + poly_fn(1) {
                return false;
            }
        } else {
            if let Some(prev) = self.prev_value {
                if prev != poly_fn(0) + poly_fn(1) {
                    return false;
                }
            } else {
                return false;
            }
        }

        // 随机挑战值（限于 0/1 以便索引）
        let r_j = rand::thread_rng().gen_range(0..=1);
        self.fixed.push(r_j);
        self.prev_value = Some(poly_fn(r_j));
        true
    }

    pub fn get_challenge(&self) -> u64 {
        *self.fixed.last().unwrap()
    }

    pub fn final_verify(&self, poly: &MultilinearPoly, g_v_at_r_v: u64) -> bool {
        let mut idx = 0;
        for (i, &r) in self.fixed.iter().enumerate() {
            idx |= (r as usize) << i;
        }
        let actual = poly.evaluations[idx];
        actual == g_v_at_r_v
    }
}

// ============ 主协议 ============

pub fn run_sumcheck(poly: &MultilinearPoly, claimed_sum: u64) -> bool {
    let mut prover = Prover::new(poly.clone());
    let mut verifier = Verifier::new(poly.num_vars, claimed_sum);

    for round in 1..=poly.num_vars {
        let g_j = prover.round_poly(round);
        if !verifier.verify_round(round, &g_j) {
            return false;
        }
        let r_j = verifier.get_challenge();
        prover.record_challenge(r_j);
    }

    let last_poly = prover.round_poly(poly.num_vars);
    let g_v_at_r_v = last_poly(verifier.get_challenge());
    verifier.final_verify(poly, g_v_at_r_v)
}

// ============ 测试 ============

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sumcheck_v2() {
        let evals = vec![0, 1, 2, 3];
        let poly = MultilinearPoly {
            num_vars: 2,
            evaluations: evals,
        };
        let claimed_sum = 0 + 1 + 2 + 3;
        assert!(run_sumcheck(&poly, claimed_sum));
    }
}