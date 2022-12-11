use anyhow::Error;


fn main() -> anyhow::Result<()> {
	let sdl_ctx = sdl2::init().map_err(Error::msg)?;
	let sdl_video = sdl_ctx.video().map_err(Error::msg)?;

	let gl_attr = sdl_video.gl_attr();
	gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
	gl_attr.set_context_version(4, 5);

	// Part 1 of setting up a debug context
	gl_attr.set_context_flags().debug().set();
	
	// Part 1 of ensuring srgb-correctness
	gl_attr.set_framebuffer_srgb_compatible(true);
	gl_attr.set_stencil_size(8);

	let window = sdl_video.window("Sprite", 1366, 768)
		.position_centered()
		.resizable()
		.opengl()
		.build()?;

	let gl_ctx = window.gl_create_context().map_err(Error::msg)?;
	window.gl_make_current(&gl_ctx).map_err(Error::msg)?;

	sdl_video.gl_set_swap_interval(sdl2::video::SwapInterval::VSync)
		.map_err(Error::msg)?;

	gl::load_with(|s| sdl_video.gl_get_proc_address(s) as *const _);

	unsafe {
		// Part 2 of setting up a debug context
		gl::DebugMessageCallback(Some(gl_message_callback), std::ptr::null());
		gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);

		// Disable performance messages
		gl::DebugMessageControl(
			gl::DONT_CARE,
			gl::DEBUG_TYPE_PERFORMANCE,
			gl::DONT_CARE,
			0, std::ptr::null(),
			0 // false
		);

		// Disable notification messages
		gl::DebugMessageControl(
			gl::DONT_CARE,
			gl::DONT_CARE,
			gl::DEBUG_SEVERITY_NOTIFICATION,
			0, std::ptr::null(),
			0 // false
		);

		// Part 2 of ensuring srgb-correctness
		gl::Enable(gl::FRAMEBUFFER_SRGB);
	}


	// Create a shader program.
	let main_shader = unsafe {
		use std::ffi::CString;

		let sources = [
			(gl::VERTEX_SHADER, include_str!("shaders/vert.glsl")),
			(gl::FRAGMENT_SHADER, include_str!("shaders/frag.glsl")),
		];

		let program = gl::CreateProgram();

		for (ty, src) in sources {
			let src_c = CString::new(src)?;

			let shader = gl::CreateShader(ty);
			gl::ShaderSource(shader, 1, &src_c.as_ptr(), std::ptr::null());
			gl::CompileShader(shader);

			// This will leak the program on error but we don't care because
			// we're dying immediately anyway.
			check_shader_status(shader)?;

			gl::AttachShader(program, shader);

			// Calling glDeleteShader here will not delete it immediately, but will defer its
			// deletion until the program it is linked to is deleted.
			gl::DeleteShader(shader);
		}

		gl::LinkProgram(program);

		check_program_status(program)?;

		program
	};


	// Create a VAO to allow us to render.
	// One is required for any drawcall that reads vertex array state (including enablement state).
	let vao = unsafe {
		let mut handle = 0;
		gl::CreateVertexArrays(1, &mut handle);
		handle
	};


	let mut event_pump = sdl_ctx.event_pump()
		.map_err(Error::msg)?;

	'main: loop {
		use sdl2::event::{Event, WindowEvent};
		use sdl2::keyboard::Scancode;

		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} | Event::KeyDown { scancode: Some(Scancode::Escape), .. } => {
					break 'main
				}

				Event::Window{ win_event: WindowEvent::Resized(..), .. } => {
					// resize whatever
				}

				_ => {}
			}
		}

		unsafe {
			gl::ClearColor(0.1, 0.1, 0.1, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT|gl::DEPTH_BUFFER_BIT);

			gl::BindVertexArray(vao);
			gl::UseProgram(main_shader);
			gl::DrawArrays(gl::TRIANGLES, 0, 6);
		}


		window.gl_swap_window();
	}


	Ok(())
}





extern "system" fn gl_message_callback(source: u32, ty: u32, _id: u32, severity: u32,
	_length: i32, msg: *const i8, _ud: *mut std::ffi::c_void)
{
	let severity_str = match severity {
		gl::DEBUG_SEVERITY_HIGH => "high",
		gl::DEBUG_SEVERITY_MEDIUM => "medium",
		gl::DEBUG_SEVERITY_LOW => "low",
		gl::DEBUG_SEVERITY_NOTIFICATION => return,
		_ => panic!("Unknown severity {}", severity),
	};

	let ty = match ty {
		gl::DEBUG_TYPE_ERROR => "error",
		gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => "deprecated behaviour",
		gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR => "undefined behaviour",
		gl::DEBUG_TYPE_PORTABILITY => "portability",
		gl::DEBUG_TYPE_PERFORMANCE => "performance",
		gl::DEBUG_TYPE_OTHER => "other",
		_ => panic!("Unknown type {}", ty),
	};

	let source = match source {
		gl::DEBUG_SOURCE_API => "api",
		gl::DEBUG_SOURCE_WINDOW_SYSTEM => "window system",
		gl::DEBUG_SOURCE_SHADER_COMPILER => "shader compiler",
		gl::DEBUG_SOURCE_THIRD_PARTY => "third party",
		gl::DEBUG_SOURCE_APPLICATION => "application",
		gl::DEBUG_SOURCE_OTHER => "other",
		_ => panic!("Unknown source {}", source),
	};

	eprintln!("GL ERROR!");
	eprintln!("Source:   {}", source);
	eprintln!("Severity: {}", severity_str);
	eprintln!("Type:     {}", ty);

	unsafe {
		let msg = std::ffi::CStr::from_ptr(msg as _).to_str().unwrap();
		eprintln!("Message: {}", msg);
	}

	match severity {
		gl::DEBUG_SEVERITY_HIGH | gl::DEBUG_SEVERITY_MEDIUM => panic!("GL ERROR!"),
		_ => {}
	}
}





fn check_shader_status(shader_handle: u32) -> anyhow::Result<()> {
	unsafe {
		let mut status = 0;
		gl::GetShaderiv(shader_handle, gl::COMPILE_STATUS, &mut status);

		if status == 0 {
			let mut length = 0;
			gl::GetShaderiv(shader_handle, gl::INFO_LOG_LENGTH, &mut length);

			let mut buffer = vec![0u8; length as usize];
			gl::GetShaderInfoLog(
				shader_handle,
				length,
				std::ptr::null_mut(),
				buffer.as_mut_ptr() as *mut _
			);

			let error_msg = String::from_utf8_lossy(&buffer[..buffer.len()-1]);
			anyhow::bail!("Shader failed to compile: {error_msg}");
		}	
	}

	Ok(())
}


fn check_program_status(program_handle: u32) -> anyhow::Result<()> {
	unsafe {
		let mut status = 0;
		gl::GetProgramiv(program_handle, gl::LINK_STATUS, &mut status);

		if status == 0 {
			let mut length = 0;
			gl::GetProgramiv(program_handle, gl::INFO_LOG_LENGTH, &mut length);

			let mut buffer = vec![0u8; length as usize];
			gl::GetProgramInfoLog(
				program_handle,
				length,
				std::ptr::null_mut(),
				buffer.as_mut_ptr() as *mut _
			);

			let error_msg = String::from_utf8_lossy(&buffer[..buffer.len()-1]);
			anyhow::bail!("Program failed to link: {error_msg}");
		}	
	}

	Ok(())
}