#[cfg(test)]
mod firestore {
    use crate::connection::Credentials;
    use crate::firestore::v1::{Document, Firestore};
    use crate::google::firestore::v1::value::ValueType;
    use crate::google::firestore::v1::{
        CreateDocumentRequest, DeleteDocumentRequest, GetDocumentRequest,
    };
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

    fn create_document(firestore: &mut Firestore) {}

    #[tokio::test]
    async fn it_creates_a_document() {
        let mut connection = establish_connection()
            .await
            .expect("Unable to establish connection");

        let mut document = connection.new_document("test-new-doc-thing");
        document.push_address("test-collection");
        let request = document.create_document_request();

        let response = connection.create_document(request).await;
        debug_assert!(response.is_ok(), "{:?}", &response);

        let deleted = connection
            .delete_document(response.unwrap().get_ref().delete_document_request())
            .await;

        debug_assert!(deleted.is_ok(), "{:?}", deleted);
    }

    #[tokio::test]
    async fn it_can_get_a_document() {
        let mut connection = establish_connection()
            .await
            .expect("Unable to establish connection");

        let mut document = connection.new_document("dcaecaw");
        document.push_address("test-collection");
        connection.create_document(document.create_document_request()).await;

        let item = connection.get_document(document.get_document_request()).await;
        debug_assert!(item.is_ok(), "{:?}", &item);

        connection.delete_document(document.delete_document_request()).await;
    }

    #[tokio::test]
    async fn it_updates_a_document() {
        let mut connection = establish_connection()
            .await
            .expect("Unable to establish connection");
        let mut document = connection.new_document("awdagfawegac-doc");
        document.push_address("test-collection");

        let created_doc = connection
            .create_document(document.create_document_request())
            .await;

        debug_assert!(created_doc.is_ok(), "{:?}", &created_doc);
        let mut response = created_doc.unwrap();
        let doc = response.get_mut();
        doc.set_field("value", "value");

        let updated_doc = connection
            .update_document(doc.update_document_request())
            .await;

        debug_assert!(updated_doc.is_ok(), "{:?}", &updated_doc);
        let updated_doc = updated_doc.unwrap();
        assert_eq!(
            &updated_doc.get_ref().get_field_value("value"),
            &Some(ValueType::StringValue("value".to_owned()))
        );
        let next = updated_doc.get_ref().to_owned();
        connection.delete_document(next.delete_document_request()).await;
    }

    async fn test_create_read_delete() {
        let mut connection = establish_connection()
            .await
            .expect("Unable to establish connection");

        let document = connection
            .create_document(CreateDocumentRequest {
                parent: connection.generate_document_prefix(""),
                collection_id: "test-collection".to_string(),
                document_id: "test-item".to_string(),
                document: None,
                mask: None,
            })
            .await;

        println!("{:?}", &document);
        assert!(document.is_ok());

        let document = connection
            .get_document(GetDocumentRequest {
                name: connection.generate_document_prefix("test-collection/test-item"),
                mask: None,
                consistency_selector: None,
            })
            .await;

        assert!(document.is_ok());

        let document = connection
            .delete_document(DeleteDocumentRequest {
                name: connection.generate_document_prefix("test-collection/test-item"),
                current_document: None,
            })
            .await;

        assert!(document.is_ok());
    }

    #[tokio::test]
    async fn test_update() {}
}
