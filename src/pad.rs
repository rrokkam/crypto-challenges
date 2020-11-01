pub fn pkcs7(block: &mut Vec<u8>, blocksize: u8) {
    let padding = blocksize - (block.len() as u8 % blocksize);
    block.extend(std::iter::repeat(padding).take(padding as usize));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pkcs7() {
        let mut block = Vec::from("YELLOW SUBMARINE");
        pkcs7(&mut block, 20);
        assert_eq!(
            block,
            Vec::from("YELLOW SUBMARINE\x04\x04\x04\x04")
        );
    }
}
