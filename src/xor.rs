use crate::score::{Score, Scorer};
use std::{cmp, iter};

fn xor<'a, T, U>(first: T, second: U) -> Vec<u8>
where
    T: IntoIterator<Item = &'a u8>,
    U: IntoIterator<Item = &'a u8>,
{
    first
        .into_iter()
        .zip(second.into_iter())
        .map(|(&a, &b)| a ^ b)
        .collect()
}

fn repeating_key_xor(ciphertext: &[u8], key: &[u8]) -> Vec<u8> {
    xor(ciphertext, key.iter().cycle())
}

fn decrypt_single_byte_xor(ciphertext: &[u8], freqs: &Scorer) -> Option<(Score, Vec<u8>)> {
    (u8::MIN..=u8::MAX)
        .map(|byte| xor(ciphertext, iter::repeat(&byte)))
        .max_by_key(|text| freqs.score(text))
        .map(|text| (freqs.score(&text), text))
}

fn find_single_byte_xor(ciphertexts: Vec<&[u8]>, freqs: &Scorer) -> Option<Vec<u8>> {
    ciphertexts
        .iter()
        .filter_map(|&ciphertext| decrypt_single_byte_xor(ciphertext, freqs))
        .max_by_key(|a| a.0)
        .map(|a| a.1)
}

fn edit_distance(first: &[u8], second: &[u8]) -> u32 {
    xor(first, second).iter().map(|&a| a.count_ones()).sum()
}

fn keysize_score(ciphertext: &[u8], keysize: usize) -> f64 {
    let first = &ciphertext[..keysize];
    let second = &ciphertext[keysize..(keysize * 2)];
    edit_distance(first, second) as f64 / keysize as f64
}

fn best_keysizes(ciphertext: &[u8]) -> Vec<usize> {
    let mut sorted_keysizes = (2..cmp::min(40, 2 * ciphertext.len()))
        .map(|keysize| (keysize, keysize_score(ciphertext, keysize)))
        .collect::<Vec<_>>();
    println!("{:?}", sorted_keysizes);
    sorted_keysizes.sort_by(|n, m| m.1.partial_cmp(&n.1).unwrap());
    sorted_keysizes.iter().map(|&(size, _)| size).collect()
}

fn break_repeating_key_xor_size(ciphertext: &[u8], freqs: &Scorer, keysize: usize) -> Vec<u8> {
    todo!()
}

fn break_repeating_key_xor(ciphertext: &[u8], freqs: &Scorer) -> Vec<u8> {
    best_keysizes(ciphertext)
        .iter()
        .take(4)
        .map(|&keysize| break_repeating_key_xor_size(ciphertext, freqs, keysize))
        .max_by_key(|plaintext| freqs.score(plaintext))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::BufRead;

    const CORPUS_FILE_PATH: &str = "ulysses.txt";

    #[test]
    fn test_xor() {
        let first = hex_literal::hex!("1c0111001f010100061a024b53535009181c");
        let second = hex_literal::hex!("686974207468652062756c6c277320657965");
        let xored = xor(&first, &second);

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
        let corpus = fs::read(CORPUS_FILE_PATH).unwrap();
        let freqs = Scorer::new(&corpus);

        let (_, plaintext_guess) = decrypt_single_byte_xor(&ciphertext, &freqs).unwrap();
        assert_eq!(
            &plaintext_guess[..],
            &b"Cooking MC's like a pound of bacon"[..]
        );
    }

    #[test]
    fn test_find_single_byte_xor() {
        let ciphertexts: Vec<Vec<u8>> = fs::read("single_byte_xored.txt")
            .unwrap()
            .lines()
            .map(|ciphertext| hex::decode(ciphertext.unwrap()).unwrap())
            .collect();

        let ciphertexts = ciphertexts.iter().map(|item| item.as_slice()).collect();

        let corpus = fs::read(CORPUS_FILE_PATH).unwrap();
        let freqs = Scorer::new(&corpus);

        let plaintext_guess = find_single_byte_xor(ciphertexts, &freqs).unwrap();

        assert_eq!(plaintext_guess, b"Now that the party is jumping\n");
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
    fn test_keysize() {
        let ciphertext = fs::read_to_string("repeating_key_xored.txt")
            .unwrap()
            .replace("\n", "");

        let ciphertext = base64::decode(ciphertext).unwrap();

        assert!((keysize_score(&ciphertext, 2) - 2.5).abs() < std::f64::EPSILON);
        assert!((keysize_score(&ciphertext, 3) - 2.0).abs() < std::f64::EPSILON);
        assert!((keysize_score(&ciphertext, 4) - 3.5).abs() < std::f64::EPSILON);
        assert!((keysize_score(&ciphertext, 5) - 1.2).abs() < std::f64::EPSILON);
        assert!((keysize_score(&ciphertext, 16) - 3.0).abs() < std::f64::EPSILON);
    }

    #[test]
    fn test_break_repeating_key_xor() {
        let ciphertext = fs::read_to_string("repeating_key_xored.txt")
            .unwrap()
            .replace("\n", "");

        let ciphertext = base64::decode(ciphertext).unwrap();

        let corpus = fs::read(CORPUS_FILE_PATH).unwrap();
        let freqs = Scorer::new(&corpus);

        let plaintext_guess = break_repeating_key_xor(&ciphertext, &freqs);
        println!("{:?}", plaintext_guess);
    }
}
