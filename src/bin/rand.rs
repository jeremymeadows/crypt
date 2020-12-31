use std::env;

use libcrypt::base64;
use libcrypt::random::Generator;

fn main() {
    let args: Vec<String> = env::args().collect();

    let s = "dax.png";
    let a = base64::encode(s.as_bytes().to_vec());
    let b = String::from_utf8(base64::decode(a.clone())).unwrap();

    println!("{}\n{}\n{}", s, a, b);

    // let mut gen = Generator::new();
    // if args.len() > 1 {
    //     gen.seed(u32::from_str_radix(&args[1], 10).unwrap());
    // }
    //
    // for _ in 0..500 {                       //   <--+
    //     println!("{}", gen.next() % 100);   //      | different ways of printing 500 random nums
    // }                                       //      |
    // for i in gen.get(500) {                 //   <--+
    //     println!("{}", i % 100);
    // }
}
