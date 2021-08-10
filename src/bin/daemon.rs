use std::{io, env, fs, thread};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

use libcrypt::base64;
use libcrypt::encrypt::Encryptable;
use libcrypt::decrypt::Decryptable;
use libcrypt::Mode::{self, ENCRYPT, DECRYPT};

fn main() {
    let args: Vec<String> = env::args().collect();

    let (mode, input, output) = (Mode::from(&args[1]), &args[2].trim_end_matches('/'), &args[3].trim_end_matches('/'));
    let key = if args.len() == 5 {
        args[4].trim().to_string().into_bytes()
    } else {
        let mut key = String::new();
        println!("Enter crypt key: ");
        io::stdin().read_line(&mut key).expect("failed to read key input");
        println!("\u{1b}[F{}", String::from_utf8(vec![0x20; key.len()]).unwrap());

        key.into_bytes()
    };
    let mut dir = HashMap::<String, (u128, bool)>::new();
    let mut dirt = 0;

    println!("daemon starting");

    let (mode, clear, crypt) = match mode {
        Some(ENCRYPT) => (ENCRYPT, input, output),
        Some(DECRYPT) | None => {
            (DECRYPT, output, input)
        },
    };

    let (clear, crypt) = match mode {
        DECRYPT => {
            for f in fs::read_dir(&clear).expect("failed to read encrypted directory") {
                let f = f.unwrap().path();
                // gets path to the file
                let path = match f.to_str() {
                    Some(path) => path.to_string(),
                    None => continue
                };
                // gets file name
                let file = path.rsplit('/').collect::<Vec<&str>>()[0].to_string();

                // let name = String::from_utf8(base64::decode(&file).decrypt(&key)).unwrap();
                let name = String::from_utf8(base64::decode(&file)).unwrap();
                let (input, output) = (format!("{}/{}", clear, file), format!("{}/{}", crypt, name));

                let mut contents = fs::read(&input).expect("failed to open file for reading");
                contents = contents.decrypt(&key);
                fs::write(output, contents).expect("failed to write to output file");
            }
            (crypt, clear)
        },
        _ => (clear, crypt)
    };

    loop {
        match fs::metadata(&clear) {
            Ok(data) => {
                let dt = data.modified().unwrap().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
                if dirt < dt {
                    dirt = dt;
                    println!("dir modified");
                } else {
                    println!("no mod - sleeping");
                    thread::sleep(Duration::from_millis(1000));
                    continue;
                }
            },
            Err(_) => {}
        }
        // only checks files owner by current user with open read access
        for f in fs::read_dir(&clear).expect("failed to read cleartext directory") {
            let f = f.unwrap().path();
            // gets path to the file
            let path = match f.to_str() {
                Some(path) => path.to_string(),
                None => continue
            };
            // gets file name
            let file = path.rsplit('/').collect::<Vec<&str>>()[0].to_string();
            // gets metadata of files
            let meta = match fs::metadata(&path) {
                Ok(data) => data,
                Err(_) => continue
            };

            if meta.is_file() {
                let t = meta.modified().unwrap().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
                match dir.get(&path) {
                    Some((v, _)) => {
                        if v < &t {
                            dir.insert(file, (t, false));
                        }
                    },
                    None => {
                        dir.insert(file, (t, false));
                    }
                }
            } else {
                // is a subdirectory
            }
        }

        for f in dir.clone() {
            let (file, (time, encrypted)) = f;
            if !encrypted {
                // let name = base64::encode(&file.as_bytes().to_vec().encrypt(&key.to_vec())).replace("=", "");
                let name = base64::encode(&file.as_bytes().to_vec()).replace("=", "");
                let (input, output) = (format!("{}/{}", clear, file), format!("{}/{}", crypt, name));

                let mut contents = fs::read(&input).expect("failed to open file for reading");
                contents = contents.encrypt(&key);
                fs::write(output, contents).expect("failed to write to output file");

                dir.insert(file, (time, true));
            } else {
                match fs::metadata(format!("{}/{}", clear, file)) {
                    Ok(_) => (),
                    Err(_) => {
                        // let name = base64::encode(&file.as_bytes().to_vec().encrypt(&key.to_vec())).replace("=", "");
                        let name = base64::encode(&file.as_bytes().to_vec()).replace("=", "");
                        fs::remove_file(format!("{}/{}", crypt, name)).unwrap();
                    }
                }
            }
        }
    }
}
