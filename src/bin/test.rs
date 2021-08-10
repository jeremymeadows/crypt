#[allow(unused)]
use std::env;

#[allow(unused_imports)]
use std::time::{Duration, Instant};
#[allow(unused_imports)]
use libcrypt::base64;
#[allow(unused_imports)]
use libcrypt::random::Generator;

#[allow(unused_variables)]
fn main() {
    let mut s = Generator::default();
    for i in 0..10 {
        println!("{}", s.next());
    }
}
