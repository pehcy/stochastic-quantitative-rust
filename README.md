<b>This project is still under active research and development. ⚗️</b>

This project is aiming for:
- **API**: Setup an API gateway to retrieve end of day data, and resampling the time serires data into 
shorter period.

- **Accuracy**: The test scores for each model should be > 90% under cross validation.

- **Automated Trading Strategy**: Design a machine learning 
approach to decide trading strategy.

- **Self Learning Purpose**: I tend to 
learn quantitative finance and Rust programming at the same time. Feel free 
to give me any advice to improve the code 
qualities.

## Why using Rust?
Although Python is widely used by quantitative analysts 
and statistician. While comparing Rust with Python, Rust provide the best performance and faster compilation speed. Also, it's more C like. The code can be written more idiomatic.

## Reading Notes
A minimal notes for quantitative finance 
can be found [here](https://raw.githubusercontent.com/pehcy/stochastic-quantitative-rust/main/notes.pdf).

## TODO List
- [ ] Implement Square-root diffusion, 
Geometric Brownian motion, and their variation of 
euler discretization.
- [ ] Give a sample jupyterNotebook for 
visualization
- [ ] Test their test scores with sample data.
- [ ] Eod data using yahoo finance API