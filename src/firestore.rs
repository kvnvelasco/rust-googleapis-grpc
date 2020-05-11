pub mod v1 {
    use tonic::transport::Channel;

    use crate::connection::{Credentials, GrpcEndpoint};
    use crate::google::firestore::v1::firestore_client::FirestoreClient;
    pub use crate::google::firestore::v1::{
        CreateDocumentRequest, DeleteDocumentRequest, GetDocumentRequest,
    };

    use crate::google::firestore::v1::value::ValueType;
    use crate::google::firestore::v1::{Document as RPCDocument, UpdateDocumentRequest, Value};
    use std::collections::HashMap;
    use std::error::Error;
    use tonic::metadata::MetadataValue;
    use tonic::{Code, Response};

    pub struct Firestore {
        service: FirestoreClient<Channel>,
        credentials: Credentials,
        pub project_id: String,
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
            self.service
                .create_document(req)
                .await
                .map(transform_response_to_document_response)
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
            self.service
                .get_document(req)
                .await
                .map(transform_response_to_document_response)
        }

        pub async fn update_document(
            &mut self,
            request: UpdateDocumentRequest,
        ) -> Result<tonic::Response<Document>, tonic::Status> {
            let req = self.add_metadata_to_request(request).await.map_err(|_| {
                tonic::Status::new(
                    Code::FailedPrecondition,
                    "Unable to add metadata to update document request",
                )
            })?;

            self.service
                .update_document(req)
                .await
                .map(transform_response_to_document_response)
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

    fn transform_response_to_document_response(
        response: Response<RPCDocument>,
    ) -> Response<Document> {
        let mut resp = Response::new(Document::from(response.get_ref()));
        let metadata = resp.metadata_mut();
        *metadata = response.metadata().to_owned();
        resp
    }

    fn create_firestore_default_prefix(project_id: &str) -> String {
        return format!("projects/{}/databases/(default)/documents", project_id);
    }

    #[derive(Debug)]
    pub struct Document {
        inner: RPCDocument,
        fields: HashMap<String, Value>,
        address: Vec<String>,
        name: String,
    }

    impl Into<RPCDocument> for Document {
        fn into(self) -> RPCDocument {
            RPCDocument {
                name: self.address.join("/"),
                fields: self.fields,
                create_time: self.inner.create_time,
                update_time: self.inner.update_time,
            }
        }
    }

    impl From<Response<RPCDocument>> for Document {
        fn from(x: Response<RPCDocument>) -> Self {
            Document::from(x.get_ref())
        }
    }

    impl From<&RPCDocument> for Document {
        fn from(d: &RPCDocument) -> Self {
            Document::from(d.clone())
        }
    }

    impl From<RPCDocument> for Document {
        fn from(d: RPCDocument) -> Self {
            let d = d.to_owned();
            let address: Vec<String> = d.name.split("/").map(|i| i.to_owned()).collect();
            // all returned documents from the server have full paths, we want to strip that and only take the required path;
            // TOOD: Make this better somehow
            let address = address[5..].to_vec();
            Document {
                inner: d.clone(),
                fields: d.fields,
                name: address.last().expect("Document has no name").to_owned(),
                address: address[0..address.len() - 1].to_vec(),
            }
        }
    }

    pub trait IntoDocumentValue {
        fn into_value(self) -> Value;
    }

    impl IntoDocumentValue for String {
        fn into_value(self) -> Value {
            Value {
                value_type: Some(ValueType::StringValue(self)),
            }
        }
    }

    impl IntoDocumentValue for &'static str {
        fn into_value(self) -> Value {
            Value {
                value_type: Some(ValueType::StringValue(self.to_owned())),
            }
        }
    }

    impl IntoDocumentValue for i64 {
        fn into_value(self) -> Value {
            Value {
                value_type: Some(ValueType::IntegerValue(self)),
            }
        }
    }

    impl IntoDocumentValue for f64 {
        fn into_value(self) -> Value {
            Value {
                value_type: Some(ValueType::DoubleValue(self)),
            }
        }
    }

    impl Document {
        pub fn new<Name: AsRef<str>>(name: Name) -> Self {
            Document {
                inner: RPCDocument {
                    fields: Default::default(),
                    create_time: None,
                    update_time: None,
                    name: "".to_owned(),
                },
                address: Vec::new(),
                fields: Default::default(),
                name: name.as_ref().to_owned(),
            }
        }
        pub fn set_field<F: AsRef<str>, T: IntoDocumentValue>(
            &mut self,
            field_name: F,
            field_value: T,
        ) {
            let value = field_value.into_value();
            if let Some(_) = self.fields.get(field_name.as_ref()) {
                let inner = self.fields.get_mut(field_name.as_ref()).unwrap();
                *inner = value;
            } else {
                self.fields.insert(field_name.as_ref().to_owned(), value);
            }
        }

        pub fn get_field_value<F: AsRef<str>>(&self, key: F) -> Option<ValueType> {
            let maybe = self.fields.get(key.as_ref()).map(|v| v.value_type.clone());
            match maybe {
                Some(Some(v)) => Some(v),
                _ => None,
            }
        }

        pub fn push_address<S: AsRef<str>>(&mut self, value: S) -> &mut Self {
            self.address.push(value.as_ref().to_owned());
            self
        }

        fn create_parent(&self, project_id: &str) -> String {
            match self.address.len() {
                1 => create_firestore_default_prefix(project_id),
                _ => format!(
                    "{}/{}",
                    create_firestore_default_prefix(project_id),
                    self.address[0..self.address.len() - 1].join("/")
                ),
            }
        }

        pub fn create_document_request(&self, project_id: &str) -> CreateDocumentRequest {
            let parent = self.create_parent(project_id);
            let address = self.address.last().unwrap().to_owned();
            CreateDocumentRequest {
                parent,
                collection_id: address,
                document_id: self.name.clone(),
                document: Some(RPCDocument {
                    name: "".to_string(),
                    fields: self.fields.clone(),
                    create_time: None,
                    update_time: None,
                }),
                mask: None,
            }
        }

        pub fn delete_document_request(&self, project_id: &str) -> DeleteDocumentRequest {
            DeleteDocumentRequest {
                name: format!(
                    "{}/{}/{}",
                    create_firestore_default_prefix(project_id),
                    self.address.join("/"),
                    self.name
                ),
                current_document: None,
            }
        }
    }
}
