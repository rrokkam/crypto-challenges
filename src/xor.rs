#[allow(dead_code)]
fn fixed_xor(first: Vec<u8>, second: Vec<u8>) -> Vec<u8> {
    first
        .iter()
        .zip(second.iter())
        .map(|(a, b)| a ^ b)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bytes::Hex;

    #[test]
    fn test_fixed_xor() {
        let first = Vec::from(Hex::new("1c0111001f010100061a024b53535009181c".to_string()));
        let second = Vec::from(Hex::new("686974207468652062756c6c277320657965".to_string()));
        assert_eq!(
            Hex::from(fixed_xor(first, second)),
            Hex::new("746865206b696420646f6e277420706c6179".to_string())
        );
    }
}
