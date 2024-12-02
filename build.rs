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

	let mut args = vec![
		"src/wrapper.h".to_string(),
		"--use-core".to_string(),
		"--no-layout-tests".to_string(),
		"--".to_string(),
		"-x".to_string(),
		"c++".to_string(),
	];
	for include_path in luau.include_paths {
		args.push("-I".to_string());
		args.push(include_path.to_string_lossy().to_string());
	}
	for cxx_stdlib in &cxx_stdlib {
		args.push("-l".to_string());
		args.push(cxx_stdlib.clone());
	}

	let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

	std::fs::write(
		out_path.join("bindings.rs"),
		std::process::Command::new("bindgen")
			.args(args)
			.output()
			.expect("Failed to execute bindgen")
			.stdout,
	)
	.expect("Failed to write bindings");
}
