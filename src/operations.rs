// Copyright (c) 2024 Venkatesh Omkaram

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    aes::cipher::{
        crypto_common::generic_array::GenericArray,
        typenum::{UInt, UTerm, B0, B1, /*U32, U16*/ U12},
    },
    Aes256Gcm, //, Nonce, Key // Or `Aes128Gcm`
};
use byte_aes::Aes256Cryptor;


/* What do the above imports do?
-----------------------
aes_gcm - Has the functions which helps to encrypt and decrypt the files for GCM mode
byte_aes - Has the functions which helps to encrypt and decrypt the files for ECB mode


Read the Cargo.toml and Attributions to see which versions and the Authors who made these crates
*/


pub fn encrypt_files(
    mode: &str,
    data: String,
    key_gen: GenericArray<u8, UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>>,
) -> Option<Vec<u8>> {
    match mode {
        "ECB" => {
            //Create Aes256Cryptor Object
            let encrypt_obj = Aes256Cryptor::new({
                let mut key = [0u8; 32];
                key.copy_from_slice(&key_gen);
                key
            });

            // Call the encrypt method on the Aes256Cryptor Object
            let encrypted_bytes = encrypt_obj.encrypt(data); // vec<u8>
            Some(encrypted_bytes)
        }

        "GCM" => {
            // Extract the 32 byte key from the Vec and construct a Aes256Gcm object
            let cipher: aes_gcm::AesGcm<aes_gcm::aes::Aes256, _, _> = Aes256Gcm::new(&key_gen);

            // Generate a random 12 byte Nonce
            let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
            web_sys::console::log_1(&"Nonce generated".into());

            // Call the encrypt method on the Aes256Gcm object and see if was successful
            match cipher.encrypt(&nonce, data.as_ref()) {
                Ok(encrypted_bytes) => {
                    web_sys::console::log_1(&"Completed the encryption".into());
                    Some([encrypted_bytes, nonce.to_vec()].concat())
                }
                Err(_) => {
                    web_sys::console::log_1(&"Failed the decryption".into());
                    None
                }
            } // vec<u8>
        }
        &_ => todo!(),
    }
}

pub fn decrypt_files(
    mode: &str,
    data: Vec<u8>,
    key_gen: GenericArray<u8, UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>>,
) -> Option<Vec<u8>> {
    if let "GCM" = mode {
        let mut file_data: Vec<u8> = data;

        //let nonce = file_data.clone().into_iter().rev().take(12).rev().collect::<Vec<u8>>();
        // Onc we have file_data to be decrypted we need to extract the Nonce which we used to Encrypt.
        // The None is part of the file. It is the last 12 bytes in the encrypted file. We need to know where to Split
        // saturating_sub helps to find the position at which the split needs to happen which varies based on the file_data length.
        let final_length = file_data.len().saturating_sub(12);

        // Splits at the final_length. This length is the end off the actual file content and the start of the Nonce. It then returns the Nonce in a new Vec<u8>
        let nonce = file_data.split_off(final_length);
        web_sys::console::log_1(&"Nonce retrieved".into());

        let cipher: aes_gcm::AesGcm<aes_gcm::aes::Aes256, _, _> = Aes256Gcm::new(&key_gen);

        // We need the Nonce to be of type GenericArray to be used by the decrypt function
        let nonce = GenericArray::<u8, U12>::from_slice(nonce.as_ref());

        // File is decrypted here
        match cipher.decrypt(nonce, file_data.as_ref()) {
            Ok(res) => {
                web_sys::console::log_1(&"Completed the decryption".into());
                Some(res)
            }
            Err(_) => {
                web_sys::console::log_1(&"Failed the decryption".into());
                None
            }
        }
    } else {
        //Create Aes256Cryptor Object
        let decrypt_obj = Aes256Cryptor::new({
            let mut key = [0u8; 32];
            key.copy_from_slice(&key_gen);
            key
        });

        let decrypted_result = decrypt_obj.decrypt(data);

        if let Ok(decrypted_bytes) = decrypted_result {
            Some(decrypted_bytes)
        } else {
            None
        }
    }
}
