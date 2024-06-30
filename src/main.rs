use aes_gcm::{
    aead::{
        Aead,
        AeadCore,
        KeyInit,
        OsRng
    },
    Aes256Gcm, Key, Nonce
};
use sha2::{
    Digest, 
    Sha256,
    digest::generic_array::typenum::U32,
    digest::generic_array::GenericArray
};

use std::io;
use std::fs;
use std::env;
use std::path::Path;

#[derive(Debug)]
enum Control {
    Encrypt,
    Decrypt
}

#[derive(Debug)]
struct Options {
    control: Control,
    path:    String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opt: Options = { 
        Options {
            control: Control::Decrypt,
            path: "".to_string()
        }
    };
    for (i, arg) in args.iter().enumerate() {
        if arg == "-e" {
            opt.control = Control::Encrypt;
        }
        if arg == "-d" {
            opt.control = Control::Decrypt;
        }
        if arg == "-p" {
            opt.path = args[i + 1].clone();
        }
    }

    let pth      = Path::new(&opt.path);
    let password = "Password".to_string();
    let key      = gen_key(password.as_bytes());

    match opt.control {
        Control::Encrypt => {
            encrypt(&key, pth)
                .expect("encrypt failed.");
        }
        Control::Decrypt => {
            decrypt(&key, pth)
                .expect("decrypt failed.");
        }
    }
}


fn encrypt(key: &GenericArray<u8, U32>, dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path  = entry.path();

            encrypt(key, &path)
                .expect("Encrypt failed.");
        }

    } else {
        let data: Vec<u8> = fs::read(dir)
            .expect("Unable to read file.");

        let key = Key::<Aes256Gcm>::from_slice(key);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let cipher = Aes256Gcm::new(key);
        let cipher_text = cipher.encrypt(&nonce, &*data)
            .expect("failed to encrypt");
    
        let mut encryped_data: Vec<u8> = nonce.to_vec();
        encryped_data.extend_from_slice(&cipher_text);
    
        fs::write(dir, hex::encode(encryped_data))
            .expect("Encrypt: Unable to write data.");
    }

    return Ok(());
}


fn decrypt(key: &GenericArray<u8, U32>, dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path  = entry.path();

            decrypt(key, &path)
                .expect("Decrypt failed.")
        }

    } else {
        let encrypted_data = hex::decode(String::from_utf8(fs::read(dir)
            .expect("Unable to read file."))
            .expect("Unable to convert to String from utf8"))
            .expect("Failed to decode hex string into Vec<u8>.");

        let key = Key::<Aes256Gcm>::from_slice(key);
        let (nonce_arr, ciphered_data) = encrypted_data.split_at(12);
        let nonce = Nonce::from_slice(nonce_arr);
        let cipher = Aes256Gcm::new(key);
        let decrypted_data = cipher.decrypt(nonce, ciphered_data)
            .expect("failed to decrypt data");
    
        fs::write(dir, decrypted_data)
            .expect("Decrypt: Unable to write data.");
    }

    return Ok(());
}


fn gen_key(password: &[u8]) -> GenericArray<u8, U32> {
    let mut hash = Sha256::new();
    hash.update(password);

    return hash.finalize();
}