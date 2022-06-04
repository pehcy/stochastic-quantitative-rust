use std::io;
use statrs::distribution::{Normal, ContinuousCDF};
use rand::distributions::{Distribution, Standard, Uniform};
use rand::Rng;

fn main() {
    println!("The Pi approximation is: {:?}", mcs_pi_simulation(5000));
}

pub struct EuropeanOption {
    risk_free:      f64,
    k:              f64,
    s0:             f64,
    t:              f64,
    sigma:          f64,
    cost_of_carry:  f64,
    opt_type:       OptionType
}

enum OptionType {
    Call,
    Put
}

impl EuropeanOption {
    pub fn call_price(&self) -> f64 {
        let weighted_volatility : f64 = self.sigma * (self.t).sqrt();
        let d1 = ((self.s0 / self.k).ln() 
            + (self.cost_of_carry + 0.5_f64 * f64::powi(self.sigma, 2))) / weighted_volatility;
        let d2 = d1 - weighted_volatility;
        
        // initialize the standard gaussian districution
        let n = Normal::new(0.0, 1.0).unwrap();
        let c1: f64 = self.s0 * (self.t * (self.cost_of_carry - self.risk_free)).exp() * n.cdf(d1 as f64);
        let c2: f64 = self.k * (-self.risk_free * self.t).exp() * n.cdf(d2 as f64);
        
        c1 - c2
    }

    pub fn put_price(&self) -> f64 {
        let weighted_volatility : f64 = self.sigma * (self.t).sqrt();
        let d1 = ((self.s0 / self.k).ln() 
            + (self.cost_of_carry + 0.5_f64 * f64::powi(self.sigma, 2))) / weighted_volatility;
        let d2 = d1 - weighted_volatility;

        // initialize the standard gaussian distribution
        let n = Normal::new(0.0, 1.0).unwrap();
        let p1: f64 = self.k * (-self.risk_free * self.t).exp() * n.cdf(-d2 as f64);
        let p2: f64 = self.s0 * (self.t * (self.cost_of_carry - self.risk_free)).exp() * n.cdf(-d1 as f64);

        p1 - p2
    }

    pub fn toggle(&mut self) {
        match self.opt_type {
            OptionType::Call => self.opt_type = OptionType::Put,
            OptionType::Put => self.opt_type = OptionType::Call
        }
    }
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

const _UNIT_CIRCLE_RADIUS : f64 = 1.0f64;

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let step = Uniform::new(-1.0f64, 1.0f64);
        let v: Vec<f64> = rng.sample_iter(&step)
                            .take(2)
                            .collect();
        Point {
            x: v[0],
            y: v[1],
        }
    }
}

// generate 5000 point on R^2 plane 
fn mcs_pi_simulation(n: usize) -> f64 {
    let mut rng = rand::thread_rng();
    let mut count: i32 = 0;
    let rand_points: Vec<Point> = (0..n).map(|_| rng.gen()).collect();
    
    for p in &rand_points {
        if f64::powi(p.x, 2) + f64::powi(p.y, 2) <= _UNIT_CIRCLE_RADIUS {
            count += 1;
        }
    }

    (4.0_f64 * count as f64) / n as f64
}