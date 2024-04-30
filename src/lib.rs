// Copyright (c) 2024 Venkatesh Omkaram

mod operations;

use crate::operations::{decrypt_files, encrypt_files};
use aes_gcm::{Aes256Gcm, Key};
use base64::{engine::general_purpose, Engine as _};
use pbkdf2::pbkdf2_hmac_array;
use sha2::Sha256;
use wasm_bindgen::prelude::*;
use web_sys;
use zeroize::Zeroize;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

// Program execution begins here
#[wasm_bindgen]
pub fn run(
    input: &str,
    password: &str,
    salt: &str,
    operation: &str,
    mode: &str,
    iterations: u32,
) -> String {
    // web_sys::console::log_1(&input.into());
    // web_sys::console::log_1(&password.into());
    // web_sys::console::log_1(&salt.into());
    // web_sys::console::log_1(&operation.into());
    // web_sys::console::log_1(&mode.into());
    // web_sys::console::log_1(&iterations.into());
    web_sys::console::log_1(&"Generating a key based on PBKDF2 HMAC (SHA256) function ...".into());

    // Using the PBKDF2 SHA256 function generate a 32 byte key array based on the password and the salt provided as bytes, and the number of iterations
    let mut key = pbkdf2_hmac_array::<Sha256, 32>(password.as_bytes(), salt.as_bytes(), iterations);

    // Generate a Key of type Generic Array which can be used by the core AES GCM module from the 32 byte key array
    let mut key_gen = Key::<Aes256Gcm>::clone_from_slice(key.as_slice());

    web_sys::console::log_1(&"The key is generated".into());

    let output = match operation {
        "Encrypt" => {
            web_sys::console::log_1(&"Encrypting now".into());

            if let Some(output) = encrypt_files(mode, input.to_string(), key_gen) {
                let base64 = general_purpose::STANDARD.encode(output);
                //alert(&format!("Encrypted data: {}", base64));
                base64
            } else {
                "Failed to encrypt".to_owned()
            }
        }
        "Decrypt" => {
            let encrypted_bytes = general_purpose::STANDARD.decode(input).unwrap();
            web_sys::console::log_1(&"Decrypting now".into());

            if let Some(output) = decrypt_files(mode, encrypted_bytes, key_gen) {
                // alert(&format!(
                //     "Decrypted data: {}",
                //     String::from_utf8_lossy(&output)
                // ));
                String::from_utf8_lossy(&output).to_string()
            } else {
                "Failed to decrypt".to_owned()
            }
        }
        &_ => todo!(),
    };

    key_gen.zeroize();
    key.zeroize();

    //     key_gen.zeroize();
    // }
    assert_eq!(key_gen.as_slice(), &[0; 32]);
    assert_eq!(key.as_slice(), &[0; 32]);

    web_sys::console::log_1(&"Cleared the key".into());

    output
}
