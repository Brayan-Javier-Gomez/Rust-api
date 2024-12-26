// use rand_core::OsRng;
// use x25519_dalek::{EphemeralSecret, PublicKey};
use openssl::rsa::Rsa;
use base64::{engine::general_purpose, Engine};

pub fn generate_keys_open_ssl() -> Result<(String,String), String> {
    // Generar clave privada
    let private_key = Rsa::generate(2048).expect("falló al generar la clave privada");
    let public_key_der = private_key.public_key_to_der().map_err(|e| e.to_string())?;
    let public_key = Rsa::public_key_from_der(&public_key_der).map_err(|e| e.to_string())?;
    let public_key_base64 = general_purpose::STANDARD.encode(&public_key.public_key_to_der().map_err(|e| e.to_string())?);
    let private_key_base64 = general_purpose::STANDARD.encode(&private_key.private_key_to_der().map_err(|e| e.to_string())?);
    Ok((private_key_base64, public_key_base64))
}

// pub fn generate_keypair() -> (EphemeralSecret, PublicKey) {
//     let key_secret = EphemeralSecret::random_from_rng(OsRng);
//     let key_public = PublicKey::from(&key_secret);
//     (key_secret, key_public)
// }

// pub fn derive_shared_key(
//     my_private_key: EphemeralSecret,
//     their_public_key: &PublicKey,
// ) -> Result<[u8; 32], String> {
//     Ok(my_private_key.diffie_hellman(their_public_key).to_bytes())
// }

// pub fn serialize_keys(key_secret: EphemeralSecret, key_public: PublicKey) -> (String, String) {
//     let public_key_bytes = key_public.as_bytes();
//     let private_key_bytes = key_secret.diffie_hellman(&key_public);
//     let test = "tes";
//     println!("Error de base de datos: {:?}", test);
//     (private_key_bytes, public_key_bytes)
// }
