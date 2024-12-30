use reqwest::blocking::Client;
use crate::cipher_item::encrypt_payload;

pub fn post_http_request(url :&str, plain_text_payload : &str,
                         key : &[u8; 32],
                         associated_data : &[u8] ) -> reqwest::Result<reqwest::blocking::Response> {

    let encrypted_payload = encrypt_payload(key, plain_text_payload.as_bytes(), associated_data)
        .expect("Encryption failed");

    let client = Client::new();
    let formatted_body = to_json_literal_string(encrypted_payload.as_str()); //  format!("\"{}\"", encrypted_payload);

    client
        .post(url)
        .header("Content-Type", "application/json")
        .body(formatted_body) // Wrap in quotes to make it a JSON string
        .send()
}

pub fn to_json_literal_string(payload: &str) -> String {
    let escaped_payload = payload.replace("\"", "\\\"");
    format!("\"{}\"", escaped_payload)
}