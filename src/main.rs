use aes_gcm::{
    aead::{ Aead, AeadCore, KeyInit, OsRng },
    Aes256Gcm, Key, Nonce
};

fn main() {
    let plaintext = "This is a test string".to_string();
    let key_str = "thiskeystrmustbe32charlongtowork".to_string();

    let encryped_data = encrypt(key_str.clone(), plaintext);

    println!("Encrypted data: {:?}", encryped_data.clone());

    let decrypted_data = decrypt(key_str, encryped_data);

    println!("Decrypted data: {:?}", decrypted_data);
}


fn encrypt(key_str: String, plaintext: String) -> String {
    let key = Key::<Aes256Gcm>::from_slice(key_str.as_bytes());
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let cipher = Aes256Gcm::new(key);
    let cipher_text = cipher.encrypt(&nonce, plaintext.as_bytes())
        .expect("failed to encrypt");

    let mut encryped_data: Vec<u8> = nonce.to_vec();
    encryped_data.extend_from_slice(&cipher_text);

    return hex::encode(encryped_data);
}

fn decrypt(key_str: String, encrypted_data: String) -> String {
    let encrypted_data = hex::decode(encrypted_data)
        .expect("failed to decode hex string into vec");

    let key = Key::<Aes256Gcm>::from_slice(key_str.as_bytes());

    let (nonce_arr, ciphered_data) = encrypted_data.split_at(12);
    let nonce = Nonce::from_slice(nonce_arr);

    let cipher = Aes256Gcm::new(key);

    let plaintext = cipher.decrypt(nonce, ciphered_data)
        .expect("failed to decrypt data");

    String::from_utf8(plaintext)
        .expect("failed to convert vector of bytes to string")
}


