use crate::score;

fn fixed_xor(first: Vec<u8>, second: Vec<u8>) -> Vec<u8> {
    first
        .iter()
        .zip(second.iter())
        .map(|(a, b)| a ^ b)
        .collect()
}

fn single_byte_xor(buffer: Vec<u8>, c: u8) -> Vec<u8> {
    buffer.iter().map(|b| b ^ c).collect()
}

fn decrypt_single_byte_xor(ciphertext: Vec<u8>, corpus: &str) -> (f64, String) {
    let freqs = score::frequencies(corpus);
    (u8::MIN..=u8::MAX)
        .map(|c| String::from_utf8(single_byte_xor(ciphertext.clone(), c)))
        .filter_map(Result::ok)
        .map(|text| (score::score(&text, &freqs), text))
        .max_by(|a, b| a.0.partial_cmp(&b.0).expect("Got Inf or NaN as a score"))
        .expect("No single byte xor produced a valid UTF8-encoded string")
}

fn find_single_byte_xor(ciphertexts: Vec<Vec<u8>>, corpus: &str) -> String {
    let freqs = score::frequencies(corpus);
    ciphertexts
        .iter()
        .map(|text| {
            (
                score::score(
                    std::str::from_utf8(text.clone().as_slice()).unwrap(),
                    &freqs,
                ),
                decrypt_single_byte_xor(text.clone(), corpus),
            )
        })
        .max_by(|a, b| {
            ((a.1).0 - a.0)
                .partial_cmp(&((b.1).0 - b.0))
                .expect("Got Inf or NaN as a score")
        })
        .unwrap()
        .1
         .1
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;
    use std::fs;

    #[test]
    fn test_fixed_xor() {
        let first = hex!("1c0111001f010100061a024b53535009181c").to_vec();
        let second = hex!("686974207468652062756c6c277320657965").to_vec();
        let xored = fixed_xor(first, second);
        assert_eq!(xored, hex!("746865206b696420646f6e277420706c6179").to_vec());
    }

    #[test]
    fn test_decrypt_single_byte_xor() {
        let ciphertext =
            hex!("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736").to_vec();
        let corpus = fs::read_to_string("ulysses.txt")
            .expect("No corpus found! Add a file called ulysses.txt in the crate root.");
        let (_, plaintext_guess) = decrypt_single_byte_xor(ciphertext, &corpus);
        assert_eq!(plaintext_guess, "Cooking MC's like a pound of bacon");
    }

    #[test]
    fn test_find_single_byte_xor() {
        let ciphertexts = fs::read("single_byte_xored.txt")
            .unwrap()
            .split(|c| *c == b'\n')
            .map(|v| v.to_vec())
            .collect();
        let corpus = fs::read_to_string("ulysses.txt")
            .expect("No corpus found! Add a file called ulysses.txt in the crate root.");
        let text = find_single_byte_xor(ciphertexts, &corpus);
        println!("{:#?}", text);
    }
}
