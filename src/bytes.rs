pub struct Hex(String);

impl From<Vec<u8>> for Hex {
    fn from(bytes: Vec<u8>) -> Self {
        Hex("".to_string())
    }
}

impl From<Hex> for Vec<u8> {
    fn from(hex: Hex) -> Self {
        Vec::new()
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
}
