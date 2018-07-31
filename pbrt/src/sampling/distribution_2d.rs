use num;
use crate::prelude::*;
use super::Distribution1d;

#[derive(Copy, Clone, Debug)]
pub struct Distribution2dSample {
    pub value: Point2f,
    pub pdf: Float,
}

pub struct Distribution2d {
    p_conditional_v: Vec<Distribution1d>,
    p_marginal: Distribution1d,
}

impl Distribution2d {
    pub fn new(f: &[Float], nu: usize, nv: usize) -> Self {
        let mut p_conditional_v = Vec::with_capacity(nv);

        for v in 0..nv {
            let start = v * nu;
            let end = start + nu;
            p_conditional_v.push(Distribution1d::new(&f[start..end]));
        }

        let mut marginal_func = Vec::with_capacity(nv);

        for v in p_conditional_v.iter().take(nv) {
            marginal_func.push(v.func_int);
        }


        Self {
            p_conditional_v,
            p_marginal: Distribution1d::new(&marginal_func),
        }
    }

    pub fn sample_continuous(&self, u: Point2f) -> Distribution2dSample {
        let d1 = self.p_marginal.sample_continuous(u[1]);
        let d0 = self.p_conditional_v[d1.offset].sample_continuous(u[0]);

        let pdf = d1.pdf * d0.pdf;
        let value = Point2f::new(d0.value, d1.value);

        Distribution2dSample { pdf, value }
    }

    pub fn pdf(&self, p: Point2f) -> Float {
        let iu = num::clamp(
            (p[0] * float(self.p_conditional_v[0].len())).raw() as usize,
            0,
            self.p_conditional_v[0].len() - 1);

        let iv = num::clamp(
            (p[1] * float(self.p_marginal.len())).raw() as usize,
            0,
            self.p_marginal.len() - 1);

        self.p_conditional_v[iv].function[iu] / self.p_marginal.func_int
    }
}
