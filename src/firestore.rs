pub mod v1 {
    use tonic::transport::Channel;

    use crate::connection::{Credentials, GrpcEndpoint};
    use crate::google::firestore::v1::firestore_client::FirestoreClient;

    pub struct Firestore {
        service: FirestoreClient<Channel>,
        project_id: String,
    }

    impl Firestore {
        pub async fn connect(credentials: Credentials) -> Result<Self, Box<dyn std::error::Error>> {
            let service = FirestoreClient::connect(
                GrpcEndpoint::new(&credentials, "firestore.googleapis.com").await?,
            )
            .await?;

            Ok(Firestore {
                service,
                project_id: credentials.project_id.clone(),
            })
        }
    }
}
