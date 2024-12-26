// use aes_gcm::aead::{Aead, KeyInit, OsRng};
// use aes_gcm::{Aes256Gcm, Key, Nonce};
// use rand::RngCore;
// use generic_array::GenericArray;
// use openssl::pkey::Private;
use base64::Engine;
use base64::engine::general_purpose;
use openssl::rsa::{Padding, Rsa};
use openssl::pkey::Public;

pub fn encrypt_message_with_openssl(public_key:Rsa<Public>, plaintext:&str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // let public_key_decoded = general_purpose::STANDARD.decode(public_key).expect("Error al decodificar la clave pública");
    // let public_key_decripted = rsa::Rsa::public_key_from_der(&public_key).map_err(|e| format!("Error al crear la clave pública desde DER: {}", e))?;
    let mut encrypted = vec![0u8; public_key.size() as usize];
    public_key.public_encrypt(plaintext.as_bytes(), &mut encrypted, Padding::PKCS1)?;
    Ok(encrypted)
}

pub fn encode_base64(encrypted_message: &Vec<u8>) -> String {
    general_purpose::STANDARD.encode(encrypted_message)
}
// pub fn decrypt_message_with_openssl(encrypted_message: &[u8], private_key_der: Rsa<Private>){   
//     let mut decrypted = vec![0u8; private_key_der.size() as usize];
//     private_key_der.private_decrypt(encrypted_message, &mut decrypted, Padding::PKCS1).unwrap();
//     println!("Mensaje descifrado: {:?}", String::from_utf8(decrypted).unwrap());
// }

// pub fn decode_base64(base64_message: String) -> Vec<u8> {
//     general_purpose::STANDARD.decode(base64_message).expect("Error al intentar decodificar el mensaje cifrado")
// }

// pub fn encrypt_message(
//     shared_key: &[u8; 32], // Tamaño de clave especificado
//     plaintext: &str,
// ) -> Result<(String, String), String> {
//     // Convertir la clave compartida en un objeto `Key`
//     let key = Key::<Aes256Gcm>::from_slice(shared_key); // Especificar el tipo explícitamente
//     let cipher = Aes256Gcm::new(key);

//     // Generar un nonce de 12 bytes aleatorios
//     let mut nonce_bytes = [0u8; 12];
//     OsRng.fill_bytes(&mut nonce_bytes);
//     let nonce = Nonce::from_slice(&nonce_bytes);

//     // Intentar cifrar el mensaje
//     match cipher.encrypt(nonce, plaintext.as_bytes()) {
//         Ok(ciphertext) => {
//             let ciphertext_base64 = general_purpose::STANDARD.encode(&ciphertext);
//             let nonce_base64 = general_purpose::STANDARD.encode(&nonce);
//             Ok((ciphertext_base64, nonce_base64))
//         },
//         Err(err) => Err(format!("Error cifrando mensaje: {:?}", err)),
//     }
// }


// pub fn decrypt_message(
//     shared_key: &[u8; 32],
//     ciphertext: String,
//     nonce: String,
// ) -> Result<String, String> {


//     let ciphertext_decoded = match general_purpose::STANDARD.decode(ciphertext) {
//         Ok(data) => data,
//         Err(_) => return Err("Error al decodificar el ciphertext de base64.".to_string()),
//     };
//     let nonce_decoded = match general_purpose::STANDARD.decode(nonce) {
//         Ok(data) => data,
//         Err(_) => return Err("Error al decodificar el ciphertext de base64.".to_string()),
//     };
    
//     if nonce_decoded.len() != 12 {
//         return Err("Nonce debe tener exactamente 12 bytes.".to_string());
//     }
//     let nonce = GenericArray::from_slice(&nonce_decoded);
//     let key = Key::<Aes256Gcm>::from_slice(shared_key);
//     let cipher = Aes256Gcm::new(key);

//     match cipher.decrypt(nonce, ciphertext_decoded.as_ref()) {
//         Ok(plaintext) => {
//             // Convertir el mensaje descifrado de bytes a String y devolverlo
//             match String::from_utf8(plaintext) {
//                 Ok(plaintext_str) => Ok(plaintext_str),
//                 Err(_) => Err("Error al convertir el mensaje descifrado a UTF-8.".to_string()),
//             }
//         },
//         Err(err) => Err(format!("Error descifrando mensaje: {:?}", err)),
//     }
// }