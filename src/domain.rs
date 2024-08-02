#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct JwtResponse {
    token_type: String,
    access_token: String,
    refresh_token: String,
    id_token: String,
    expires_in: u16,
}
