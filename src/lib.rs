#![warn(
    clippy::all,
    clippy::complexity,
    clippy::correctness,
    clippy::deprecated,
    clippy::nursery,
    clippy::pedantic,
    clippy::perf,
    clippy::style,
    clippy::suspicious,
)]

pub mod cartesian_iterator;
pub mod dependency;
pub mod device;
pub mod service;
pub mod smart_home;
pub mod subsystem;
pub mod update;
