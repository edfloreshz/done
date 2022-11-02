fn main() -> Result<(), Box<dyn std::error::Error>> {
	glib_build_tools::compile_resources(
		"data/resources/",
		"data/resources/resources.gresource.xml",
		"resources.gresource",
	);
	Ok(())
}
