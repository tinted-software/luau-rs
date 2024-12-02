use std::ffi::CString;

use luau_sys::{
	luaL_newstate, luaL_openlibs, lua_pcall, luau_compile, luau_load,
};

fn main() {
	unsafe {
		let state = luaL_newstate();

		luaL_openlibs(state);

		let code = r#"
            print("Hello, world!")
        "#;

		let source = CString::new(code).unwrap();
		let mut compile_options = luau_sys::lua_CompileOptions {
			optimizationLevel: 1,
			debugLevel: 1,
			typeInfoLevel: 1,
			coverageLevel: 0,
			vectorLib: core::ptr::null_mut(),
			vectorCtor: core::ptr::null_mut(),
			vectorType: core::ptr::null_mut(),
			mutableGlobals: core::ptr::null_mut(),
			userdataTypes: core::ptr::null_mut(),
		};

		let mut bytecode_size = 0;
		let bytecode = luau_compile(
			source.as_ptr(),
			code.len() as _,
			&mut compile_options,
			&mut bytecode_size,
		);

		let chunk_name = CString::new("test").unwrap();
		luau_load(state, chunk_name.as_ptr(), bytecode, bytecode_size, 0);

		lua_pcall(state, 0, 0, 0);
	}
}
