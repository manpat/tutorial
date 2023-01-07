#[allow(dead_code)]
pub fn main() {
	let sdl_ctx = sdl2::init().expect("SDL init failed");
	let sdl_video = sdl_ctx.video().expect("video subsystem init failed");

	// Describe the version of OpenGL we want a context for _before_ creating the window.
	// We are using modern OpenGL, so 4.5+ and core profile is what we want.
	// ANCHOR: describe_context
	let gl_attr = sdl_video.gl_attr();
	gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
	gl_attr.set_context_version(4, 5);

	let window = sdl_video.window("Tutorial", 1366, 768)
		.position_centered()
		.resizable()
		.opengl() // This is important!
		.build()
		.expect("Failed to create window");
	// ANCHOR_END: describe_context

	// Create our context
	// ANCHOR: create_context_and_make_current
	let gl_ctx = window.gl_create_context().unwrap();
	window.gl_make_current(&gl_ctx).unwrap();
	// ANCHOR_END: create_context_and_make_current

	// Load GL functions
	// ANCHOR: load_gl_funcs
	gl::load_with(|s| sdl_video.gl_get_proc_address(s) as *const _);
	// ANCHOR_END: load_gl_funcs

	// Render an empty frame to make sure _something_ is working.
	// ANCHOR: make_sure_it_works
	unsafe {
		gl::ClearColor(1.0, 0.0, 1.0, 1.0);
		gl::Clear(gl::COLOR_BUFFER_BIT);
	}

	window.gl_swap_window();
	// ANCHOR_END: make_sure_it_works

	std::thread::sleep(std::time::Duration::from_secs(3));
}