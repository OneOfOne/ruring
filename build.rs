extern crate bindgen;

use std::process::Command;

fn main() {
	Command::new("sh")
		.arg("-e")
		.arg("[ -f lib/Makefile ] ||  git submodule update --init --recursive")
		.current_dir("lib")
		.status()
		.expect("failed to configure liburing");

	Command::new("sh")
		.arg("configure")
		.current_dir("lib")
		.status()
		.expect("failed to build liburin");
	
	Command::new("make")
		.arg("liburing.a")
		.current_dir("lib/src")
		.env("CFLAGS", "-fPIC -O2 -fno-plt")
		.status()
		.expect("failed to build liburing.a");

	println!("cargo:rustc-link-lib=static=uring");
	println!("cargo:rerun-if-changed=wrapper.h");
	println!("cargo:rustc-link-search=native=lib/src");

	// Generate bindings
	let bindings = bindgen::Builder::default()
		.allowlist_function("__io_uring.*")
		.allowlist_function("io_uring.*")
		.allowlist_var("IORING.*")
		.allowlist_var("IOSQE.*")
		.allowlist_type("io_uring.*")
		.header("wrapper.h")
		.use_core()
		.prepend_enum_name(false)
		.disable_name_namespacing()
		.generate()
		.expect("Unable to generate bindings");

	bindings.write_to_file("src/raw/bindings.rs").expect("Couldn't write bindings!");
}
