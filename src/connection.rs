use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use tonic::transport::{Channel, ClientTlsConfig, Endpoint, Identity, Uri};

#[derive(Serialize)]
struct Claim {
    iss: String,
    scope: String,
    aud: &'static str,
    exp: usize,
    iat: usize,
}

impl Claim {
    fn new<S: AsRef<str>>(iss: String, scope: S) -> Self {
        let now = {
            let systime = SystemTime::now();
            systime.duration_since(UNIX_EPOCH).unwrap().as_secs() as usize
        };
        Claim {
            iss,
            scope: scope.as_ref().to_owned(),
            aud: "https://oauth2.googleapis.com/token",
            exp: now + 3600,
            iat: now,
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct Credentials {
    client_x509_cert_url: String,
    // auth_provider_x509_cert_url: String,
    client_email: String,
    private_key_id: String,
    pub project_id: String,
    private_key: String,
    oauth_token: Option<String>,
}
#[derive(Deserialize, Debug)]
struct OauthResponse {
    access_token: String,
    scope: Option<String>,
    token_type: String,
    expires_in: usize,
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

    pub async fn get_oauth_token(&mut self, scope: &str) -> Result<String, Box<dyn Error>> {
        match &self.oauth_token {
            Some(x) => Ok(x.to_owned()),
            None => {
                self.request_oauth_token(scope).await?;
                if let Some(token) = &self.oauth_token {
                    Ok(token.clone())
                } else {
                    Err("Unable to get things".into())
                }
            }
        }
    }

    async fn request_oauth_token(&mut self, scope: &str) -> Result<(), Box<dyn Error>> {
        let request = self.generate_jwt_request(scope)?;
        let client = reqwest::Client::new();
        let params = [
            ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
            ("assertion", &request),
        ];
        let res: reqwest::Response = client
            .post("https://oauth2.googleapis.com/token")
            .form(&params)
            .send()
            .await?;
        let body: OauthResponse = res.json::<OauthResponse>().await?;
        self.oauth_token = Some(body.access_token);
        Ok(())
    }

    fn generate_jwt_request(&self, scope: &str) -> Result<String, Box<dyn Error>> {
        let req = Claim::new(self.client_email.to_owned(), scope);
        let header = Header {
            alg: Algorithm::RS256,
            cty: None,
            jku: None,
            kid: None,
            x5u: None,
            typ: Some("JWT".to_owned()),
            x5t: None,
        };

        let output = encode(
            &header,
            &req,
            &EncodingKey::from_rsa_pem(self.private_key.as_bytes())?,
        );
        output.map_err(|e| e.into())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_gets_credentials() {}

    #[test]
    fn it_generates_a_jwt_token_correcty() {
        let creds = Credentials::from_json(include_str!("./tests/credentials.json")).unwrap();
        let jwt = creds.generate_jwt_request("https://www.googleapis.com/auth/datastore");
        println!("Generated JWT: {:?}", &jwt);
        assert!(jwt.is_ok())
    }

    #[tokio::test]
    async fn it_grabs_the_correct_oauth_token() {
        let mut creds = Credentials::from_json(include_str!("./tests/credentials.json")).unwrap();
        let output = creds
            .request_oauth_token("https://www.googleapis.com/auth/datastore")
            .await;
        println!("{:?}", output);
        assert!(creds.oauth_token.is_some());
    }
}
