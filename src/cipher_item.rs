use base64::{engine::general_purpose, Engine};
// use hex;
use serde::{Deserialize, Serialize};
use std::error::Error;

//use aes_gcm::aead::consts::U12;
use aes_gcm::aead::{Aead, Payload};
use aes_gcm::{
    aead::{AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};

type Keylen32u8 = [u8; 32];
pub type AesGcmKey<'a> = &'a Keylen32u8;

#[derive(Serialize, Deserialize, Debug)]
struct CipherItem<T> {
    c: T,
    n: T,
    a: T,
}

impl CipherItem<String> {}

impl CipherItem<String> {
    fn to_json(&self) -> Result<String, Box<dyn Error>> {
        let json_string = serde_json::to_string(&self)?;
        Ok(json_string)
    }
    pub fn decrypt(&self, key: Key<Aes256Gcm>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let nonce_bytes = general_purpose::STANDARD.decode(&self.n)?;
        let nonce = Nonce::from_slice(&nonce_bytes);
        let ciphertext = general_purpose::STANDARD.decode(&self.c)?;
        let associated_data = self.a.as_bytes();
        let cipher = Aes256Gcm::new(&key);
        let decrypted_bytes = cipher
            .decrypt(
                nonce,
                Payload {
                    msg: &ciphertext,
                    aad: associated_data,
                },
            )
            .map_err(|e| format!("Decryption failed: {:?}", e))?;
        Ok(decrypted_bytes)
    }
}

pub fn aes_gcm_key_from_string_literal(data: &[u8]) -> Keylen32u8 {
    assert_eq!(data.len(), 32, "Data must be exactly 32 bytes long");
    let mut key = [0u8; 32];
    key.copy_from_slice(data);
    key
}

pub fn encrypt_bytes(
    key: &[u8; 32],
    plaintext: &[u8],
    associated_data: &[u8],
) -> CipherItem<String> {
    let key = <Key<Aes256Gcm>>::from(aes_gcm_key_from_string_literal(key));
    let key = <Key<Aes256Gcm>>::from(key);

    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per encryption

    let encrypted_bytes = cipher
        .encrypt(
            &nonce,
            Payload {
                msg: plaintext,
                aad: associated_data,
            },
        )
        .expect("Encryption failed");

    let ciphertext_b64 = general_purpose::STANDARD.encode(encrypted_bytes);
    let nonce_b64 = general_purpose::STANDARD.encode(nonce);

    CipherItem {
        c: ciphertext_b64,
        n: nonce_b64,
        a: general_purpose::STANDARD.encode(associated_data),
    }
}

pub fn client_encrypt_and_server_decrypt_test() -> Result<(), Box<dyn std::error::Error>> {
    println!("running test client_encrypt_and_server_decrypt_test");
    let test_key = b"0123456789abcdef0123456789abcdef";
    let associated_data = b"";

    let json_str = encrypt_payload(
        test_key,
        RUN_BASH_SCRIPT_PAYLOAD01.as_bytes(),
        associated_data,
    )?;

    decrypt_payload(json_str, test_key)?;

    Ok(())
}

fn encrypt_payload(
    key: &[u8; 32],
    plain_text: &[u8],
    associated_data: &[u8],
) -> Result<String, Box<dyn std::error::Error>> {
    println!("Encrypting {} bytes now", plain_text.len());

    let associated_data = b"";
    let ci = encrypt_bytes(
        key,
        plain_text, //RUN_BASH_SCRIPT_PAYLOAD01.as_bytes(),
        associated_data,
    );
    let json_str = ci.to_json()?;
    println!("encrypted request data ");
    println!("{}", json_str);
    Ok(json_str)
}

fn decrypt_payload(
    json_str: String,
    test_key: &[u8; 32],
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Decrypting {} bytes", json_str.len());
    let ci: CipherItem<String> = serde_json::from_str(json_str.as_str())?;
    let key = <Key<Aes256Gcm>>::from(aes_gcm_key_from_string_literal(test_key));
    let d = ci.decrypt(key)?;
    println!("{}", String::from_utf8(d)?);
    Ok(())
}

const RUN_BASH_SCRIPT_PAYLOAD01: &str = r#"{
    "request_params": {
        "request_type": "command_execution",
        "command_params": {
            "command_type": "run_bash_script",
            "run_mode": "async",
            "command_data": {
                "run_bash_script_data": {
                    "script_data": "ZWNobyAnSGVsbG8sIFdvcmxkIScK",
                    "script_data_type": "bash_script_b64_utf8"
                }
            },
            "command_progress_info_params": {
                "stream_progress_type": "realtime"
            }
        }
    }
}"#;
//
//
