use std::{path::PathBuf, env};

fn main() {
	glib_build_tools::compile_resources(
		"data/resources/",
		"data/resources/resources.gresource.xml",
		"resources.gresource",
	);
	let build_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    build_path.join("build");
    let build_path = build_path.to_str().unwrap();

    println!("cargo:rustc-link-lib=squid");
    println!("cargo:rustc-link-search=native={}", build_path);
    meson::build(".", build_path);
}
