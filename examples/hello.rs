use luau_sys::safe::LuaState;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let state = LuaState::new();

	let code = r#"
	   print("Hello, world!")
	"#;

	// Comment this out to see a call error
	state.load_default_functions();
	state.load(code, "hello", None)?;
	state.call(0, 0, 0)?;

	Ok(())
}
