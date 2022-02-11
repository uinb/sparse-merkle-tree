use crate::{traits::Hasher, H256};
use blake2::{Blake2b, Digest};
use generic_array::typenum::U32;

pub struct Blake2bHasher(Blake2b<U32>);

impl Default for Blake2bHasher {
    fn default() -> Self {
        Blake2bHasher(Blake2b::new())
    }
}

impl Hasher for Blake2bHasher {
    fn write_h256(&mut self, h: &H256) {
        self.0.update(h.as_slice());
    }

    fn write_byte(&mut self, b: u8) {
        self.0.update(&[b][..]);
    }

    fn finish(self) -> H256 {
        self.0.finalize().into()
    }
}
