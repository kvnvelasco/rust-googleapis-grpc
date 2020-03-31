#[cfg(test)]
mod firestore {
    use crate::connection::Credentials;
    use crate::firestore::v1::Firestore;
    use crate::google::firestore::v1::{CreateDocumentRequest, GetDocumentRequest, DeleteDocumentRequest};
    use std::error::Error;

    async fn establish_connection() -> Result<Firestore, Box<dyn Error>> {
        Credentials::from_json(include_str!("./credentials.json"))
            .map(|c| Firestore::connect(c))
            .expect("Unable to create connection")
            .await
    }

    #[tokio::test]
    async fn test_authorization() {
        let connection = establish_connection().await;
        assert!(connection.is_ok())
    }

    #[tokio::test]
    async fn test_create_document() {
        let mut connection = establish_connection()
            .await
            .expect("Unable to establish connection");

        let output = connection
            .create_document(CreateDocumentRequest {
                parent: connection.generate_document_prefix(""),
                collection_id: "test-collection".to_string(),
                document_id: "test-item".to_string(),
                document: None,
                mask: None,
            })
            .await;

        assert!(output.is_ok())
    }

    #[tokio::test]
    async fn test_get_document() {
        let mut connection = establish_connection()
            .await
            .expect("Unable to establish connection");

        let doc = connection
            .get_document(GetDocumentRequest {
                name: connection.generate_document_prefix("test-collection/test-item"),
                mask: None,
                consistency_selector: None,
            })
            .await;

        assert!(doc.is_ok());
    }

    #[tokio::test]
    async fn test_delete_document() {
        let mut connection = establish_connection()
            .await
            .expect("Unable to establish connection");

        let doc = connection
            .delete_document(DeleteDocumentRequest {
                name: connection.generate_document_prefix("test-collection/test-item"),
                current_document: None
            })
            .await;

        assert!(doc.is_ok());
    }
}
