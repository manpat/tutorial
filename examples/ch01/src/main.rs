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

		gl::Enable(gl::DEPTH_TEST);
	}


	// Create a shader program.
	// ANCHOR: create_shader_program
	let main_shader = unsafe {
		// Compile our shaders, bailing if we hit any problems.
		let vert_shader = compile_shader(gl::VERTEX_SHADER, include_str!("shaders/vert.glsl"))?;
		let frag_shader = compile_shader(gl::FRAGMENT_SHADER, include_str!("shaders/frag.glsl"))?;

		// Create our program and attach our shaders to it for linking.
		let program = gl::CreateProgram();
		gl::AttachShader(program, vert_shader);
		gl::AttachShader(program, frag_shader);

		gl::LinkProgram(program);

		// Clean up the shaders we compiled above since we no longer need
		// them after linking, even if linking failed.
		for shader in [vert_shader, frag_shader] {
			gl::DetachShader(program, shader);
			gl::DeleteShader(shader);
		}

		check_program_status(program)?;

		program
	};
	// ANCHOR_END: create_shader_program


	// Create our empty VAO to allow glDrawArrays to draw without buffers bound.
	// ANCHOR: create_dummy_vao
	unsafe {
		let mut vao = 0;
		gl::CreateVertexArrays(1, &mut vao);
		gl::BindVertexArray(vao);
	}
	// ANCHOR_END: create_dummy_vao


	let mut event_pump = sdl_ctx.event_pump()
		.map_err(Error::msg)?;

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

		// Draw
		unsafe {
			let (w, h) = window.drawable_size();
			gl::Viewport(0, 0, w as i32, h as i32);

			gl::ClearColor(0.1, 0.1, 0.1, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT|gl::DEPTH_BUFFER_BIT);

			// ANCHOR: final_render
			// This corresponds to the number of indices in our vertex shader.
			let num_vertices = 6;

			gl::UseProgram(main_shader);
			gl::DrawArrays(gl::TRIANGLES, 0, num_vertices);
			// ANCHOR_END: final_render
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


// ANCHOR: compile_shader
fn compile_shader(ty: u32, src: &str) -> anyhow::Result<u32> {
	let src_c = std::ffi::CString::new(src)?;

	unsafe {
		let shader = gl::CreateShader(ty);
		gl::ShaderSource(shader, 1, &src_c.as_ptr(), std::ptr::null());
		gl::CompileShader(shader);

		// NOTE: on error, this will technically leak `shader`.
		// But we don't care for now since we're planning to
		// bail immediately on error
		check_shader_status(shader)?;

		Ok(shader)
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
// ANCHOR_END: compile_shader


// ANCHOR: check_program_status
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
// ANCHOR_END: check_program_status
