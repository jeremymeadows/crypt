use std::{env, fs, thread};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

//use libcrypt::encrypt::*;
//use libcrypt::decrypt::*;
use libcrypt::Mode;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (_key, src, _dest) = (&args[1], &args[2], &args[3]);
    let mode: Mode;
    let mut dir = HashMap::<String, (u128, bool)>::new();
    let mut dirt = 0;

    println!("daemon starting");

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
            // gets path of all files
            let path = match f.to_str() {
                Some(path) => path.to_string(),
                None => continue
            };
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
                            println!("{} -> {}", path, t);
                            dir.insert(path, (t, false));
                        }
                    },
                    None => {
                        println!("{} : {}", path, t);
                        dir.insert(path, (t, false));
                    }
                }
            } else {
                // is another directory
            }
        }

        for f in dir.clone() {
            let (p, (t, e)) = f;
            if !e {
                println!("recrypting {}", p);
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
