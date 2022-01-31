# Smart Holmes

## About
This is a library *smart_holmes* and a simulation.
We created *smart_holmes* to autogenerate smart home configurations based on several properties, such as the number of devices, dependencies etc.
Using only these configurations a nearly completely random smart home is generated with several available updates for its devices.
In the simulation (code is at new_main.rs) we generate smart home configurations with increasing numbers of e.g. devices and measure for 1000 smart homes that were generated through a specific configuration, what the update score is for each of the 3 implemented strategies.

## Usage

1) Set the variable that you want to iterate over in new_main.rs -> generate_smart_home

2) Set the upper and lower bounds and the step for each simulation ( for input in (0..=100).step_by(5) { )

3)
```
cargo run --release
```

4) Copy the data to the plots.tex 

![Flamegraph](https://github.com/TheRustyStorm/smart-holmes/blob/main/flamegraph.svg)
