#[cfg(test)]
mod firestore_initialization {
    use crate::connection::Credentials;
    use crate::firestore::v1::Firestore;

    #[tokio::test]
    async fn test_authorization() {
        let connection = Credentials::auto_acquire()
            .map(|creds| Firestore::connect(creds))
            .expect("Unable to create connection");

        assert!(connection.await.is_ok())
    }
}
