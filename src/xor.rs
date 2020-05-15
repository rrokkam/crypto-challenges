use std::iter::FromIterator;

#[allow(dead_code)]
fn fixed_xor<T, U>(first: T, second: T) -> U
where
    T: IntoIterator<Item = u8>,
    U: FromIterator<u8>,
{
    //    if first.len() != second.len() {
    //        panic!("Can't XOR two bytestrings of different length");
    //    }
    first
        .into_iter()
        .zip(second.into_iter())
        .map(|(a, b)| a ^ b)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    #[test]
    fn test_fixed_xor() {
        let first = hex!("1c0111001f010100061a024b53535009181c").to_vec();
        let second = hex!("686974207468652062756c6c277320657965").to_vec();
        let xored: Vec<u8> = fixed_xor(first, second);
        assert_eq!(xored, hex!("746865206b696420646f6e277420706c6179"));
    }
}
