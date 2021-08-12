use std::{io, env, fs, process};

use libcrypt::chacha::{Encryptable, Decryptable};
use libcrypt::Mode::{self, ENCRYPT, DECRYPT};
use libcrypt::read_hidden::Input;

fn help() -> ! {
    println!("\nUsage: crypt COMMAND INPUT [OUTPUT]\n
Crypt uses key-based AES to encrypt/decrypt your files.

Commands:
  help   \tshows this help text
  encrypt\tencrypts INPUT and stores it in OUTPUT
  decrypt\tdecrypts INPUT and stores it in OUTPUT
        
INPUT must exist
OUTPUT will be overwritten or created if it does not exist
        
Examples:
  crypt encrypt foo.txt
    - saves an encrypted version of foo.txt at ./foo.txt.crypt
  crypt e foo.txt secret_msg
    - saves an encrypted version of foo.txt at ./secret_msg
  crypt decrypt foo.txt.crypt
    - saves a decrypted version of foo.txt.crypt at ./foo.txt.crypt
  crypt dec bar.crypt pic.png
    - saves a decrypted version of bar.crypt at ./pic.png
");
    process::exit(0);
}

fn argparse() -> (Mode, String, String) {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 3 || args.len() > 5 {
        help();
    }

    let mode = match args[1].as_str() {
        "help" | "-h" => {
            help();
        },
        "encrypt" | "enc" | "e" => { ENCRYPT },
        "decrypt" | "dec" | "d" => { DECRYPT }
        _ => {
            help();
        }
    };
    let in_file = args[2].trim_end_matches('/').to_string();
    let out_file = if args.len() > 3 {
        args[3].clone()
    } else {
        match mode {
            ENCRYPT => format!("{}.crypt", in_file),
            DECRYPT => format!("{}", in_file),
        }
    };

    (mode, in_file, out_file)
}

fn main() -> io::Result<()> {
    let (mode, input, output) = argparse();

    let key = io::stdin().input_hidden("Enter Crypt key:")?.into_bytes();

    match fs::metadata(&input).expect("failed to collect file metadata").is_file() {
        true => {
            let mut contents = fs::read(input).expect("failed to open file for reading");
            match mode {
                ENCRYPT => contents = contents.encrypt(&key),
                DECRYPT => contents = contents.decrypt(&key),
            }
            fs::write(output, &contents)?;
            Ok(())
        },
        false => {
            match mode {
                ENCRYPT => {
                    let input = input + "/";
                    let temp_out = format!(".crypt.temp.{}", &output);
                    let mut meta: Vec<(String, String)> = Vec::new();

                    match fs::remove_dir_all(&temp_out) {
                        _ => fs::create_dir(&temp_out)?
                    }

                    fn get_dir_contents(path: &str) -> Result<Vec<String>, ()> {
                        let mut v = Vec::new();

                        for e in fs::read_dir(&path).unwrap() {
                            let path = e.unwrap().path();
                            let path = path.to_str().unwrap();

                            if fs::metadata(&path).unwrap().is_dir() {
                                v.append(&mut get_dir_contents(&path)?);
                            } else {
                                v.push(path.to_string());
                            }
                        }

                        Ok(v)
                    }

                    let mut dirs = Vec::new();

                    for e in get_dir_contents(&input).unwrap() {
                        match e.rfind('/') {
                            Some(i) => if !dirs.contains(&e[0..i].to_string()) {
                                dirs.push(e[0..i].to_string());
                            },
                            None => (),
                        }

                        meta.push((format!("{:02X}", meta.len()+1), e.replace(&input, "")));
                    }
                    for d in dirs {
                        fs::create_dir_all(&d)?;
                    }

                    let mut meta_file = String::new();
                    let mut data;

                    for i in meta {
                        meta_file += &format!("{}\n{}\n", i.0, i.1);
                        data = fs::read(format!("{}/{}", &input, i.1)).expect("failed to read one of the input files").encrypt(&key);
                        fs::write(format!("{}/{}", &temp_out, i.0), data)?;
                    }

                    let meta = Vec::from(meta_file.trim().as_bytes()).encrypt(&key);
                    fs::write(format!("{}/00", &temp_out), meta)?;

                    match fs::remove_dir_all(&output) {
                        _ => fs::rename(&temp_out, &output)?
                    }

                    Ok(())
                },
                DECRYPT => {
                    let temp_out = format!(".crypt.temp.{}", &output);
                    let meta = String::from_utf8(fs::read(format!("{}/00", input)).expect("failed to read one of the input files").decrypt(&key)).expect("failed to decrypt the meta file");

                    match fs::remove_dir_all(&temp_out) {
                        _ => fs::create_dir(&temp_out)?
                    }

                    let meta = meta.split('\n').collect::<Vec<&str>>();
                    let mut data;

                    for line in (0..meta.len()).step_by(2) {
                        let file = meta[line + 1];
                        data = fs::read(format!("{}/{}", &input, meta[line]))?.decrypt(&key);

                        match file.rfind('/') {
                            Some(i) => fs::create_dir_all(format!("{}/{}", &temp_out, file[0..i].to_string()))?,
                            None => (),
                        };

                        fs::write(format!("{}/{}", &temp_out, file), data)?;
                    }

                    match fs::remove_dir_all(&output) {
                        _ => fs::rename(&temp_out, &output)?
                    }

                    Ok(())
                },
            }
        }
    }
}
