pub fn encrypt(key: u8, plain_text: String) {
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

pub fn decrypt(key: u8, cipher_text: String) {
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

pub fn brute_force(dict_path: String, cipher_text: String) {
    println!("Dictionary Path: {dict_path}, Cipher Text: {cipher_text}");
}