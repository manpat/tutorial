#[allow(dead_code)]
pub fn main() {
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