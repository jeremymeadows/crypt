//! This library for the Crypt program contains my implementations for the ChaCha20 stream
//! cipher, as well as for the Mersenne Twister psuedo-random number generator.
//!
//! They both pass the test vectors that were provided in their respective papers.
//!
//! It also includes some custom trait implementstions for `Stdin` to make reading console
//! input slightly simpler.

pub mod chacha;
pub mod chacha_rng;
pub mod mersenne_twister;

pub mod stdin_extras;
