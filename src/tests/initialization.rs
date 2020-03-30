#[cfg(test)]
mod firestore_initialization {
    use crate::firestore;
    use std::path::{Path, PathBuf};

    #[tokio::test]
    async fn test_authorization() {
        let creds = include_str!("./credentials.json");
        let connection = firestore::v1::connect(
            firestore::v1::Credentials::from_json(creds).expect("Unable to parse credentials"),
        )
        .await;

        dbg!(&connection);
        assert!(connection.is_ok())
    }
}
