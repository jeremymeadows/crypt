const BASE64: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+-=";

pub fn encode(v: &Vec<u8>) -> String {
    // string representing the individual bits of the input vec
    let mut bstr = String::new();
    // string which stores the base64 encoding
    let mut b64str = String::new();

    if v.len() != 0 {
        for b in v {
            bstr = format!("{}{:0>8b}", bstr, b);
        }

        while bstr.len() % 6 != 0 {
            bstr.push('0');
        }

        for i in 0..(bstr.len() / 6) {
            let n = 6 * i;
            let c = usize::from_str_radix(&bstr[(n)..(6 + n)], 2).unwrap();
            b64str = format!("{}{}", b64str, BASE64.get(c..(c + 1)).unwrap());
        }

        while b64str.len() % 4 != 0 {
            b64str += "=";
        }
    }

    b64str
}

pub fn decode(b64str: &String) -> Vec<u8> {
    // string representing the individual bits of the input vec
    let mut bstr = String::new();
    // vec which stores the decoded bytes
    let mut v = Vec::<u8>::new();

    for c in b64str.trim_matches('=').chars() {
        bstr = format!("{}{:0>6b}", bstr, BASE64.find(c).unwrap());
    }

    for i in 0..(bstr.len() / 8) {
        let n = 8 * i;
        v.push(u8::from_str_radix(&bstr[(n)..(8 + n)], 2).unwrap());
    }

    v
}

#[cfg(test)]
mod test {
    use crate::base64::*;

    static TESTS: &'static [(&'static str, &'static str)] = &[
        ("foo", "Zm9v"),
        ("Hello, world!", "SGVsbG8sIHdvcmxkIQ=="),
        ("The quick brown fox jumped over the lazy dog.", "VGhlIHF1aWNrIGJyb3duIGZveCBqdW1wZWQgb3ZlciB0aGUgbGF6eSBkb2cu"),
        ("", ""),
    ];

    #[test]
    fn test_encode() {
        for i in 0..TESTS.len() {
            let (s, e) = TESTS[i];

            println!("encoding '{}'", s);
            assert_eq!(encode(&Vec::from(s.as_bytes())).as_str(), e);
        }
    }

    #[test]
    fn test_decode() {
        for i in 0..TESTS.len() {
            let (s, e) = TESTS[i];

            println!("decoding '{}'", s);
            assert_eq!(decode(&String::from(e)), Vec::from(s.as_bytes()));
        }
    }

    #[test]
    fn test_symmetric() {
        for i in 0..TESTS.len() {
            let (s, e) = TESTS[i];

            println!("testing '{}'", s);
            assert_eq!(encode(&decode(&String::from(e))).as_str(), e);
            assert_eq!(decode(&encode(&Vec::from(s.as_bytes()))), Vec::from(s.as_bytes()));
        }
    }
}

