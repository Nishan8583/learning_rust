use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;

pub fn create_signed_key(username: String) -> Result<String, String> {
    let mut key = Hmac::new_from_slice(b"secret");
    if let Err(err) = key {
        println!("Error while crafting a hmac");
        return Err(err.to_string());
    };

    let key: Hmac<Sha256> = key.unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("user", username);
    let token = match claims.sign_with_key(&key) {
        Ok(v) => Ok(v),
        Err(err) => Err(err.to_string()),
    };

    println!("token {:?}", token);
    return token;
}
