use std::collections::HashMap;
use std::error::Error;

use serde::Deserialize;
use std::path::PathBuf;
use tonic::transport::{Channel, ClientTlsConfig, Endpoint, Identity, Uri};

#[derive(Deserialize)]
pub struct Credentials {
    client_x509_cert_url: String,
    // auth_provider_x509_cert_url: String,
    private_key_id: String,
    pub project_id: String,
    private_key: String,
}

impl Credentials {
    pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_str)
    }

    pub fn auto_acquire() -> Result<Self, Box<dyn Error>> {
        let path = std::env::var("GOOGLE_APPLICATION_CREDENTIALS").map(|env| PathBuf::from(env))?;
        Ok(std::fs::read_to_string(&path).map(|data| {
            Credentials::from_json(&data).expect("Unable to parse credentials in environment")
        })?)
    }
}

pub struct GrpcEndpoint {
    inner: Endpoint,
}

impl GrpcEndpoint {
    pub async fn new(credentials: &Credentials, domain_name: &str) -> Result<Self, Box<dyn Error>> {
        let identity = {
            let re: HashMap<String, String> = reqwest::get(&credentials.client_x509_cert_url)
                .await?
                .json::<HashMap<String, String>>()
                .await?;
            let key = re.get(&credentials.private_key_id).expect("No private key");

            Identity::from_pem(key.as_bytes(), &credentials.private_key.as_bytes())
        };

        let tls_config = ClientTlsConfig::new()
            .identity(identity)
            .domain_name(domain_name);

        let endpoint = Channel::builder(
            Uri::builder()
                .scheme("https")
                .authority(domain_name)
                .path_and_query("/")
                .build()
                .expect("Unable to build uri"),
        )
        .tls_config(tls_config);
        Ok(GrpcEndpoint { inner: endpoint })
    }
}

impl Into<Endpoint> for GrpcEndpoint {
    fn into(self) -> Endpoint {
        self.inner
    }
}
