//! Virus alert simulation package.
//!
//! This crate allows to simulate and study the dynamics defined in the
//! [Virus Alert](https://ist.ac.at/en/education/ist-for-kids/virus-alert/) educational board game.
//!

pub use building::*;
pub use individual::*;
pub use population::*;
pub use board::*;
pub use recording::*;

mod individual;
mod building;
mod population; 
mod board;
mod recording;

pub mod errors {
    use thiserror::Error;

    #[derive(Error, Debug, PartialEq, Eq)]
    pub enum BuildingError {
        #[error("building is full")]
        Full,
        #[error("Sick individuals are not allowed in the buildings")]
        Sick,
    }
}

#[cfg(test)]
mod tests {
    /// Construct a deterministic RNG with the given seed
    pub fn rng(seed: u64) -> impl rand::RngCore {
        // For tests, we want a statistically good, fast, reproducible RNG.
        // PCG32 will do fine, and will be easy to embed if we ever need to.
        const INC: u64 = 11634580027462260723;
        rand_pcg::Pcg32::new(seed, INC)
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
