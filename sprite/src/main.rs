use anyhow::Error;
use glam::{Vec2, Vec3, Vec3A, Vec4, Mat4, Mat3A, Vec3Swizzles};


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


	// Create a buffer to house our uniforms.
	let uniform_buffer = unsafe {
		let mut handle = 0;
		gl::CreateBuffers(1, &mut handle);
		handle
	};

	// Create a buffer to house our per-sprite data.
	let sprite_buffer = unsafe {
		let mut handle = 0;
		gl::CreateBuffers(1, &mut handle);
		handle
	};


	// Load our sprite atlas.
	let texture = load_texture("sprite/assets/atlas.png")?;


	let mut event_pump = sdl_ctx.event_pump()
		.map_err(Error::msg)?;

	let mut time = 0.0f32;

	'main: loop {
		time += 1.0 / 60.0;

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

		let view_matrix = Mat4::from_translation(-Vec3::Z * 3.0)
						* Mat4::from_rotation_y(time*0.6);

		let sprites = [
			SpriteUniform {
				transform: Mat3A::from_cols(Vec3A::X, Vec3A::Y, Vec3A::ZERO),
				color: Vec4::new(1.0, 1.0, 1.0, (time*0.8).cos() * 0.4 + 0.6),
				uv_scale: Vec2::splat(0.5),
				uv_offset: Vec2::ZERO,
			},

			SpriteUniform {
				transform: Mat3A::from_cols(0.3 * Vec3A::Y, -0.4 * Vec3A::X, Vec3A::new(0.7, 0.4, 0.2)),
				color: Vec4::new(1.0, 0.5, 1.0, 0.5),
				uv_scale: Vec2::splat(0.5),
				uv_offset: Vec2::ZERO,
			},

			SpriteUniform {
				transform: {
					let right = 0.3 * Vec2::from((-time).sin_cos());
					let up = right.perp();
					let offset = 0.8 * Vec2::from((0.7 * time).sin_cos());

					Mat3A::from_cols(right.extend(0.0).into(), up.extend(0.0).into(), offset.extend(-0.6).into())
				},
				color: Vec4::new(1.0, 1.0, 0.5, 1.0),
				uv_scale: Vec2::splat(0.5),
				uv_offset: Vec2::new(0.5, 0.0),
			},

			SpriteUniform {
				transform: {
					let inv_view = view_matrix.inverse();

					let right = 0.3 * inv_view.x_axis;
					let up = 0.3 * inv_view.y_axis;
					let offset = Vec3::new(2.0, 1.0, 2.0) * Vec2::from((0.6 * time).sin_cos()).extend(0.5).zyx();

					Mat3A::from_cols(right.into(), up.into(), offset.into())
				},
				color: Vec4::new(0.5, 1.0, 0.5, 1.0),
				uv_scale: Vec2::splat(0.5),
				uv_offset: Vec2::new(0.5, 0.0),
			},

			SpriteUniform {
				transform: Mat3A::from_cols(Vec3A::X, -Vec3A::Z, Vec3A::new(0.0, -1.0, 0.0)),
				color: Vec4::new(0.5, 1.0, 1.0, 1.0),
				uv_scale: Vec2::splat(0.5),
				uv_offset: Vec2::new(0.5, 0.0),
			},
		];

		// Update buffers
		unsafe {
			use std::f32::consts::PI;

			let (w, h) = window.drawable_size();
			let aspect = w as f32 / h as f32;
			let uniforms = Uniforms {
				// Create an orthographic projection that preserves a 1x1 safe region in the center of the screen.
				// projection: Mat4::from_scale(Vec3::new(aspect.recip().min(1.0), aspect.min(1.0), 1.0)),
				projection: {
					Mat4::perspective_rh_gl(PI/3.0, aspect, 0.01, 100.0) * view_matrix
				},
			};

			upload_buffer(uniform_buffer, &[uniforms], gl::STREAM_DRAW);
			upload_buffer(sprite_buffer, &sprites, gl::STREAM_DRAW);
		}


		// Draw
		unsafe {
			let (w, h) = window.drawable_size();
			gl::Viewport(0, 0, w as i32, h as i32);

			gl::ClearColor(0.1, 0.1, 0.1, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT|gl::DEPTH_BUFFER_BIT);

			// Bind our uniform buffer to 0th ubo binding slot - matching the layout specified in vert.glsl
			gl::BindBufferBase(gl::UNIFORM_BUFFER, 0, uniform_buffer);

			// Bind our sprite buffer to 0th ssbo binding slot - matching the layout specified in vert.glsl
			gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 0, sprite_buffer);

			// Bind our sprite atlas to 0th texture unit - matching the binding specified in frag.glsl
			gl::BindTextureUnit(0, texture);

			gl::BindVertexArray(vao);
			gl::UseProgram(main_shader);
			gl::DrawArrays(gl::TRIANGLES, 0, (sprites.len() * 6) as _);
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



unsafe fn upload_buffer<T: Copy>(handle: u32, data: &[T], usage: u32) {
	if data.is_empty() {
		return
	}

	let size_bytes = data.len() * std::mem::size_of::<T>();

	unsafe {
		gl::NamedBufferData(
			handle,
			size_bytes as _,
			data.as_ptr() as *const _,
			usage
		);
	}
}



// NOTE: Must respect glsl std140 layout rules.
// Lucky for us, Mat4 fits this description.
#[repr(C)]
#[derive(Copy, Clone)]
struct Uniforms {
	projection: Mat4,
}


// NOTE: Must respect glsl std430 layout rules.
#[repr(C)]
#[derive(Copy, Clone)]
struct SpriteUniform {
	transform: Mat3A,
	color: Vec4,
	uv_scale: Vec2,
	uv_offset: Vec2,
}



pub fn load_texture(path: impl AsRef<std::path::Path>) -> anyhow::Result<u32> {
	let image = image::open(path)?.flipv().into_rgba8().into_flat_samples();
	let (width, height) = (image.layout.width as i32, image.layout.height as i32);
	let data = image.samples;

	unsafe {
		let mut texture_handle = 0;
		gl::CreateTextures(gl::TEXTURE_2D, 1, &mut texture_handle);

		// Allocate storage
		gl::TextureStorage2D(texture_handle, 1, gl::SRGB8_ALPHA8, width, height);

		// Upload image data
		let (level, offset_x, offset_y) = (0, 0, 0);
		gl::TextureSubImage2D(
			texture_handle,
			level, offset_x, offset_y,
			width, height,
			gl::RGBA,
			gl::UNSIGNED_BYTE,
			data.as_ptr() as *const _
		);

		// Set sampling parameters.
		// If we don't set these we'd need to generate mipmaps, since GL_TEXTURE_MIN_FILTER defaults to GL_NEAREST_MIPMAP_LINEAR
		gl::TextureParameteri(texture_handle, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
		gl::TextureParameteri(texture_handle, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

		Ok(texture_handle)
	}
}