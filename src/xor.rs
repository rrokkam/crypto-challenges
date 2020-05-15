#[allow(dead_code)]
fn fixed_xor(first: Vec<u8>, second: Vec<u8>) -> Vec<u8> {
    if first.len() != second.len() {
        panic!("Can't XOR two bytestrings of different length");
    }
    first
        .iter()
        .zip(second.iter())
        .map(|(a, b)| a ^ b)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex::FromHex;

    #[test]
    fn test_fixed_xor() {
        let first = Vec::from_hex("1c0111001f010100061a024b53535009181c").unwrap();
        let second = Vec::from_hex("686974207468652062756c6c277320657965").unwrap();
        assert_eq!(
            fixed_xor(first, second),
            Vec::from_hex("746865206b696420646f6e277420706c6179").unwrap()
        );
    }
}
