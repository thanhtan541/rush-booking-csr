use once_cell::sync::OnceCell;
use reqwest::header::{CONTENT_TYPE, ACCEPT};

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct ResponseData<T> {
    pub data: Vec<T>,
    pub message: String,
    pub code: u16,
}

#[derive(Debug)]
pub struct ApiAdapter {
    pub address: String,
    pub client: reqwest::Client,
}

/// A shared instacen of api adapter
pub static API_ADAPTER_INSTANCE: OnceCell<ApiAdapter> = OnceCell::new();

impl ApiAdapter {
    /// Call to get instance of api adapter
    pub fn global() -> &'static ApiAdapter {
        API_ADAPTER_INSTANCE
            .get()
            .expect("Api Adapter is not initialized")
    }

    pub async fn post_login(&self, body: &serde_json::Value) -> reqwest::Response {
        self.client
            .post(&format!("{}/login", &self.address))
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json")
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_users(&self) -> reqwest::Response {
        self.client
            .get(&format!("{}/admin/users", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_rooms(&self) -> reqwest::Response {
        self.client
            .get(&format!("{}/admin/rooms", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }
}
