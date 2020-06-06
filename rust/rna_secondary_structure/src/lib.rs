//! A crate for analysing RNA (and DNA) secondary structures in Rust.

#![warn(missing_docs)]

pub mod secondary_structure;
pub mod io;
pub mod distance_metrics;
pub mod read_rfam;
pub mod combinatorics;