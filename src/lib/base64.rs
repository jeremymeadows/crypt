const BASE64: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+-=";

pub fn encode(v: &Vec<u8>) -> String {
    let mut bstr = String::new();
    let mut b64str = String::new();

    for e in v {
        bstr = format!("{}{:0>8b}", bstr, e);
    }

    let pad = 6 - (bstr.len() as u8 % 6);
    for _ in 0..pad {
        bstr.push('0');
    }

    for i in 0..((bstr.len() as u8 + pad) / 6) {
        let n = 6 * i as usize;
        let c = usize::from_str_radix(&bstr[(n)..(6 + n)], 2).unwrap();
        b64str = format!("{}{}", b64str, BASE64.get(c..(c + 1)).unwrap());
    }

    for _ in 0..(4 - (b64str.len() % 4)) {
        b64str = format!("{}{}", b64str, "=");
    }

    b64str
}

pub fn decode(b64str: &String) -> Vec<u8> {
    let mut bstr = String::new();
    let mut v = Vec::<u8>::new();

    for c in b64str.chars() {
        if c != '=' {
            bstr = format!("{}{:0>6b}", bstr, BASE64.find(c).unwrap());
        }
    }

    for i in 0..(bstr.len() as u8 / 8) {
        let n = 8 * i as usize;
        v.push(u8::from_str_radix(&bstr[(n)..(8 + n)], 2).unwrap());
    }

    v
}
