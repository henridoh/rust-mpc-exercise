use rand::{Rng, SeedableRng};
use crate::mul_triple::MulTriple;
use crate::mul_triple::provider::MTProvider;

pub struct SharedSeedMTP<T: SeedableRng + Rng> {
    rng: T,
}

impl<T: SeedableRng + Rng> SharedSeedMTP<T> {
    pub fn new(seed: T::Seed) -> Self {
        SharedSeedMTP {
            rng: T::from_seed(seed)
        }
    }
}

impl<T: SeedableRng + Rng> MTProvider for SharedSeedMTP<T> {
    fn get_triple(&mut self) -> MulTriple {
        let a = self.rng.gen();
        let b = self.rng.gen();
        let c = self.rng.gen();

        MulTriple { a, b, c }
    }
}