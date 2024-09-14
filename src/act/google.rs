use crate::prelude::*;
use std::env;

#[async_trait::async_trait]
pub trait GoogleOidcClientAct {
    async fn take(
        &self,
        server_addr: String,
        client_id: String,
        client_secret: String,
        callback_path: String
    ) -> anyhow::Result<CoreClient> {

        let client_id = ClientId::new(client_id);
        let client_secret = ClientSecret::new(client_secret);

        let issuer_url = IssuerUrl::new("https://accounts.google.com".to_string())
            .map_err(|e| e)?;

        let redirect_url = RedirectUrl::new(format!("{}{}", server_addr, callback_path))
            .map_err(|e| e)?;

        let provider_metadata = CoreProviderMetadata::discover_async(issuer_url, async_http_client)
            .await
            .map_err(|e| e)?;

        Ok(CoreClient::from_provider_metadata(
            provider_metadata,
            client_id,
            Some(client_secret),
        ).set_redirect_uri(redirect_url))
    }
}