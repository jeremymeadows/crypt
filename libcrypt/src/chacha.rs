//! Implementation for the ChaCha20 symmetric stream cipher.

use std::cmp;

const CONSTANTS: [u32; 4] = [0x61707865, 0x3320646E, 0x79622D32, 0x6B206574];

/// A ChaCha symmetric stream cipher.
pub struct ChaCha {
    state: [u32; 16],
    key: [u32; 8],
    counter: u32,
    nonce: [u32; 3],
}

impl ChaCha {
    /// Creates a new ChaCha cipher from the given key.
    pub fn new(key: &Vec<u8>) -> Self {
        Self::from_state(key, 1, [0x00000000, 0x00000000, 0x00000000])
    }

    /// Creates a new ChaCha cipher from the given key, also setting the current state of the
    /// counter and the values of the nonce.
    pub fn from_state(key: &Vec<u8>, counter: u32, nonce: [u32; 3]) -> Self {
        let key = ChaCha::expand_key(&mut key.clone());

        let mut cc = Self {
            state: [0; 16],
            key: key,
            counter: counter,
            nonce: nonce,
        };

        cc.state = cc.calc_state();
        cc
    }

    fn expand_key(key: &mut Vec<u8>) -> [u32; 8] {
        let mut a = [0u32; 8];
        while key.len() < 32 {
            key.append(&mut key.clone());
        }
        key.resize(32, 0);

        for i in 0..a.len() {
            for j in (0..4).rev() {
                a[i] |= (key[(i * 4) + j] as u32) << (j * 8);
            }
        }
        a
    }

    fn calc_state(&self) -> [u32; 16] {
        [
            CONSTANTS[0],  CONSTANTS[1],  CONSTANTS[2],  CONSTANTS[3],
             self.key[0],   self.key[1],   self.key[2],   self.key[3],
             self.key[4],   self.key[5],   self.key[6],   self.key[7],
            self.counter, self.nonce[0], self.nonce[1], self.nonce[2],
        ]
    }

    fn quarter_round(&mut self, a: usize, b: usize, c: usize, d: usize) {
        self.state[a] = self.state[a].wrapping_add(self.state[b]);
        self.state[d] = (self.state[d] ^ self.state[a]).rotate_left(16);

        self.state[c] = self.state[c].wrapping_add(self.state[d]);
        self.state[b] = (self.state[b] ^ self.state[c]).rotate_left(12);

        self.state[a] = self.state[a].wrapping_add(self.state[b]);
        self.state[d] = (self.state[d] ^ self.state[a]).rotate_left(8);

        self.state[c] = self.state[c].wrapping_add(self.state[d]);
        self.state[b] = (self.state[b] ^ self.state[c]).rotate_left(7);
    }

    fn block_round(&mut self) {
        let old_state = self.state.clone();

        for _ in 0..10 {
            self.quarter_round(0, 4, 8, 12);
            self.quarter_round(1, 5, 9, 13);
            self.quarter_round(2, 6, 10, 14);
            self.quarter_round(3, 7, 11, 15);

            self.quarter_round(0, 5, 10, 15);
            self.quarter_round(1, 6, 11, 12);
            self.quarter_round(2, 7, 8, 13);
            self.quarter_round(3, 4, 9, 14);
        }

        for i in 0..self.state.len() {
            self.state[i] = self.state[i].wrapping_add(old_state[i]);
        }
    }

    fn serialize(&self) -> Vec<u8> {
        let mut v = Vec::new();

        for i in 0..16 {
            for j in 0..4 {
                v.push((self.state[i] >> (j * 8)) as u8);
            }
        }

        v
    }

    /// Encrypts the given plaintext, returning the ciphertext.
    pub fn encrypt(&mut self, plaintext: &Vec<u8>) -> Vec<u8> {
        let mut ciphertext = Vec::<u8>::new();
        let mut ndx = 0;

        while ndx < plaintext.len() {
            self.state = self.calc_state();
            self.counter += 1;
            self.block_round();

            let key_stream = self.serialize();
            let len = cmp::min(plaintext.len() - ndx, 64);
            for i in 0..len {
                ciphertext.push(key_stream[i] ^ plaintext[ndx + i]);
            }
            ndx += len;
        }

        ciphertext
    }

