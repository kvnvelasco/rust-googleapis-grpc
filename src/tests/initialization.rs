#[cfg(test)]
mod firestore_initialization {
    use crate::connection::Credentials;
    use crate::firestore::v1::Firestore;

    #[tokio::test]
    async fn test_authorization() {
        let creds = include_str!("./credentials.json");
        let connection = Firestore::connect(
            Credentials::from_json(creds).expect("Unable to parse credentials"),
        )
        .await;

        assert!(connection.is_ok())
    }
}
