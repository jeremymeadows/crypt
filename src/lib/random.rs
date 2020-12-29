/* Psuedo-Random Number Generator
    Simulates the Mercenne-Twister algorithm to generate PRNs
*/
pub struct Generator {
    seed: u32,
    state: Vec<i32>,
    next: usize,
}

#[allow(overflowing_literals)]
impl Generator {
    // const X: u32 = 0x7fffffff;
    // const Y: u32 = 0x3fff;
    const S: usize = 624;

    pub fn new() -> Generator {
        let mut gen = Generator {
            seed: 0xff,
            state: vec![0; Generator::S],
            next: 0,
        };

        gen.state[0] = gen.seed as i32;
        for i in 1..Generator::S {
            gen.state[i] = (1812433253u64 * (gen.state[i-1] ^ (gen.state[i-1] >> 30)) as u64 + i as u64) as i32;
        }

        gen.twist();
        gen
    }

    pub fn seed(&mut self, seed: u32) {
        self.seed = seed;
        self.state[0] = seed as i32;

        for i in 1..Generator::S {
            self.state[i] = (1812433253u64 * (self.state[i-1] ^ (self.state[i-1] >> 30)) as u64 + i as u64) as i32;
        }

        self.twist();
    }

    pub fn next(&mut self) -> i32 {
        if self.next >= Generator::S {
            self.twist();
        }

        let mut x = self.state[self.next];
        x ^= x >> 11;
        x ^= (x << 7) & 0x9d2c5680;
        x ^= (x << 15) & 0xefc60000;
        x ^= x >> 18;
        self.next += 1;

        x
    }

    pub fn get(&mut self, n: usize) -> Vec<i32> {
        let mut v = Vec::<i32>::with_capacity(n);
        for _ in 0..n {
            v.push(self.next());
        }

        v
    }

    fn twist(&mut self) {
        let m = 397;
        let a = Generator::S - m;

        let mut i = 0;
        while i < a {
            let bits = (self.state[i] & 0x80000000) | (self.state[i+1] & 0x7fffffff);
            self.state[i] = self.state[i+m] ^ (bits >> 1) ^ ((bits & 1) * 0x9908b0df);

            i += 1;
        }
        while i < Generator::S - 1 {
            let bits = (self.state[i] & 0x80000000) | (self.state[i+1] & 0x7fffffff);
            self.state[i] = self.state[i-a] ^ (bits >> 1) ^ ((bits & 1) * 0x9908b0df);

            i += 1;
        }

        let bits = (self.state[i] & 0x80000000) | (self.state[0] & 0x7fffffff);
        self.state[i] = self.state[m-1] ^ (bits >> 1) ^ ((bits & 1) * 0x9908b0df);

        self.next = 0;
    }
}
