use std::env;

use libcrypt::random::Generator;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut gen = Generator::new();
    if args.len() > 1 {
        gen.seed(u32::from_str_radix(&args[1], 10).unwrap());
    }

    for _ in 0..500 {                       //   <--+
        println!("{}", gen.next() % 100);   //      | different ways of printing 500 random nums
    }                                       //      |
    for i in gen.get(500) {                 //   <--+
        println!("{}", i % 100);
    }
}
