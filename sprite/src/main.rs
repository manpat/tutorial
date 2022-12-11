use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
	let sdl_ctx = sdl2::init()?;
	let sdl_video = sdl_ctx.video()?;

	let gl_attr = sdl_video.gl_attr();
	gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
	gl_attr.set_context_version(4, 5);
	gl_attr.set_context_flags().debug().set();
	
	gl_attr.set_framebuffer_srgb_compatible(true);
	gl_attr.set_stencil_size(8);

	let window = sdl_video.window("Sprite", 1366, 768)
		.position_centered()
		.resizable()
		.opengl()
		.build()?;

	let gl_ctx = window.gl_create_context()?;
	window.gl_make_current(&gl_ctx)?;

	sdl_video.gl_set_swap_interval(sdl2::video::SwapInterval::VSync)?;

	gl::load_with(|s| sdl_video.gl_get_proc_address(s) as *const _);

	let mut event_pump = sdl_ctx.event_pump()?;


	'main: loop {
		use sdl2::event::Event;
		use sdl2::keyboard::Scancode;

		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} | Event::KeyDown { scancode: Some(Scancode::Escape), .. } => {
					break 'main
				}
				_ => {}
			}
		}

		unsafe {
			gl::ClearColor(0.1, 0.1, 0.1, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT|gl::DEPTH_BUFFER_BIT);
		}


		window.gl_swap_window();
	}


	Ok(())
}
