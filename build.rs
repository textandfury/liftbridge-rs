fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .compile(&["proto/api.v1.proto"], &["proto"])?;
    Ok(())
}
