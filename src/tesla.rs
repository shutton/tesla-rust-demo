#![allow(dead_code)]

use serde::{Deserialize, Serialize};

//
// These are well-known values that everyone uses
//
// https://tesla-api.timdorr.com/api-basics/authentication
//
const CLIENT_ID: &str = "81527cff06843c8634fdc09e8ac0abefb46ac849f38fe1e431c2ef2106796384";
const CLIENT_SECRET: &str = "c7257eb71a564034f9419ee651c7d0e5f7aa6bfbd18bafb5c5c033b093bb2fa3";

#[derive(Deserialize, Serialize, Debug)]
pub struct AuthTokenReq {
    grant_type: String,
    client_id: String,
    client_secret: String,
    email: String,
    password: String,
}

impl AuthTokenReq {
    pub fn new(email: &str, password: &str) -> Self {
        AuthTokenReq {
            grant_type: "password".to_owned(),
            client_id: CLIENT_ID.to_owned(),
            client_secret: CLIENT_SECRET.to_owned(),
            email: email.to_string(),
            password: password.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn new_auth_token() {
        let token = super::AuthTokenReq::new("jschmoe@example.com", "xyzzy");
        eprintln!("token = {:?}", token);
        let token_json = serde_json::to_string(&token).expect("encode json");
        eprintln!("token_json = {}", token_json);
    }
}
