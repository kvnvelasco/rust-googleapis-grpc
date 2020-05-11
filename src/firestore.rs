pub mod v1 {
    use tonic::transport::Channel;

    use crate::connection::{Credentials, GrpcEndpoint};
    use crate::google::firestore::v1::firestore_client::FirestoreClient;
    pub use crate::google::firestore::v1::{
        CreateDocumentRequest, DeleteDocumentRequest, Document, GetDocumentRequest,
    };
    use std::error::Error;
    use tonic::metadata::MetadataValue;
    use tonic::Code;

    pub struct Firestore {
        service: FirestoreClient<Channel>,
        credentials: Credentials,
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
                credentials,
            })
        }

        async fn add_metadata_to_request<X, R: tonic::IntoRequest<X>>(
            &mut self,
            document: R,
        ) -> Result<tonic::Request<X>, Box<dyn Error>> {
            let mut request = document.into_request();
            let meta = request.metadata_mut();
            let token = format!(
                "Bearer {}",
                self.credentials
                    .get_oauth_token("https://www.googleapis.com/auth/datastore")
                    .await?
            );
            meta.insert("authorization", MetadataValue::from_str(&token)?);
            Ok(request)
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
            let req = self.add_metadata_to_request(request).await.map_err(|_| {
                tonic::Status::new(
                    Code::FailedPrecondition,
                    "Unable to add metadata to  create document request",
                )
            })?;
            self.service.create_document(req).await
        }

        pub async fn get_document(
            &mut self,
            request: GetDocumentRequest,
        ) -> Result<tonic::Response<Document>, tonic::Status> {
            let req = self.add_metadata_to_request(request).await.map_err(|_| {
                tonic::Status::new(
                    Code::FailedPrecondition,
                    "Unable to add metadata to get document request",
                )
            })?;
            self.service.get_document(req).await
        }

        pub async fn delete_document(
            &mut self,
            request: DeleteDocumentRequest,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            let req = self.add_metadata_to_request(request).await.map_err(|_| {
                tonic::Status::new(
                    Code::FailedPrecondition,
                    "Unable to add metadata to  delete document request",
                )
            })?;
            self.service.delete_document(req).await
        }
    }
}
