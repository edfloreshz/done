fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional")
        .type_attribute(".", "#[derive(serde::Deserialize, serde::Serialize)]")
        .compile(&["provider.proto"], &["."])?;
    Ok(())
}