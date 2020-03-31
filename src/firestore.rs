pub mod v1 {
    use tonic::transport::Channel;

    use crate::connection::{Credentials, GrpcEndpoint};
    use crate::google::firestore::v1::firestore_client::FirestoreClient;
    pub use crate::google::firestore::v1::{
        CreateDocumentRequest, DeleteDocumentRequest, Document, GetDocumentRequest,
    };

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

        pub fn generate_document_prefix(&self, name: &str) -> String {
            if name.len() > 0 {
                format!(
                    "projects/{}/databases/(default)/documents/{}",
                    &self.project_id, name
                )
            } else {
                format!(
                    "projects/{}/databases/(default)/documents",
                    &self.project_id
                )
            }
        }

        pub async fn create_document(
            &mut self,
            request: CreateDocumentRequest,
        ) -> Result<tonic::Response<Document>, tonic::Status> {
            self.service.create_document(request).await
        }

        pub async fn get_document(
            &mut self,
            request: GetDocumentRequest,
        ) -> Result<tonic::Response<Document>, tonic::Status> {
            self.service.get_document(request).await
        }

        pub async fn delete_document(
            &mut self,
            request: DeleteDocumentRequest,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.service.delete_document(request).await
        }
    }
}
