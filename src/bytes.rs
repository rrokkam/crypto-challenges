pub struct Hex(String);

impl Hex {
    fn char_to_raw(hexchar: u8) -> u8 {
        match hexchar {
            b'0'..=b'9' => hexchar - b'0',
            b'a'..=b'f' => hexchar - b'a' + 10,
            _ => panic!("Invalid hexchar"),
        }
    }
}

impl From<Vec<u8>> for Hex {
    fn from(bytes: Vec<u8>) -> Self {
        Hex(bytes.iter().map(|byte| format!("{:02x}", byte)).collect())
    }
}

impl From<Hex> for Vec<u8> {
    fn from(hex: Hex) -> Self {
        hex.0
            .as_bytes()
            .chunks(2)
            .map(|pair| Hex::char_to_raw(pair[0]) << 4 | Hex::char_to_raw(pair[1]) & 0xF)
            .collect()
    }
}

pub struct Base64(String);

impl From<Vec<u8>> for Base64 {
    fn from(bytes: Vec<u8>) -> Self {
        Base64(base64::encode(bytes))
    }
}

impl From<Base64> for Vec<u8> {
    fn from(base: Base64) -> Self {
        base64::decode(base.0).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_from_vec() {
        let hex = Hex::from(vec![99, 111]);
        assert_eq!(hex.0, "636f");
    }

    #[test]
    fn vec_from_hex() {
        let vec = Vec::from(Hex("636f".to_string()));
        assert_eq!(vec, vec![99, 111]);
    }

    #[test]
    fn base64_from_vec() {
        let base64 = Base64::from("Man".as_bytes().to_vec());
        assert_eq!(base64.0, "TWFu".to_string());
    }

    #[test]
    fn vec_from_base64() {
        let vec = Vec::from(Base64("TWFu".to_string()));
        assert_eq!(vec, "Man".as_bytes().to_vec());
    }
}
