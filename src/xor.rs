fn fixed_xor(first: Vec<u8>, second: Vec<u8>) -> Vec<u8> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bytes::Hex;

    #[test]
    fn test_fixed_xor() {
        let first = Vec::from(Hex::new("1c0111001f010100061a024b53535009181c".to_string()));
        let second = b"686974207468652062756c6c277320657965".to_vec();
        assert_eq!(
            fixed_xor(first, second),
            b"746865206b696420646f6e277420706c6179".to_vec()
        );
    }
}
