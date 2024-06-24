use aes_gcm::{
    aead::{ Aead, AeadCore, KeyInit, OsRng },
    Aes256Gcm, Key, Nonce
};

use std::fs;
use std::env;
use std::path::Path;

enum Control {
    FileEnc,
    FileDec,
    DirEnc,
    DirDec
}

fn main() {
    let pth     = Path::new("./test/heart.png");
    let key_str = "thiskeystrmustbe32charlongtowork".to_string();

    let con = Control::FileDec;

    match con {
        Control::FileEnc => {
            let data = fs::read(pth)
                .expect("Unable to read file.");
            let data = encrypt(&key_str, data);
    
            fs::write(pth, data)
                .expect("Unable to write encrypted data to file.");
        }
        Control::FileDec => {
            let data = String::from_utf8(fs::read(pth)
                            .expect("Unable to read file."))
                            .expect("Unable to convert data to String.");
            
            let data = decrypt(key_str, data);
            fs::write(pth, data)
                .expect("Unable to write decrypted data to file.");
        }
        _ => panic!("hsdf")
    }

    // if pth.is_file() {
    //     let data = fs::read(pth)
    //         .expect("Unable to read file.");
    //     let enc = encrypt(&key_str, data);

    //     fs::write(pth, enc)
    //         .expect("Unable to write file.");
    // }
}


fn encrypt(key_str: &String, data: Vec<u8>) -> String {
    let key = Key::<Aes256Gcm>::from_slice(key_str.as_bytes());
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let cipher = Aes256Gcm::new(key);
    let cipher_text = cipher.encrypt(&nonce, &*data)
        .expect("failed to encrypt");

    let mut encryped_data: Vec<u8> = nonce.to_vec();
    encryped_data.extend_from_slice(&cipher_text);

    return hex::encode(encryped_data);
}

fn decrypt(key_str: String, encrypted_data: String) -> Vec<u8> {
    let encrypted_data = hex::decode(encrypted_data)
        .expect("failed to decode hex string into vec");

    let key = Key::<Aes256Gcm>::from_slice(key_str.as_bytes());
    let (nonce_arr, ciphered_data) = encrypted_data.split_at(12);
    let nonce = Nonce::from_slice(nonce_arr);
    let cipher = Aes256Gcm::new(key);
    let decrypted_data = cipher.decrypt(nonce, ciphered_data)
        .expect("failed to decrypt data");

    return decrypted_data;
}