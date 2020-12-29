pub fn decrypt(key: &Vec<u8>, msg_16: &mut Vec<u16>) -> Vec<u8> {
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

pub fn decrypt_16(key: &Vec<u8>, msg_16: &mut Vec<u16>) -> Vec<u16> {
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
