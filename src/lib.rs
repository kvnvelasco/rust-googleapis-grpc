mod google {
    pub mod firestore {
        pub mod v1 {
            include!("protodefs/google.firestore.v1.rs");
        }
    }
    pub mod rpc {
        include!("protodefs/google.rpc.rs");
    }
    pub mod r#type {
        include!("protodefs/google.r#type.rs");
    }
}

pub mod firestore {
    use super::google;
    pub use google::firestore::v1::Document;
    use serde::Deserialize;
    use std::path::Path;
    use tokio::fs;
    use tonic::transport::{Certificate, ClientTlsConfig, Error};

    const HOST: &'static str = "https://firestore.googleapis.com";

    pub mod v1 {
        use super::*;
        use std::borrow::BorrowMut;
        use std::collections::HashMap;
        use tonic::client;
        use tonic::transport::{Channel, Identity, Endpoint};
        use crate::google::firestore::v1::{GetDocumentRequest, ListDocumentsRequest, CreateDocumentRequest, Value};
        use crate::google::firestore::v1::firestore_client::FirestoreClient;
        use crate::google::firestore::v1::value::ValueType;

        #[derive(Deserialize, Debug)]
        pub struct Credentials {
            pub client_x509_cert_url: String,
            auth_provider_x509_cert_url: String,
            private_key_id: String,
            project_id: String,
            private_key: String,
        }

        impl Credentials {
            pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
                serde_json::from_str(json_str)
            }
        }

        pub async fn connect(credentials: Credentials) -> Result<(), Box<dyn std::error::Error>> {

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
                .domain_name("firestore.googleapis.com");

            let channel = Channel::from_static(HOST)
                .tls_config(tls_config);



            let mut service = FirestoreClient::connect(channel).await?;

            let test_doc = Document {
                name: "".to_string(),
                fields: [( "Norway".to_owned(), Value { value_type: Some(ValueType::IntegerValue(53)) } )]
                    .iter().cloned().collect(),
                create_time: None,
                update_time: None
            };

            let data = service.create_document(CreateDocumentRequest {
                parent: "projects/gcp-infrastructure-272612/databases/(default)/documents".to_string(),
                collection_id: "test-collection".to_string(),
                document_id: "".to_string(),
                document: Some(test_doc),
                mask: None
            }).await?;

            dbg!(&data);

            Ok(())
        }
    }
}

mod tests;
