# Rust GoogleApis RPC Bindings 

Provides an autocomplete friendly set of bindings to googleapis via grpc. Utilizes the 
excellent `tonic` package for codegen. 


# Basic Usage 

## Authorization 
The library assumes that you will be using service account authorization and requires a
google credentials json file. The library will eventually load the credential file from 
GOOGLE_APPLICATION_CREDENTIALS if available. 

### Connecting

As an example for connecting to the firestore service:

```rust
async fn main() {
    let firestore = Credentials::auto_acquire()
        .map(|creds| Firestore::connect(creds))
        .expect("Unable to create connection");
}
```

## Progress 

- [ ] Firestore
   - [x]  Create, read, delete 
   - [ ]  Transactions, commits, and rollbacks 
   - [ ]  Streaming 
   - [ ]  Subscriptions 
   - [ ]  Derive Macros
- [ ] Datastores 
- [ ] App Engine 
- [ ] Big Table 
- [ ] Cloud Run 
- [ ] Spanner 
- [ ] Streetview