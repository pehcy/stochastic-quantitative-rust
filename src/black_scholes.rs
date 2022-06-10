// use crate::linearalg::*;

use rand::distributions::Uniform;
use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct BlackScholesModel {
    pub c:  f64,
    pub k:  f64, 
    pub s0: f64,
    pub r:  f64,
    pub sigma: f64,
    pub t:  f64
}

pub trait GeomBM {
    fn static_geobm(&self, n: usize) -> Vec<f64>;
    fn dynamic_geobm(&self, m: usize) -> Vec<Vec<f64>>;
}

pub const _SAMPLE_SIZE : usize = 10000;

impl GeomBM for BlackScholesModel {

    /// static geometric brownian motion.
    /// 
    /// Assume the Black-scholes-Merton model is in its 
    /// dynamic form, with constant volatility and risk-free interest rate
    /// The SDE can be formulate as follows:
    /// 
    /// ```formula
    /// S_t = S_{t - Δ_t} * exp((r - 1/2 * σ^2) * Δ_t + σ * sqrt{Δ_t} z_t)
    /// ```
    ///
    fn static_geobm(&self, n: usize) -> Vec<f64> {
        let rng = rand::thread_rng();
        let range = Uniform::new(-1.0f64, 1.0f64);
        let stds: Vec<f64> = rng.sample_iter(&range)
                            .take(n)
                            .collect();
        let mut result: Vec<f64> = vec![0.0f64; n];
        
        for i in 0..n {
            let st1 = self.s0 * ((self.r - 0.5 * f64::powi(self.sigma, 2)) + 
                        self.sigma * f64::sqrt(self.t) * &stds[i]).exp();
            result[i] = st1;
        }
        result
    }

    /// dynamic geometric brownian motion.
    /// 
    /// The Black-Scholes SDE that has been discretized exactly 
    /// by an Euler scheme, with Δ_t being the fixed discretization interval
    /// and z_t being an array of random sampling from standard normal.
    /// 
    fn dynamic_geobm(&self, m: usize) -> Vec<Vec<f64>> {
        let mut levels = vec![vec![0f64; _SAMPLE_SIZE]; m];
        let dt = self.t / m as f64;

        for i in 0..m {
            let rng = rand::thread_rng();
            let range = Uniform::new(-1.0f64, 1.0f64);
            let stds: Vec<f64> = rng.sample_iter(&range)
                            .take(_SAMPLE_SIZE)
                            .collect();

            for j in 0.._SAMPLE_SIZE {
                if i == 0 {
                    levels[i][j] = self.s0;
                }
                else {
                    levels[i][j] = &levels[i-1][j] 
                        * ((self.r - 0.5_f64 * f64::powi(self.sigma, 2)) * dt
                        + self.sigma * dt.sqrt() * &stds[j]).exp(); 
                }
            }
        }

        levels
    }
}