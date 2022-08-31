use docopt::Docopt;
use serde::Deserialize;

const USAGE: &'static str = "
Caesar Cipher Tool.

Usage:
    caesar-cipher encrypt [--key <int>] (--plaintext <string>)
    caesar-cipher decrypt [--key <int>] (--ciphertext <string>)
    caesar-cipher (--help)

Options:
    -h, --help                          Show this help message and exit.
    -k <int>, --key <int>               Rotation key value as an integer between 1 and 25 [default: 13].
    -p <string>, --plaintext <string>   Plaintext to be encrypted. Must contain only English letters and spaces.
    -c <string>, --ciphertext <string>  Ciphertext to be decrypted. Must contain only English letters and spaces.
";

#[derive(Debug, Deserialize)]
struct Args {
    cmd_encrypt: bool,
    cmd_decrypt: bool,
    flag_key: usize,
    flag_plaintext: String,
    flag_ciphertext: String,
}

fn encrypt(key: u8, plain_text: String) {
    let mut cipher_text: Vec<char> = vec![' '; plain_text.len()];

    for (index, letter) in plain_text.chars().enumerate() {
        cipher_text[index] = if 'A' <= letter && letter <= 'Z' {
            (((((letter as u8) - ('A' as u8)) + key) % 26) + ('A' as u8)) as char
        } else if 'a' <= letter && letter <= 'z' {
            (((((letter as u8) - ('a' as u8)) + key) % 26) + ('a' as u8)) as char
        } else {
            letter
        };
    }

    let cipher_text: String = cipher_text.into_iter().collect();
    println!("{cipher_text}");
}

fn decrypt(key: u8, cipher_text: String) {
    let mut plain_text: Vec<char> = vec![' '; cipher_text.len()];

    for (index, letter) in cipher_text.chars().enumerate() {
        plain_text[index] = if 'A' <= letter && letter <= 'Z' {
            (((((letter as u8) - ('A' as u8)) + (26 - key)) % 26) + ('A' as u8)) as char
        } else if 'a' <= letter && letter <= 'z' {
            (((((letter as u8) - ('a' as u8)) + (26 - key)) % 26) + ('a' as u8)) as char
        } else {
            letter
        };
    }

    let plain_text: String = plain_text.into_iter().collect();
    println!("{plain_text}");
}
fn validate_text(string: &String) {
    if !string.chars().all( |c| c.is_ascii_alphabetic() || c.is_ascii_whitespace()){
        panic!("The text you provided was not valid. Refer to the documentation for usage details.")
    }
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    if args.cmd_encrypt {
        validate_text(&args.flag_plaintext);
        encrypt(args.flag_key as u8, args.flag_plaintext);
    } else if args.cmd_decrypt {
        validate_text(&args.flag_ciphertext);
        decrypt(args.flag_key as u8, args.flag_ciphertext);
    } else {
        unreachable!();
    }


}
