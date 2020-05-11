use tonic_build::configure;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    configure()
        .build_server(false)
        .out_dir("./src/protodefs")
        .compile(
            &[
                "googleapis-src/google/firestore/v1/firestore.proto",
                "googleapis-src/google/api/auth.proto",
                "googleapis-src/google/datastore/v1/datastore.proto",
                "googleapis-src/google/api/auth.proto",
            ],
            &["googleapis-src"],
        )?;
    Ok(())
}
