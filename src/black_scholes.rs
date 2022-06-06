mod black_scholes {
    // use crate::linearalg::*;

    use rand::distributions::{Distribution, Standard, Uniform};
    use rand::Rng;

    #[derive(Debug)]
    pub struct BlackScholesModel {
        // c:  f64,
        // k:  f64, 
        s0: f64,
        r:  f64,
        sigma: f64,
        t:  f64
    }

    pub trait GeomBM {
        fn get_sim_index_lvls(&self, n: usize) -> Vec<f64>;
        fn eudisc_index_lvls(&self) -> Vec<f64>;
    }

    impl GeomBM for BlackScholesModel {

        /// geometric brownian motion.
        /// 
        /// Assume the Black-scholes-Merton model is in its 
        /// dynamic form, with constant volatility and risk-free interest rate
        /// The SDE can be formulate as follows:
        /// 
        /// ```formula
        /// S_t = S_{t - Δ_t} * exp((r - 1/2 * σ^2) * Δ_t + σ * sqrt{Δ_t} z_t)
        /// ```
        /// 
        fn get_sim_index_lvls(&self, n: usize) -> Vec<f64> {
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

        /// GBM with euler discretization.
        /// 
        fn eudisc_index_lvls(&self) -> Vec<f64> {
            vec![0.5]
        }
    }
}