fn main() {
	glib_build_tools::compile_resources(
		"src",
		"src/done.gresource.xml",
		"compiled.gresource",
	);
}
