mod shared_seed;
pub use shared_seed::SharedSeedMTP;

use crate::mul_triple::MulTriple;

pub trait MTProvider {
    fn get_triple(&mut self) -> MulTriple;
}

pub struct TrivialMTP;
impl MTProvider for TrivialMTP {
    fn get_triple(&mut self) -> MulTriple {
        MulTriple{ a: false, b: false, c: false }
    }
}