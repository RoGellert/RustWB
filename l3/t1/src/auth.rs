use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,    // Expiry time of the token
    pub iat: usize,    // Issued at time of the token
    pub login: String, // Email associated with the token
}

// Define a structure for holding sign-in data
#[derive(Deserialize)]
pub struct SignInData {
    pub login: String,    // Email entered during sign-in
    pub password: String, // Password entered during sign-in
}
