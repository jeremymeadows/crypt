//! Implementation for the ChaCha20 symmetric stream cipher.

use crate::chacha::{ChaCha, Key};

static SEED: () = ();

pub struct ChaChaRng(ChaCha);

pub trait RngOutput {
    type Output;

    fn gen(rng: &mut ChaChaRng) -> Self::Output;
}

macro_rules! impl_rng_output {
    ($t:ty) => {
        impl RngOutput for $t {
            type Output = $t;

            fn gen(rng: &mut ChaChaRng) -> Self::Output {
                let bytes = (Self::BITS / 8) as usize;
                let v = rng.0.encrypt(&vec![0; bytes]);
                let mut n = 0;

                for i in 0..bytes {
                    n |= (v[i] as Self::Output) << (Self::BITS - ((i as u32 + 1) * 8));
                }

                n
            }
        }
    };
}

impl_rng_output!(u8);
impl_rng_output!(u16);
impl_rng_output!(u32);
impl_rng_output!(u64);
impl_rng_output!(u128);

impl ChaChaRng {
    pub fn new() -> ChaChaRng {
        ChaChaRng(ChaCha::new([0_u8; 32].as_slice()))
    }

    pub fn from<T: Key>(seed: T) -> ChaChaRng {
        ChaChaRng(ChaCha::new(seed).with_counter(0))
    }

    pub fn from_entropy() -> ChaChaRng {
        ChaChaRng(ChaCha::new(
            (&SEED as *const () as u64).to_ne_bytes().as_slice(),
        ))
    }

    pub fn next<T: RngOutput>(&mut self) -> T::Output {
        T::gen(self)
    }

    pub fn get_bytes(&mut self, n: usize) -> Vec<u8> {
        let mut v = Vec::with_capacity(n);
        for _ in 0..n {
            v.push(self.next::<u8>());
        }
        v
    }

    pub fn fill_bytes(&mut self, buf: &mut [u8]) {
        for i in 0..buf.len() {
            buf[i] = self.next::<u8>();
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn chi_sqrd() {
        let mut rng = ChaChaRng::from_entropy();
        let n = 100000;
        let mut buckets = [vec![0; 10], vec![0; 100]];

        for _ in 0..n {
            let r = rng.next::<u32>();

            for b in buckets.iter_mut() {
                let l = b.len() as u32;
                b[(r % l) as usize] += 1;
            }
        }

        for b in buckets.iter() {
            let d = (n / b.len()) as f32;

            for i in b.iter() {
                assert!(*i as f32 > d * 0.9 && (*i as f32) < d * 1.1);
            }
        }
    }
}
