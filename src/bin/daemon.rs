use std::{io, env, fs, thread};
use std::collections::HashMap;
use std::path::Path;
use std::time::{Duration, SystemTime};

use libcrypt::base64;
use libcrypt::encrypt::Encryptable;
use libcrypt::decrypt::Decryptable;
use libcrypt::Mode;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (_mode, src, dest, key) = if args.len() == 5 {
        (&args[1], &args[2], &args[3], args[4].trim().to_string().into_bytes())
    } else {
        let mut key = String::new();
        println!("Enter crypt key: ");
        io::stdin().read_line(&mut key).expect("failed to read key input");
        println!("\u{1b}[F{}", String::from_utf8(vec![0x20; key.len()]).unwrap());

        (&args[1], &args[2], &args[3], key.into_bytes())
    };
    let mut dir = HashMap::<String, (u128, bool)>::new();
    let mut dirt = 0;

    println!("daemon starting");

    fs::create_dir(dest);

    loop {
        match fs::metadata(&src) {
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
        for f in fs::read_dir(&src).expect("failed to read source directory") {
            let f = f.unwrap().path();
            // gets path to the file
            let path = match f.to_str() {
                Some(path) => path.to_string(),
                None => continue
            };
            // gets file name
            let file = path.rsplit('/').collect::<Vec<&str>>()[0];
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
                            // println!("{} -> {}", path, t);
                            dir.insert(path, (t, false));
                        }
                    },
                    None => {
                        // println!("{} : {}", path, t);
                        // println!("\nfrom {}{}", src, file);
                        let name = base64::encode(file.to_string().encrypt_to_vec(&key.to_vec())).replace("=", "");
                        // println!("to {}{}", dest, name);
                        Path::new(&format!("{}{}", src, file)).encrypt(&key.to_vec(), Path::new(&format!("{}{}", dest, name)));
                        dir.insert(file.to_string(), (t, false));
                    }
                }
            } else {
                // is a subdirectory
            }
        }

        for f in dir.clone() {
            let (p, (t, e)) = f;
            if !e {
                // println!("recrypting {}", p);
                dir.insert(p, (t, true));
            } else {
                match fs::metadata(&p) {
                    Ok(_) => (),
                    Err(_) => println!("{} -X", p)
                }
            }
        }
    }
}
