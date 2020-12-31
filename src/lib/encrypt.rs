pub trait Encryptable {
    fn encrypt(&self, key: &Vec<u8>) -> Vec<u8>;
}

impl Encryptable for Vec<u8> {
    fn encrypt(&self, key: &Vec<u8>) -> Vec<u8> {
        encrypt(&key, &mut self.clone())
    }
}

pub fn encrypt(key: &Vec<u8>, mut msg: &mut Vec<u8>) -> Vec<u8> {
    let ext = (16 - (msg.len() % 16)) % 16;
    msg.resize(msg.len() + ext, 0);
    msg.push(ext as u8);
    msg.append(&mut vec![0u8; 15]);

    let mut msg_16 = encrypt_8(&key, &mut msg);

    for _ in 0..16 {
        msg_16 = encrypt_16(&key, &mut msg_16);
    }

    let mut msg: Vec<u8> = Vec::new();
    for e in msg_16 {
        msg.push(e as u8);
        msg.push((e >> 8) as u8);
    }

    msg
}

fn encrypt_8(key: &Vec<u8>, msg: &mut Vec<u8>) -> Vec<u16> {
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

fn encrypt_16(key: &Vec<u8>, msg_16: &mut Vec<u16>) -> Vec<u16> {
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

    msg_16.to_vec()
}
