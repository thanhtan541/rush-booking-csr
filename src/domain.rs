use validator::validate_email;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct JwtResponse {
    token_type: String,
    access_token: String,
    refresh_token: String,
    id_token: String,
    expires_in: u16,
}

#[derive(Debug)]
pub struct CustomerEmail(String);

impl CustomerEmail {
    pub fn parse(s: String) -> Result<CustomerEmail, String> {
        // TODO: add validation!
        if validate_email(&s) {
            Ok(Self(s))
        } else {
            Err(format!("{} is not a valid customer email", s))
        }
    }
}

impl AsRef<str> for CustomerEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
