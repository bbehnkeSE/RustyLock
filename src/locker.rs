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
use std::path::PathBuf;

pub fn encrypt(key: &GenericArray<u8, U32>, path: &PathBuf) -> Result<(), io::Error> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path  = entry.path();

            encrypt(key, &path)?;
        }
    } else {
        let data = fs::read(path)?;
        let encrypted_data = encrypt_file(&data, key);

        match encrypted_data {
            Ok(encrypted_data) => fs::write(path, encrypted_data)?,
            Err(e) => fs::write("./rusty_error.log", e.to_string() + "\nEncryption failed.")?
        };
    }
    Ok(())
}

fn encrypt_file(data: &Vec<u8>, key: &GenericArray<u8, U32>) -> Result<Vec<u8>, aes_gcm::Error> {
    let nonce  = Aes256Gcm::generate_nonce(&mut OsRng);
    let cipher = Aes256Gcm::new(key);
    let cipher = cipher.encrypt(&nonce, &**data)?;
    let mut encrypted_data: Vec<u8> = nonce.to_vec();
    encrypted_data.extend_from_slice(&cipher);

    Ok(encrypted_data)
}


pub fn decrypt(key: &GenericArray<u8, U32>, path: &PathBuf) -> Result<(), io::Error> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path  = entry.path();

            decrypt(key, &path)?;
        }

    } else {
        let encrypted_data = fs::read(path)?;
        let data = decrypt_file(encrypted_data, key);

        match data {
            Ok(data) => fs::write(path, data)?,
            Err(e)   => fs::write("./rusty_error.log", e.to_string()
                + "\nDecryption Failed."
                + "\n\nLikely entered an incorrect password or tried to decrypt "
                + "a file that wasn't encrypted.")?,
        };
    }

    Ok(())
}

fn decrypt_file(encrypted_data: Vec<u8>, key: &GenericArray<u8, U32>) -> Result<Vec<u8>, aes_gcm::Error> {
    let key    = Key::<Aes256Gcm>::from_slice(key);
    let (nonce_slice, cipher_data) = encrypted_data.split_at(12);
    let nonce  = Nonce::from_slice(nonce_slice);
    let cipher = Aes256Gcm::new(key);
    let data   = cipher.decrypt(nonce, cipher_data)?;

    Ok(data)
}


pub fn gen_key(password: &[u8]) -> GenericArray<u8, U32> {
    let mut hash = Sha256::new();
    hash.update(password);

    return hash.finalize();
}