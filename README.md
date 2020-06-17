Rust solutions to the Cryptopals (formerly Matasano) Challenges. The challenges
start from basic cryptography and gradually progress to more advanced stuff. You
can find more information about the challenges at the [Cryptopals
website](cryptopals.com). Currently I'm working on Set 1 and getting more
familiar with the Rust language and tooling.

## Set 1
Key functionalities I see for this set are:
- conversion to/from hex and base64 (use an existing crate?)
- xor two iterables and return a buffer
    - xor one iterable with a repeated byte
    - xor one iterable with a cycled list of bytes
- object that encapsulates rule scoring
    - iterate over several scorables and return the best N
    - `Vec<u8>` to `(Score, Vec<u8>)`
- scorable object that can be incrementally added to (?)
- hamming distance
    - weighted hamming distance

### Challenge 1
hex->bytes
bytes->base64

### Challenge 2
hex->bytes
xor two buffers
bytes->hex

### Challenge 3
hex->bytes
xor buffer with byte for each possible byte
iterate over each buffer and return the best N

### Challenge 4
hex->bytes
xor buffer with byte for each possible byte and line
iterate over each buffer and return the best N

### Challenge 5
string->bytes
xor two buffers
bytes->hex

### Challenge 6