    /// Decrypts the given ciphertext, returning the plaintext.
    pub fn decrypt(&mut self, plaintext: &Vec<u8>) -> Vec<u8> {
        self.encrypt(plaintext)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_quarter_round() {
        let mut cc = ChaCha::new(&vec![0]);
        let exp = [
            0x879531e0, 0xc5ecf37d, 0xbdb886dc, 0xc9a62f8a,
            0x44c20ef3, 0x3390af7f, 0xd9fc690b, 0xcfacafd2,
            0xe46bea80, 0xb00a5631, 0x974c541a, 0x359e9963,
            0x5c971061, 0xccc07c79, 0x2098d9d6, 0x91dbd320,
        ];

        cc.state = [
            0x879531e0, 0xc5ecf37d, 0x516461b1, 0xc9a62f8a,
            0x44c20ef3, 0x3390af7f, 0xd9fc690b, 0x2a5f714c,
            0x53372767, 0xb00a5631, 0x974c541a, 0x359e9963,
            0x5c971061, 0x3d631689, 0x2098d9d6, 0x91dbd320,
        ];
        cc.quarter_round(2, 7, 8, 13);

        assert_eq!(cc.state, exp);
    }

    #[test]
    fn test_block_round() {
        let mut cc = ChaCha::from_state(
            &vec![
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
                0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
                0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
            ],
            0x00000001,
            [0x09000000, 0x4a000000, 0x00000000],
        );
        let exp = [
            0xe4e7f110, 0x15593bd1, 0x1fdd0f50, 0xc47120a3,
            0xc7f4d1c7, 0x0368c033, 0x9aaa2204, 0x4e6cd4c3,
            0x466482d2, 0x09aa9f07, 0x05d7c214, 0xa2028bd9,
            0xd19c12b5, 0xb94e16de, 0xe883d0cb, 0x4e3c50a2,
        ];

        cc.block_round();

        assert_eq!(cc.state, exp);
    }

    #[test]
    fn test_serialize() {
        let mut cc = ChaCha::from_state(
            &vec![
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
                0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
                0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
            ],
            0x00000001,
            [0x09000000, 0x4a000000, 0x00000000],
        );
        let exp = [
            0x10, 0xf1, 0xe7, 0xe4, 0xd1, 0x3b, 0x59, 0x15,
            0x50, 0x0f, 0xdd, 0x1f, 0xa3, 0x20, 0x71, 0xc4,
            0xc7, 0xd1, 0xf4, 0xc7, 0x33, 0xc0, 0x68, 0x03,
            0x04, 0x22, 0xaa, 0x9a, 0xc3, 0xd4, 0x6c, 0x4e,
            0xd2, 0x82, 0x64, 0x46, 0x07, 0x9f, 0xaa, 0x09,
            0x14, 0xc2, 0xd7, 0x05, 0xd9, 0x8b, 0x02, 0xa2,
            0xb5, 0x12, 0x9c, 0xd1, 0xde, 0x16, 0x4e, 0xb9,
            0xcb, 0xd0, 0x83, 0xe8, 0xa2, 0x50, 0x3c, 0x4e,
        ];

        cc.block_round();

        assert_eq!(cc.serialize(), exp);
    }

    #[test]
    fn test_encrypt() {
        let mut cc = ChaCha::from_state(
            &vec![
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
                0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
                0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
            ],
            0x00000001,
            [0x00000000, 0x4a000000, 0x00000000],
        );

        let plaintext = "Ladies and Gentlemen of the class of '99: If I could offer you only one tip for the future, sunscreen would be it.";
        let ciphertext = [
            0x6e, 0x2e, 0x35, 0x9a, 0x25, 0x68, 0xf9, 0x80,
            0x41, 0xba, 0x07, 0x28, 0xdd, 0x0d, 0x69, 0x81,
            0xe9, 0x7e, 0x7a, 0xec, 0x1d, 0x43, 0x60, 0xc2,
            0x0a, 0x27, 0xaf, 0xcc, 0xfd, 0x9f, 0xae, 0x0b,
            0xf9, 0x1b, 0x65, 0xc5, 0x52, 0x47, 0x33, 0xab,
            0x8f, 0x59, 0x3d, 0xab, 0xcd, 0x62, 0xb3, 0x57,
            0x16, 0x39, 0xd6, 0x24, 0xe6, 0x51, 0x52, 0xab,
            0x8f, 0x53, 0x0c, 0x35, 0x9f, 0x08, 0x61, 0xd8,
            0x07, 0xca, 0x0d, 0xbf, 0x50, 0x0d, 0x6a, 0x61,
            0x56, 0xa3, 0x8e, 0x08, 0x8a, 0x22, 0xb6, 0x5e,
            0x52, 0xbc, 0x51, 0x4d, 0x16, 0xcc, 0xf8, 0x06,
            0x81, 0x8c, 0xe9, 0x1a, 0xb7, 0x79, 0x37, 0x36,
            0x5a, 0xf9, 0x0b, 0xbf, 0x74, 0xa3, 0x5b, 0xe6,
            0xb4, 0x0b, 0x8e, 0xed, 0xf2, 0x78, 0x5e, 0x42,
            0x87, 0x4d,
        ];

        assert_eq!(cc.encrypt(&Vec::from(plaintext)), ciphertext);
    }

    #[test]
    fn test_decrypt() {
        let mut cc = ChaCha::from_state(
            &vec![
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
                0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
                0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
            ],
            0x00000001,
            [0x00000000, 0x4a000000, 0x00000000],
        );

        let plaintext = "Ladies and Gentlemen of the class of '99: If I could offer you only one tip for the future, sunscreen would be it.";
        let ciphertext = [
            0x6e, 0x2e, 0x35, 0x9a, 0x25, 0x68, 0xf9, 0x80,
            0x41, 0xba, 0x07, 0x28, 0xdd, 0x0d, 0x69, 0x81,
            0xe9, 0x7e, 0x7a, 0xec, 0x1d, 0x43, 0x60, 0xc2,
            0x0a, 0x27, 0xaf, 0xcc, 0xfd, 0x9f, 0xae, 0x0b,
            0xf9, 0x1b, 0x65, 0xc5, 0x52, 0x47, 0x33, 0xab,
            0x8f, 0x59, 0x3d, 0xab, 0xcd, 0x62, 0xb3, 0x57,
            0x16, 0x39, 0xd6, 0x24, 0xe6, 0x51, 0x52, 0xab,
            0x8f, 0x53, 0x0c, 0x35, 0x9f, 0x08, 0x61, 0xd8,
            0x07, 0xca, 0x0d, 0xbf, 0x50, 0x0d, 0x6a, 0x61,
            0x56, 0xa3, 0x8e, 0x08, 0x8a, 0x22, 0xb6, 0x5e,
            0x52, 0xbc, 0x51, 0x4d, 0x16, 0xcc, 0xf8, 0x06,
            0x81, 0x8c, 0xe9, 0x1a, 0xb7, 0x79, 0x37, 0x36,
            0x5a, 0xf9, 0x0b, 0xbf, 0x74, 0xa3, 0x5b, 0xe6,
            0xb4, 0x0b, 0x8e, 0xed, 0xf2, 0x78, 0x5e, 0x42,
            0x87, 0x4d,
        ];

        assert_eq!(cc.decrypt(&Vec::from(ciphertext)), Vec::from(plaintext));
    }

    #[test]
    fn test_repeated_encrypt() {
        let mut cc = ChaCha::new(&Vec::from("super_secret_key"));
        let plain = [
            "foo",
            "Hello, world!",
            "The quick brown fox jumped over the lazy dog",
        ];
        let mut cipher = Vec::new();

        for s in plain {
            cipher.push(cc.encrypt(&Vec::from(s)));
        }

        cc = ChaCha::new(&Vec::from("super_secret_key"));
        for i in 0..(plain.len()) {
            assert_eq!(cc.decrypt(&cipher[i]), Vec::from(plain[i]));
        }
    }
}
