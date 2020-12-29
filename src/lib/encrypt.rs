pub fn encrypt(key: &Vec<u8>, msg: &mut Vec<u8>) -> Vec<u16> {
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

pub fn encrypt_16(key: &Vec<u8>, msg_16: &mut Vec<u16>) -> Vec<u16> {
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
