use crate::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct Distribution1dSample {
    pub value: Float,
    pub pdf: Float,
    pub offset: usize,
}

pub struct Distribution1d {
    pub(super) function: Vec<Float>,
    cdf: Vec<Float>,
    pub(super) func_int: Float,
}

impl Distribution1d {
    pub fn new(f: &[Float]) -> Self {
        // compute the integral of step functions at xi
        let mut cdf = vec![float(0.0); f.len() + 1];

        for i in 1..=f.len() {
            cdf[i] = cdf[i - 1] + f[i - 1] / float(f.len());
        }

        // transform step function integral into cdf
        let func_int = *cdf.last().unwrap();
        if func_int == 0.0 {
            for (i, cdf) in cdf.iter_mut().enumerate().take(f.len() + 1).skip(1) {
                *cdf = float(i) / float(f.len());
            }
        } else {
            for cdf in cdf.iter_mut().take(f.len() + 1).skip(1) {
                *cdf /= func_int;
            }
        }

        Self {
            function: f.to_vec(),
            cdf,
            func_int,
        }
    }

    pub fn len(&self) -> usize {
        self.function.len()
    }

    pub fn is_empty(&self) -> bool {
        self.function.is_empty()
    }

    pub fn sample_continuous(&self, u: Float) -> Distribution1dSample {
        // find surrounding CDF segments and offset
        let offset = find_interval(self.cdf.len(), |i| self.cdf[i] <= u);

        // conpute offset along CDF segment
        let mut du = u - self.cdf[offset];
        if (self.cdf[offset + 1] - self.cdf[offset]) > float(0.0) {
            du /= self.cdf[offset + 1] - self.cdf[offset];
        }

        // compute PDF for sampled offset
        let pdf = self.function[offset] / self.func_int;

        // return x ∈ [0, 1) corresponding to sample
        let value = (float(offset) + du) / float(self.len());

        Distribution1dSample { offset, pdf, value }
    }

    pub fn sample_discrete(&self, u: Float) -> Distribution1dSample {
        let offset = find_interval(self.cdf.len(), |i| self.cdf[i] <= u);

        let pdf = self.function[offset] / (self.func_int * float(self.len()));

        let value = (u - self.cdf[offset]) / (self.cdf[offset + 1] - self.cdf[offset]);

        Distribution1dSample { offset, pdf, value }
    }

    pub fn discrete_pdf(&self, index: usize) -> Float {
        self.function[index] / (self.func_int * float(self.len()))
    }
}
