fn just_window() {
// ANCHOR: just_window
	// These two calls are more or less equivalent to `SDL_Init(SDL_INIT_VIDEO)`
	let sdl_ctx = sdl2::init().expect("SDL init failed");
	let sdl_video = sdl_ctx.video().expect("video subsystem init failed");

	let window = sdl_video.window("Tutorial", 1366, 768)
		.position_centered()
		.build()
		.expect("Failed to create window");

	std::thread::sleep(std::time::Duration::from_secs(3));
// ANCHOR_END: just_window
}


fn create_gl_context() {
	// ANCHOR: create_gl_context
	let sdl_ctx = sdl2::init().expect("SDL init failed");
	let sdl_video = sdl_ctx.video().expect("video subsystem init failed");

	// Describe the version of OpenGL we want a context for _before_ creating the window.
	// We are using modern OpenGL, so 4.5+ and core profile is what we want.
	// ANCHOR: describe_context
	let gl_attr = sdl_video.gl_attr();
	gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
	gl_attr.set_context_version(4, 5);
	// ANCHOR_END: describe_context

	let window = sdl_video.window("Tutorial", 1366, 768)
		.position_centered()
		.resizable()
		.opengl() // This is important!
		.build()
		.expect("Failed to create window");

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
	// ANCHOR_END: create_gl_context
}


fn basic_loop() {
	let sdl_ctx = sdl2::init().expect("SDL init failed");
	let sdl_video = sdl_ctx.video().expect("video subsystem init failed");

	let window = sdl_video.window("Tutorial", 1366, 768)
		.position_centered()
		.resizable()
		.opengl()
		.build()
		.expect("Failed to create window");

	let mut event_pump = sdl_ctx.event_pump()
		.expect("Failed to create event pump");

	// Create our context
	let gl_ctx = window.gl_create_context().unwrap();
	window.gl_make_current(&gl_ctx).unwrap();

	// Load GL functions
	gl::load_with(|s| sdl_video.gl_get_proc_address(s) as *const _);

	// Enable VSync
	sdl_video.gl_set_swap_interval(sdl2::video::SwapInterval::VSync).unwrap();

	'main: loop {
		for event in event_pump.poll_iter() {
			use sdl2::event::Event;
			use sdl2::keyboard::Scancode;

			match event {
				Event::Quit{..} | Event::KeyDown { scancode: Some(Scancode::Escape), .. } => {
					break 'main
				}

				_ => {}
			}
		}

		unsafe {
			gl::ClearColor(1.0, 0.0, 1.0, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT);
		}

		window.gl_swap_window();
	}
}




fn main() {
	create_gl_context();
}