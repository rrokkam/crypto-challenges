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

fn decrypt_single_byte_xor(ciphertext: &[u8], freqs: &Scorer) -> (Score, Vec<u8>, u8) {
    (u8::MIN..=u8::MAX)
        .map(|byte| (byte, xor(ciphertext, iter::repeat(&byte))))
        .max_by_key(|(_, text)| freqs.score(text))
        .map(|(byte, text)| (freqs.score(&text), text, byte))
        .unwrap() // we know that the range u8::MIN..=u8::MAX is nonempty, so max_by_key won't be None
}

fn find_single_byte_xor(ciphertexts: Vec<&[u8]>, freqs: &Scorer) -> Option<Vec<u8>> {
    ciphertexts
        .iter()
        .map(|&ciphertext| decrypt_single_byte_xor(ciphertext, freqs))
        .max_by_key(|a| a.0)
        .map(|a| a.1)
}

fn edit_distance(first: &[u8], second: &[u8]) -> u32 {
    xor(first, second).iter().map(|&a| a.count_ones()).sum()
}

fn keysize_score(ciphertext: &[u8], keysize: usize) -> f64 {
    let first = &ciphertext[..keysize];
    let second = &ciphertext[keysize..(keysize * 2)];
    let third = &ciphertext[(keysize * 2)..(keysize * 3)];
    (edit_distance(first, second) + edit_distance(second, third) + edit_distance(first, third))
        as f64
        / keysize as f64
}

fn best_keysizes(ciphertext: &[u8]) -> Vec<usize> {
    let mut sorted_keysizes = (2..cmp::min(40, 2 * ciphertext.len()))
        .map(|keysize| (keysize, keysize_score(ciphertext, keysize)))
        .collect::<Vec<_>>();
    sorted_keysizes.sort_by(|n, m| n.1.partial_cmp(&m.1).unwrap());
    sorted_keysizes.iter().map(|&(size, _)| size).collect()
}

fn break_repeating_key_xor_size(ciphertext: &[u8], freqs: &Scorer, keysize: usize) -> Vec<u8> {
    let mut groups: Vec<Vec<u8>> = Vec::with_capacity(keysize);
    for _ in 0..keysize {
        groups.push(Vec::new());
    }

    for (index, &byte) in ciphertext.iter().enumerate() {
        groups[index % keysize].push(byte);
    }

    let key = groups
        .iter()
        .map(|group| decrypt_single_byte_xor(group, freqs).2)
        .collect::<Vec<u8>>();

    repeating_key_xor(ciphertext, &key)
}

fn break_repeating_key_xor(ciphertext: &[u8], freqs: &Scorer) -> Vec<u8> {
    best_keysizes(ciphertext)
        .iter()
        .take(4)
        .map(|&keysize| break_repeating_key_xor_size(ciphertext, freqs, keysize))
        .max_by_key(|plaintext| freqs.score(plaintext))
        .unwrap() // For ciphertexts of size > 10, this won't panic, since we'll have 4 options to take.
                  // unsure about the behavior if we take less than 4..
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

        let (_, plaintext_guess, _) = decrypt_single_byte_xor(&ciphertext, &freqs);
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
        let plaintext_string = String::from_utf8_lossy(&plaintext_guess);

        let lyrics = "I'm back and I'm ringin' the bell \n\
        A rockin' on the mike while the fly girls yell \n\
        In ecstasy in the back of me \n\
        Well that's my DJ Deshay cuttin' all them Z's \n\
        Hittin' hard and the girlies goin' crazy \n\
        Vanilla's on the mike, man I'm not lazy. \n\
        \n\
        I'm lettin' my drug kick in \n\
        It controls my mouth and I begin \n\
        To just let it flow, let my concepts go \n\
        My posse's to the side yellin', Go Vanilla Go! \n\
        \n\
        Smooth 'cause that's the way I will be \n\
        And if you don't give a damn, then \n\
        Why you starin' at me \n\
        So get off 'cause I control the stage \n\
        There's no dissin' allowed \n\
        I'm in my own phase \n\
        The girlies sa y they love me and that is ok \n\
        And I can dance better than any kid n' play \n\
        \n\
        Stage 2 -- Yea the one ya' wanna listen to \n\
        It's off my head so let the beat play through \n\
        So I can funk it up and make it sound good \n\
        1-2-3 Yo -- Knock on some wood \n\
        For good luck, I like my rhymes atrocious \n\
        Supercalafragilisticexpialidocious \n\
        I'm an effect and that you can bet \n\
        I can take a fly girl and make her wet. \n\
        \n\
        I'm like Samson -- Samson to Delilah \n\
        There's no denyin', You can try to hang \n\
        But you'll keep tryin' to get my style \n\
        Over and over, practice makes perfect \n\
        But not if you're a loafer. \n\
        \n\
        You'll get nowhere, no place, no time, no girls \n\
        Soon -- Oh my God, homebody, you probably eat \n\
        Spaghetti with a spoon! Come on and say it! \n\
        \n\
        VIP. Vanilla Ice yep, yep, I'm comin' hard like a rhino \n\
        Intoxicating so you stagger like a wino \n\
        So punks stop trying and girl stop cryin' \n\
        Vanilla Ice is sellin' and you people are buyin' \n\
        'Cause why the freaks are jockin' like Crazy Glue \n\
        Movin' and groovin' trying to sing along \n\
        All through the ghetto groovin' this here song \n\
        Now you're amazed by the VIP posse. \n\
        \n\
        Steppin' so hard like a German Nazi \n\
        Startled by the bases hittin' ground \n\
        There's no trippin' on mine, I'm just gettin' down \n\
        Sparkamatic, I'm hangin' tight like a fanatic \n\
        You trapped me once and I thought that \n\
        You might have it \n\
        So step down and lend me your ear \n\
        '89 in my time! You, '90 is my year. \n\
        \n\
        You're weakenin' fast, YO! and I can tell it \n\
        Your body's gettin' hot, so, so I can smell it \n\
        So don't be mad and don't be sad \n\
        'Cause the lyrics belong to ICE, You can call me Dad \n\
        You're pitchin' a fit, so step back and endure \n\
        Let the witch doctor, Ice, do the dance to cure \n\
        So come up close and don't be square \n\
        You wanna battle me -- Anytime, anywhere \n\
        \n\
        You thought that I was weak, Boy, you're dead wrong \n\
        So come on, everybody and sing this song \n\
        \n\
        Say -- Play that funky music Say, go white boy, go white boy go \n\
        play that funky music Go white boy, go white boy, go \n\
        Lay down and boogie and play that funky music till you die. \n\
        \n\
        Play that funky music Come on, Come on, let me hear \n\
        Play that funky music white boy you say it, say it \n\
        Play that funky music A little louder now \n\
        Play that funky music, white boy Come on, Come on, Come on \n\
        Play that funky music \n";

        assert_eq!(plaintext_string, lyrics)
    }
}
