use std::{io, env, fs};
use std::process::{self, Command, Stdio};

use libcrypt::encrypt::*;
use libcrypt::decrypt::*;
use libcrypt::Mode::{self, ENCRYPT, DECRYPT};

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
        "encrypt" | "enc" | "e" => { Some(ENCRYPT) },
        "decrypt" | "dec" | "d" => { Some(DECRYPT) }
        _ => { None }
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
    println!("Enter crypt key: ");
    io::stdin().read_line(&mut key).expect("failed to read key input");
    println!("\u{1b}[F{}", String::from_utf8(vec![0x20; key.len()]).unwrap());

    let mut key = String::from(key).into_bytes();
    key.resize(AES_SIZE, 95);
    let key = key;

    let file = fs::metadata(&input).expect("failed to collect file metadata");
    match file.is_file() {
        true => {
            let mut msg = fs::read(&input).expect("failed to open file for reading");

            match mode {
                ENCRYPT => {
                    let ext = (16 - (msg.len() % 16)) % 16;
                    msg.resize(msg.len() + ext, 0);
                    msg.push(ext as u8);
                    msg.append(&mut vec![0u8; 15]);

                    let mut msg_16 = encrypt(&key, &mut msg);
                    for _ in 0..16 {
                        msg_16 = encrypt_16(&key, &mut msg_16);
                    }

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

                    for _ in 0..16 {
                        msg_16 = decrypt_16(&key, &mut msg_16);
                    }
                    let mut msg = decrypt(&key, &mut msg_16);

                    let ext = msg[msg.len()-16] as usize;
                    msg.resize(msg.len() - ext - 16, 0);

                    crypt = msg;
                }
            }
            fs::write(output, &crypt).expect("failed to write to output file");
        },
        false => {
            let d = Command::new("cargo")
                .args(&["run", "--bin", "cryptd"])
                .args(&["--", String::from_utf8(key).unwrap().as_str(), &input, &output])
                // .stderr(Stdio::null())
                .stdout(Stdio::null())
                .spawn()
                .expect("failed to start daemon");
            println!("started daemon as pid {}", d.id());
        }
    }
}
