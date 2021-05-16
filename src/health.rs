use std::collections::HashMap;

use crate::Opa;

impl Opa {
    /// Health Check
    pub async fn health(&self) {
        let resp = reqwest::get(format!("{}/health", self.ip())).await.unwrap();
        // .json()
        // .await
        // .unwrap();

        println!("{:#?}", resp);
    }
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn test_health() {
        let opa = crate::Opa::new();
        opa.health().await
    }
}
