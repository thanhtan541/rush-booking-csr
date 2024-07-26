use once_cell::sync::OnceCell;

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
