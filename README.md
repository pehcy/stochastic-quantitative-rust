<b>This project is about how to implement 
stochastic algorithms via Rust for European options ⚗️</b>

## Why using Rust?
Although Python is widely used by quantitative analysts 
and statistician. While comparing Rust with Python, Rust provide the best performance and faster compilation speed. Also, it's more C like. The code can be written more idiomatic.

## Test time

| Simulation | Compilation time |
| ------ | ------ |
| Monte-Carlo | 0.258s |
| Static Geometric Brownian motion | 0.247s |
| Dynamic Geometric Brownian motion | `N/A` |
| Square-root diffusion | 0.385s |

## TODO List
- [x] Implement Square-root diffusion, 
Geometric Brownian motion, and their variation of 
euler discretization.
- [ ] Test all the functions.
- [x] README