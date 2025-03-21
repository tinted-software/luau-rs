#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![cfg_attr(not(feature = "std"), no_std)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(feature = "std")]
pub mod safe {
	use std::ffi::CString;
	use std::slice;

	use crate::lua_CFunction;
	use crate::lua_CompileOptions;
	use crate::lua_State;
	use crate::lua_close;
	use crate::lua_pcall;
	use crate::lua_pushcclosurek;
	use crate::lua_tolstring;
	use crate::luaL_newstate;
	use crate::luaL_openlibs;
	use crate::luau_compile;
	use crate::luau_load;

	#[derive(Debug)]
	pub enum LuaError {
		CompileError,
		LoadError,
		CallError(String),
	}

	impl std::fmt::Display for LuaError {
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			match self {
				Self::CompileError => write!(f, "Failed to compile code"),
				Self::LoadError => write!(f, "Failed to load luau bytecode"),
				Self::CallError(message) => {
					write!(f, "Failed to call function: {}", message)
				}
			}
		}
	}
	impl std::error::Error for LuaError {}

	pub struct LuaState {
		state: *mut lua_State,
	}

	impl LuaState {
		pub fn new() -> Self {
			unsafe {
				let state = luaL_newstate();
				Self { state }
			}
		}

		pub fn load_default_functions(&self) {
			unsafe {
				luaL_openlibs(self.state);
			}
		}

		pub fn load(
			&self,
			code: &str,
			chunk_name: &str,
			options: Option<lua_CompileOptions>,
		) -> Result<(), LuaError> {
			unsafe {
				let source = CString::new(code).unwrap();
				let chunk_name = CString::new(chunk_name).unwrap();

				let mut compile_options =
					options.unwrap_or(lua_CompileOptions {
						optimizationLevel: 1,
						debugLevel: 1,
						typeInfoLevel: 1,
						coverageLevel: 0,
						..Default::default()
					});

				let mut bytecode_size = 0;
				let bytecode = luau_compile(
					source.as_ptr(),
					code.len() as _,
					&mut compile_options,
					&mut bytecode_size,
				);
				if bytecode.is_null() {
					return Err(LuaError::CompileError);
				}

				let result = luau_load(
					self.state,
					chunk_name.as_ptr(),
					bytecode,
					bytecode_size,
					0,
				);
				std::ptr::drop_in_place(bytecode);

				if result != 0 {
					return Err(LuaError::LoadError);
				}
			}

			Ok(())
		}

		pub fn call(
			&self,
			args: i32,
			results: i32,
			error_handler: i32,
		) -> Result<(), LuaError> {
			unsafe {
				if lua_pcall(self.state, args, results, error_handler) != 0 {
					let error_message = {
						let mut len: usize = 0;
						let version_ptr =
							lua_tolstring(self.state, -1, &mut len);
						let s = slice::from_raw_parts(version_ptr, len);
						core::str::from_utf8(s).unwrap()
					};

					return Err(LuaError::CallError(error_message.to_string()));
				}
			}

			Ok(())
		}

		pub fn push_cfunction(
			&self,
			function_name: String,
			parameter_count: i32,
			function: lua_CFunction,
		) {
			unsafe {
				let function_name = CString::new(function_name).unwrap();
				lua_pushcclosurek(
					self.state,
					function,
					function_name.as_ptr(),
					parameter_count,
					None,
				);
			}
		}

		pub fn get_internal_state(&self) -> *mut lua_State {
			self.state
		}
	}

	impl Default for LuaState {
		fn default() -> Self {
			Self::new()
		}
	}

	impl Drop for LuaState {
		fn drop(&mut self) {
			unsafe {
				lua_close(self.state);
			}
		}
	}
}
