fn main() -> Result<(), Box<dyn std::error::Error>> {
	tonic_build::configure()
		.out_dir("./src/services")
		.protoc_arg("--experimental_allow_proto3_optional")
		.type_attribute(".", "#[derive(serde::Deserialize, serde::Serialize)]")
		.compile(&["provider.proto"], &["./proto"])?;
	Ok(())
}
