pub fn encode(v: &Vec<u8>) -> String {
    encode_with_delim(v, ':')
}
pub fn encode_with_delim(v: &Vec<u8>, delim: char) -> String {
    let mut xstr = String::new();

    for byte in v {
        xstr += &format!("{:02X}{}", byte, delim);
    }

    xstr.trim_matches(delim).to_string()
}

pub fn decode(xstr: &String) -> Vec<u8> {
    decode_with_delim(xstr, ':')
}
pub fn decode_with_delim(xstr: &String, delim: char) -> Vec<u8> {
    let mut v = Vec::new();
    let xstr = &xstr.replace(delim, "");

    for i in (0..xstr.len()).step_by(2) {
        v.push(u8::from_str_radix(&xstr[i..=i+1], 16).expect(""));
    }

    v
}

#[cfg(test)]
mod test {
    use crate::hex_string::*;

    static TESTS: &'static [(&'static str, &'static str)] = &[
        ("hi", "68:69"),
        ("", ""),
    ];

    #[test]
    fn test_encode() {
        for i in 0..TESTS.len() {
            let (s, e) = TESTS[i];

            println!("encoding '{}'", s);
            assert_eq!(encode(&Vec::from(s)), e);
        }
    }

    #[test]
    fn test_decode() {
        for i in 0..TESTS.len() {
            let (s, e) = TESTS[i];

            println!("decoding '{}'", s);
            assert_eq!(decode(&String::from(e)), Vec::from(s));
        }
    }

    #[test]
    fn test_symmetric() {
        for i in 0..TESTS.len() {
            let (s, e) = TESTS[i];

            println!("testing '{}'", s);
            assert_eq!(encode(&decode(&String::from(e))), e);
            assert_eq!(decode(&encode(&Vec::from(s))), Vec::from(s));
        }
    }
}
