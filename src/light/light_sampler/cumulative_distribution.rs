use fastrand::Rng;

use crate::light::SampleLightResult;

pub struct CDF<'a> {
    weights: &'a Vec<(SampleLightResult, f64)>,
}

impl<'a> CDF<'a> {
    pub fn new(weights: &'a Vec<(SampleLightResult, f64)>) -> Self {
        Self { weights }
    }

    pub fn sample(&self, rng: &mut Rng) -> Option<(usize, &(SampleLightResult, f64))> {
        let mut cdf: Vec<f64> = Vec::with_capacity(self.weights.len());
        let mut cumulative_sum = 0.0;
        for (_, weight) in self.weights.iter() {
            cumulative_sum += weight;
            cdf.push(cumulative_sum);
        }

        // Generate a random number in the range [0, 1)
        let random_value = rng.f64();

        // Find the index using binary search
        let index = cdf.iter().position(|&cp| random_value < cp)?;

        Some((index, &self.weights[index]))
    }
}
