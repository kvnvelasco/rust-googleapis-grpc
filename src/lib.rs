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

mod connection;

pub mod firestore;

mod tests;
