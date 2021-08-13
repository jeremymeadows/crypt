use std::{env, fs, io, process};

use libcrypt::chacha::ChaCha;
use libcrypt::mersenne_twister::Generator;
use libcrypt::stdin_extras::Input;

fn help() -> ! {
    println!(
        "Crypt v1.0.0

Usage:
    crypt <MODE> <INPUT> [OUTPUT]

Crypt uses ChaCha20 to encrypt/decrypt your files.

MODES:
    encrypt    Encrypts INPUT and stores it in OUTPUT. 
    decrypt    Decrypts INPUT and stores it in OUTPUT.
    help       Shows this help text.
        
ARGS:
    INPUT     The input file to use.
    OUTPUT    The output file to use.
              In 'encrypt' mode, will default to 'input.crypt'.
              In 'decrypt' mode, will default to overwriting the input.
        
Examples:
  crypt encrypt foo.txt
    - saves an encrypted version of foo.txt at ./foo.txt.crypt
  crypt e foo.txt secret_msg
    - saves an encrypted version of foo.txt at ./secret_msg
  crypt decrypt foo.txt.crypt
    - saves a decrypted version of foo.txt.crypt at ./foo.txt.crypt
  crypt dec bar.crypt pic.png
    - saves a decrypted version of bar.crypt at ./pic.png
"
    );

    process::exit(0);
}

fn main() -> io::Result<()> {
    let (mode, input, output) = argparse();

    let key = io::stdin().input_hidden("Enter Crypt key:")?.into_bytes();

    let mut gen = Generator::from(&key);
    let mut cc = ChaCha::from_state(
        &key,
        0,
        [gen.next() as u32, gen.next() as u32, gen.next() as u32],
    );

    match fs::metadata(&input)?.is_file() {
        true => {
            let mut contents = fs::read(input)?;
            match mode {
                Mode::ENCRYPT => contents = cc.encrypt(&contents),
                Mode::DECRYPT => contents = cc.decrypt(&contents),
            }
            fs::write(output, &contents)?;
            Ok(())
        }
        false => match mode {
            Mode::ENCRYPT => {
                let input = input + "/";
                let temp_out = format!(".crypt.temp.{}", &output);
                let mut meta: Vec<(String, String)> = Vec::new();

                match fs::remove_dir_all(&temp_out) {
                    _ => fs::create_dir(&temp_out)?,
                }

                // Recursively gets all directory contents.
                fn get_dir_contents(path: &str) -> Result<Vec<String>, io::Error> {
                    let mut v = Vec::new();

                    for e in fs::read_dir(&path)? {
                        let path = e?.path();
                        let path = path.to_str().ok_or(io::Error::new(
                            io::ErrorKind::NotFound,
                            "could not read path as UTF-8",
                        ))?;

                        if fs::metadata(&path)?.is_dir() {
                            v.append(&mut get_dir_contents(&path)?);
                        } else {
                            v.push(path.to_string());
                        }
                    }

                    Ok(v)
                }

                let mut dirs = Vec::new();

                for e in get_dir_contents(&input)? {
                    match e.rfind('/') {
                        Some(i) => {
                            if !dirs.contains(&e[0..i].to_string()) {
                                dirs.push(e[0..i].to_string());
                            }
                        }
                        None => {}
                    }

                    meta.push((format!("{:02X}", meta.len() + 1), e.replace(&input, "")));
                }
                for d in dirs {
                    fs::create_dir_all(&d)?;
                }

                let mut meta_file = String::new();
                let mut data;

                for i in &meta {
                    meta_file += &format!("{}\n{}\n", i.0, i.1);
                }

                fs::write(
                    format!("{}/00", &temp_out),
                    cc.encrypt(&Vec::from(meta_file.trim())),
                )?;

                for i in &meta {
                    data = cc.encrypt(&fs::read(format!("{}/{}", &input, i.1))?);
                    fs::write(format!("{}/{}", &temp_out, i.0), data)?;
                }

                match fs::remove_dir_all(&output) {
                    _ => fs::rename(&temp_out, &output)?,
                }

                Ok(())
            }
            Mode::DECRYPT => {
                let temp_out = format!(".crypt.temp.{}", &output);

                let meta = String::from_utf8(
                    cc.decrypt(&fs::read(format!("{}/00", input))?)
                ).expect("failed to decrypt the meta file");

                match fs::remove_dir_all(&temp_out) {
                    _ => fs::create_dir(&temp_out)?,
                }

                let meta = meta.split('\n').collect::<Vec<&str>>();
                let mut data;

                for line in (0..(meta.len())).step_by(2) {
                    let file = meta[line + 1];
                    data = cc.decrypt(&fs::read(format!("{}/{}", &input, meta[line]))?);

                    match file.rfind('/') {
                        Some(i) => {
                            fs::create_dir_all(format!(
                                "{}/{}",
                                &temp_out,
                                file[0..i].to_string(),
                            ))?;
                        }
                        None => {}
                    };

                    fs::write(format!("{}/{}", &temp_out, file), data)?;
                }

                match fs::remove_dir_all(&output) {
                    _ => fs::rename(&temp_out, &output)?,
                }

                Ok(())
            }
        }
    }
}

enum Mode {
    ENCRYPT,
    DECRYPT,
}

fn argparse() -> (Mode, String, String) {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 3 || args.len() > 5 {
        help();
    }

    let mode = match args[1].as_str() {
        "help" | "h" | "--help" | "-h" => help(),
        "encrypt" | "enc" | "e" => Mode::ENCRYPT,
        "decrypt" | "dec" | "d" => Mode::DECRYPT,
        _ => help(),
    };
    let in_file = args[2].trim_end_matches('/').to_string();
    let out_file = if args.len() > 3 {
        args[3].clone()
    } else {
        match mode {
            Mode::ENCRYPT => format!("{}.crypt", in_file),
            Mode::DECRYPT => format!("{}", in_file),
        }
    };

    (mode, in_file, out_file)
}
