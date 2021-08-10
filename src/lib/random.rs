/* Psuedo-Random Number Generator
    Uses the Mercenne-Twister algorithm to generate PRNs
*/
use std::num::Wrapping;

static NN: usize = 312;
static MM: usize = 156;
const MATRIX_A: u64 = 0xB5026F5AA96619E9;
const UM: u64 = 0xFFFFFFFF80000000;
const LM: u64 = 0x7FFFFFFF;

pub struct Generator {
    state: Vec<u64>,
    next: usize,
}

impl Default for Generator {
    fn default() -> Self {
        let mut gen = Self {
            state: vec![0; NN],
            next: NN+1,
        };

        gen.seed(&gen as *const Self as u64);
        gen
    }
}

impl From<u64> for Generator {
    fn from(seed: u64) -> Self {
        let mut gen = Self {
            state: vec![0; NN],
            next: NN+1,
        };

        gen.seed(seed);
        gen
    }
}
impl From<&Vec<u8>> for Generator {
    fn from(key: &Vec<u8>) -> Self {
        let mut gen = Self {
            state: vec![0; NN],
            next: NN+1,
        };

        gen.seed_with_key(key);
        gen
    }
}

impl Generator {
    pub fn new() -> Self {
        let mut gen = Generator {
            state: vec![0; NN],
            next: NN+1,
        };

        gen.seed(5489);
        gen
    }

    pub fn seed(&mut self, seed: u64) {
        self.state[0] = seed;
        for i in 1..NN {
            self.state[i] = (Wrapping(6364136223846793005) * Wrapping(self.state[i-1] ^ (self.state[i-1] >> 62)) + Wrapping(i as u64)).0;
        }
        self.next = NN;
    }

    pub fn seed_with_key(&mut self, key: &Vec<u8>) {
        let mut x = 0;

        for i in key {
            let i = *i as u64;
            self.seed(i + x);
            x = self.state[i as usize % NN];
        }
    }

    // generates the next state based off the previous one
    fn twist(&mut self) {
        let mut x;
        let magic = [0, MATRIX_A];

        for i in 0..(NN-MM) {
            x = (self.state[i]&UM)|(self.state[i+1]&LM);
            self.state[i] = self.state[i+MM] ^ (x >> 1) ^ magic[(x & 1) as usize];
        }
        let mn = MM as isize - NN as isize;
        for i in (NN-MM)..(NN-1) {
            x = (self.state[i]&UM)|(self.state[i+1]&LM);
            self.state[i] = self.state[(i as isize+mn) as usize] ^ (x >> 1) ^ magic[(x & 1) as usize];
        }
        x = (self.state[NN-1]&UM)|(self.state[0]&LM);
        self.state[NN-1] = self.state[MM-1] ^ (x >> 1) ^ magic[(x & 1) as usize];
    }

    // regenerates the state if its out of new values, then returns the tempered next value 
    pub fn next(&mut self) -> u64 {
        if self.next >= NN {
            self.twist();
            self.next = 0;
        }

        let mut x = self.state[self.next];
        self.next += 1;

        x ^= (x >> 29) & 0x5555555555555555;
        x ^= (x << 17) & 0x71D67FFFEDA60000;
        x ^= (x << 37) & 0xFFF7EEE000000000;
        x ^=  x >> 43;

        x
    }

    // a real number with a range of 0..=1
    pub fn next_real(&mut self) -> f64 {
        (self.next() >> 11) as f64 * (1.0 / 0x1fffffffffffffu64 as f64)
    }

    // will lose bytes if n is not a multiple of 8.
    // eg. `get_bytes(4); get_bytes(4)` will yield different values from `get_bytes(8)`
    pub fn get_bytes(&mut self, n: usize) -> Vec<u8> {
        let mut v = Vec::<u8>::new();
        let mut x = self.next();

        loop {
            for _ in 0..8 {
                v.push(x as u8 & 0xff);
                x >>= 8;

                if v.len() == n {
                    return v;
                }
            }
            x = self.next();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::random::*;

    #[test]
    fn test_new() {
        let exp = [
            14514284786278117030,
             4620546740167642908,
            13109570281517897720,
            17462938647148434322,
              355488278567739596,
             7469126240319926998,
             4635995468481642529,
              418970542659199878,
             9604170989252516556,
             6358044926049913402,
        ];

        let mut gen = Generator::new();
        for i in 0..10 {
            assert_eq!(gen.next(), exp[i]);
        }
    }

    #[test]
    fn test_default() {
        let mut gen = Generator::default();
        for _ in 0..10 {
            gen.next();
        }
    }

    #[test]
    fn test_seed() {
        let exp = [
             3220586997909315655,
             3303451203970382242,
            11896436706893466529,
             8960318650144385956,
             4679212705770455613,
            15567843309247195414,
             6961994097256010468,
            10807484256991480663,
            11890420171946432686,
            15474158341220671739,
        ];

        let mut gen = Generator::from(0xff);
        for i in 0..10 {
            assert_eq!(gen.next(), exp[i]);
        }

        gen.seed(0xff);
        for i in 0..10 {
            assert_eq!(gen.next(), exp[i]);
        }
    }

    #[test]
    fn test_real() {
        let exp = [
            0.7868209548678021,
            0.2504803406880287,
            0.7106712289786555,
            0.9466678009609706,
            0.0192710581958138,
            0.4049021448161677,
            0.2513178179280376,
            0.0227124386279267,
            0.5206431525734918,
            0.3446703060791877,
        ];
        let err = 0.0000000000000001;

        let mut gen = Generator::new();
        for i in 0..10 {
            assert!(gen.next_real() - exp[i] < err);
        }
    }

    #[test]
    fn test_bytes() {
        let exp = [
            // 14514284786278117030 -> C96D_191C_F6F6_AEA6
            // 4620546740167642908  -> 401F_7AC7_8BC8_0F1C
            // 13109570281517897720 -> B5EE_8CB6_ABE4_57F8
            0xA6,
            0xAE,
            0xF6,
            0xF6,
            0x1C,
            0x19,
            0x6D,
            0xC9,

            0x1C,

            0xF8,
        ];

        let mut gen = Generator::new();
        let mut v = gen.get_bytes(9);
        v.append(&mut gen.get_bytes(1));
        for i in 0..10 {
            assert_eq!(v[i], exp[i]);
        }
    }
}
