use std::error::Error;

use hyper::{Client, client::HttpConnector, body::HttpBody};
use hyper_tls::HttpsConnector;


pub struct RequestManager {
    client: Client<HttpsConnector<HttpConnector>>
}

impl RequestManager {
    pub fn new() -> Self {
        Self {
            client: Client::builder().build::<_, hyper::Body>(HttpsConnector::new())
        }
    }

    pub async fn make_get_request(&self, uri_with_params: &str) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>>{
        let mut resp = self.client.get(uri_with_params.parse()?).await?;
        let mut buffer = Vec::new();
        while let Some(chunk) = resp.body_mut().data().await {
            buffer.extend_from_slice(&chunk?);
        }

        Ok(buffer)
    }
}

impl Default for RequestManager {
    fn default() -> Self {
        Self::new()
    }
}