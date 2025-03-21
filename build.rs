use bindgen::{EnumVariation, RustTarget};
use pkg_config::Config;
use std::env;
use std::path::PathBuf;

fn main() {
	let luau = Config::new().probe("luau").unwrap();

	for link_path in luau.link_paths {
		println!("cargo:rustc-link-search={}", link_path.display());
	}
	println!("cargo:rustc-link-search=luau/build");
	println!("cargo:rustc-link-lib=Luau.Analysis");
	println!("cargo:rustc-link-lib=Luau.Ast");
	println!("cargo:rustc-link-lib=Luau.Config");
	println!("cargo:rustc-link-lib=Luau.Compiler");
	println!("cargo:rustc-link-lib=Luau.CodeGen");
	println!("cargo:rustc-link-lib=Luau.VM");

	let cxx_stdlib = if let Ok(cxx_stdlib) = env::var("CXX_STDLIB") {
		match cxx_stdlib.as_str() {
			"stdc++" | "libstdc++" => vec!["stdc++".to_string()],
			"c++" | "libc++" => vec!["c++".to_string(), "c++abi".to_string()],
			_ => vec![cxx_stdlib],
		}
	} else {
		vec!["stdc++".to_string()]
	};

	for lib in &cxx_stdlib {
		println!("cargo:rustc-link-lib={}", lib);
	}

	let bindings = bindgen::Builder::default()
		.header("src/wrapper.h")
		.clang_arg("-xc++")
		.use_core()
		.rust_edition(bindgen::RustEdition::Edition2024)
		.rust_target(RustTarget::stable(85, 0).ok().unwrap())
		.layout_tests(false)
		.default_enum_style(EnumVariation::NewType {
			is_bitfield: false,
			is_global: false,
		})
		.derive_default(true)
		.parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
		.generate()
		.expect("Unable to generate bindings");

	let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
	bindings
		.write_to_file(out_path.join("bindings.rs"))
		.expect("Couldn't write bindings!");
}
