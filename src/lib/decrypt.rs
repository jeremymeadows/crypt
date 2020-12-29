use std::fs;
use std::path::Path;

pub trait Decryptable {
    fn decrypt(&self, key: &Vec<u8>, dest: &Path);
}

impl Decryptable for Path {
    fn decrypt(&self, key: &Vec<u8>, dest: &Path) {
        let mut contents = fs::read(self).expect("failed to open file for reading");

        contents = decrypt(&key, &mut contents);
        fs::write(dest, contents).expect("failed to write to output file");
    }
}

pub fn decrypt(key: &Vec<u8>, msg: &mut Vec<u8>) -> Vec<u8> {
    let mut msg_16: Vec<u16> = msg
        .chunks_exact(2)
        .into_iter()
        .map(|e| u16::from_ne_bytes([e[0], e[1]]))
        .collect::<Vec<u16>>();

    for _ in 0..16 {
        msg_16 = decrypt_16(&key, &mut msg_16);
    }
    let mut msg = decrypt_8(&key, &mut msg_16);

    let ext = msg[msg.len()-16] as usize;
    msg.resize(msg.len() - ext - 16, 0);

    msg
}

fn decrypt_8(key: &Vec<u8>, msg_16: &mut Vec<u16>) -> Vec<u8> {
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

fn decrypt_16(key: &Vec<u8>, msg_16: &mut Vec<u16>) -> Vec<u16> {
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

    msg_16.to_vec()
}
