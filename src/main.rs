use std::{io, env, fs};
use std::process::{self, Command, Stdio};

use libcrypt::encrypt::Encryptable;
use libcrypt::decrypt::Decryptable;
use libcrypt::Mode::{self, ENCRYPT, DECRYPT};

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
        Some(_) => (),
        None => process::exit(0)
    }
    let mode = mode.unwrap();
    let (input, output) = (&args[2], &args[3]);

    let mut key = String::new();
    println!("Enter crypt key: ");
    io::stdin().read_line(&mut key).expect("failed to read key input");
    println!("\u{1b}[F{}", String::from_utf8(vec![0x20; key.len()]).unwrap());

    let key = String::from(key.trim()).into_bytes();
    println!("key: {:?}", key);

    match fs::metadata(&input).expect("failed to collect file metadata").is_file() {
        true => {
            let mut contents = fs::read(input).expect("failed to open file for reading");
            match mode {
                ENCRYPT => contents = contents.encrypt(&key),
                DECRYPT => contents = contents.decrypt(&key),
            }
            fs::write(output, contents).expect("failed to write to output file");
        },
        false => {
            let (m, clear, crypt) = match mode {
                ENCRYPT => ("e", input, output),
                DECRYPT => ("d", output, input),
            };
            let d = Command::new("cargo")
                .args(&["run", "--bin", "cryptd", "--"])
            // let d = Command::new("cryptd")
                .args(&[m, clear, crypt, String::from_utf8(key).unwrap().as_str()])
                .stderr(Stdio::null())
                // .stdout(Stdio::null())
                .spawn()
                .expect("failed to start daemon");
            println!("started daemon as pid {}", d.id());
        }
    }
}
