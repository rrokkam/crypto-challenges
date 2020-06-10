use crate::score::Scorer;
use std::cmp;
use std::collections::HashMap;

fn xor_with(buffer: &[u8], byte: u8) -> Vec<u8> {
    buffer.into_iter().map(|b| b ^ byte).collect()
}

fn xor_to_string(buffer: &[u8], byte: u8) -> Option<String> {
    String::from_utf8(xor_with(buffer, byte)).ok()
}

fn break_single_byte_xor(buffer: Vec<u8>, freqs: &Scorer) -> Option<String> {
    (u8::MIN..=u8::MAX)
        .filter_map(|n| xor_to_string(&buffer.clone(), n))
        .max_by(|a, b| freqs.score(a).partial_cmp(&freqs.score(b)).unwrap())
}

fn single_byte_xor(buffer: &[u8], c: u8) -> Vec<u8> {
    buffer.iter().map(|b| b ^ c).collect()
}

/// Will return None if none of the xors yielded a valid UTF8 encoding
fn decrypt_single_byte_xor(ciphertext: &[u8], freqs: &Scorer) -> Option<(f64, String)> {
    (u8::MIN..=u8::MAX)
        .filter_map(|c| String::from_utf8(single_byte_xor(ciphertext, c)).ok())
        .map(|text| (freqs.score(&text), text))
        .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
}

fn find_single_byte_xor(ciphertexts: Vec<&[u8]>, freqs: &Scorer) -> String {
    ciphertexts
        .iter()
        .map(|&ciphertext| decrypt_single_byte_xor(ciphertext, freqs))
        .filter_map(|x| x)
        .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
        .unwrap() // assert that at least one decoding was valid. TODO: remove this and return Option<String> instead.
        .1
}

fn fixed_xor(first: &[u8], second: &[u8]) -> Vec<u8> {
    first
        .iter()
        .zip(second.iter())
        .map(|(a, b)| a ^ b)
        .collect()
}

fn repeating_key_xor(ciphertext: &[u8], key: &[u8]) -> Vec<u8> {
    ciphertext
        .iter()
        .zip(key.iter().cycle())
        .map(|(a, b)| a ^ b)
        .collect()
}

fn edit_distance(first: &[u8], second: &[u8]) -> u32 {
    fixed_xor(first, second)
        .iter()
        .map(|&a| a.count_ones())
        .sum()
}

fn weighted_edit_distance(first: &[u8], second: &[u8]) -> f64 {
    if first.len() != second.len() {
        panic!("Weighted edit distance on slices of different sizes")
    }
    edit_distance(first, second) as f64 / first.len() as f64
}

fn find_repeating_xor_key_length(ciphertext: &[u8]) -> usize {
    let compare_blocks =
        |a| weighted_edit_distance(&ciphertext[..a - 1], &ciphertext[a..(a * 2 - 1)]) / a as f64;

    (2..cmp::min(40, 2 * ciphertext.len()))
        .min_by(|&n, &m| { compare_blocks(n).partial_cmp(&compare_blocks(m)) }.unwrap())
        .unwrap()
}

fn break_repeating_key_xor(ciphertext: &[u8]) -> String {
    let chunk_size = find_repeating_xor_key_length(ciphertext);
    let blocks = ciphertext.chunks(chunk_size);
    let mut transposed: Vec<Vec<u8>> = Vec::with_capacity(chunk_size);
    for block in blocks {
        for (i, &c) in (0..chunk_size).zip(block) {
            transposed[i].push(c);
        }
    }
    for group in transposed {
        // get the byte for the key
    }
    String::new() // concatenate the bytes obtained for each key
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex;
    use hex_literal;
    use std::fs;
    use std::io::BufRead;

    const CORPUS_FILE_PATH: &str = "ulysses.txt";

    #[test]
    fn test_fixed_xor() {
        let first = hex_literal::hex!("1c0111001f010100061a024b53535009181c");
        let second = hex_literal::hex!("686974207468652062756c6c277320657965");
        let xored = fixed_xor(&first, &second);

        assert_eq!(
            xored,
            hex_literal::hex!("746865206b696420646f6e277420706c6179")
        );
    }

    #[test]
    fn test_decrypt_single_byte_xor() {
        let ciphertext = hex_literal::hex!(
            "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"
        );
        let corpus = fs::read_to_string(CORPUS_FILE_PATH).unwrap();
        let freqs = Scorer::new(&corpus);

        let (_, plaintext_guess) = decrypt_single_byte_xor(&ciphertext, &freqs).unwrap();
        assert_eq!(plaintext_guess, "Cooking MC's like a pound of bacon");
    }

    #[test]
    fn test_find_single_byte_xor() {
        let ciphertexts: Vec<Vec<u8>> = fs::read("single_byte_xored.txt")
            .unwrap()
            .lines()
            .map(|ciphertext| hex::decode(ciphertext.unwrap()).unwrap())
            .collect();

        let ciphertexts = ciphertexts.iter().map(|item| item.as_slice()).collect();

        let corpus = fs::read_to_string(CORPUS_FILE_PATH).unwrap();
        let freqs = Scorer::new(&corpus);

        let plaintext_guess = find_single_byte_xor(ciphertexts, &freqs);

        assert_eq!(plaintext_guess, "Now that the party is jumping\n");
    }

    #[test]
    fn test_repeating_key_xor() {
        let plaintext: Vec<u8> =
            b"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal".to_vec();
        let key: &[u8] = b"ICE";

        let result = repeating_key_xor(&plaintext, key);

        let expected = hex_literal::hex!(
            "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272
            a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
        )
        .to_vec();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_edit_distance() {
        let first = b"this is a test";
        let second = b"wokka wokka!!!";

        assert_eq!(edit_distance(first, second), 37);
    }

    #[test]
    fn test_break_repeating_key_xor() {
        let ciphertext =
            base64::decode(fs::read_to_string("repeating_key_xored.txt").unwrap()).unwrap();

        let plaintext_guess = break_repeating_key_xor(&ciphertext);

        println!("{}", plaintext_guess);
        panic!("print");
    }
}
