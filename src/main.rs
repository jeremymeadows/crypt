use std::{io, env, fs, process};

enum Mode { ENCRYPT, DECRYPT }
// enum Error { FileNotFound, FilePermissions }

use Mode::{ENCRYPT, DECRYPT};

const AES_SIZE: usize = 16;

fn argparse(args: &Vec<String>) -> Option<Mode> {
    let help = format!("\nUsage: crypt COMMAND INPUT OUTPUT\n\n\
        Crypt uses key-based AES to encrypt/decrypt your files.\n\
        \n\
        Commands:\n\
        \u{20}help   \tshows this help text\n\
        \u{20}encrypt\tencrypts INPUT and stores it in OUTPUT\n\
        \u{20}decrypt\tdecrypts INPUT and stores it in OUTPUT\n\
        \n\
        INPUT must exist\n\
        OUTPUT will be overwritten or created if it does not exist\n\
    ");
    if args.len() < 2 || args.len() > 5 {
        println!("{}", help);
        process::exit(0);
    }

    match args[1].as_str() {
        "help" | "-h" => {
            println!("{}", help);
            None
        },
        "encrypt" | "enc" | "e" => {
            Some(ENCRYPT)
        },
        "decrypt" | "dec" | "d" => {
            Some(DECRYPT)
        }
        _ => {
            None
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mode = argparse(&args);

    match mode {
        Some(_) if args.len() != 4 => {
            println!("Invalid arguments. Please run `crypt help` for details.");
            process::exit(0)
        },
        Some(ENCRYPT) => {},
        Some(DECRYPT) => {},
        None => process::exit(0)
    }
    let mode = mode.unwrap();
    let (input, output) = (&args[2], &args[3]);
    let crypt: Vec<u8>;

    let mut key = String::new();
    io::stdin().read_line(&mut key).expect("failed to read key input");

    let mut key = String::from(key).into_bytes();
    key.resize(AES_SIZE, 95);
    let key = key;

    let mut msg = fs::read(&input).unwrap();
    msg.resize(msg.len() + 16 - (msg.len() % 16), 0);

    match mode {
        ENCRYPT => {
            let msg_16 = encrypt(&key, &mut msg);

            let mut msg: Vec<u8> = Vec::new();
            for e in msg_16 {
                msg.push(e as u8);
                msg.push((e >> 8) as u8);
            }

            crypt = msg;
        },
        DECRYPT => {
            let mut msg_16: Vec<u16> = msg
                .chunks_exact(2)
                .into_iter()
                .map(|e| u16::from_ne_bytes([e[0], e[1]]))
                .collect::<Vec<u16>>();
            let msg = decrypt(&key, &mut msg_16);

            crypt = msg;
        }
    }
    fs::write(output, &crypt).expect("failed to write to output file");
}

fn encrypt(key: &Vec<u8>, msg: &mut Vec<u8>) -> Vec<u16> {
    let mut msg_16: Vec<u16> = Vec::new();
    for e in msg {
        msg_16.push(*e as u16);
    }

    let mut i = 0;
    while i < msg_16.len() {
        msg_16[i..(i+4)].rotate_left(i%4);
        i += 4;
    }

    i = 0;
    while i < msg_16.len() {
        for n in 0..4 {
            msg_16[i+n] += key[(i+n) % key.len()] as u16;
        }
        i += 4;
    }

    msg_16
}

fn decrypt(key: &Vec<u8>, msg_16: &mut Vec<u16>) -> Vec<u8> {
    let mut i = 0;
    while i < msg_16.len() {
        for n in 0..4 {
            let k = key[(i+n) % key.len()] as u16;
            if msg_16[i+n] >= k {
                msg_16[i+n] -= k;
            }
        }
        i += 4;
    }
    msg_16.reverse();

    i = 0;
    while i < msg_16.len() {
        msg_16[i..(i+4)].rotate_right(i%4);
        i += 4;
    }
    msg_16.reverse();

    let mut msg: Vec<u8> = Vec::new();
    for e in msg_16.iter() {
        msg.push(*e as u8);
    }

    msg
}
