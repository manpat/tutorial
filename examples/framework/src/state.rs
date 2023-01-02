use crate::prelude::*;


#[allow(dead_code)]
pub struct State {
	sdl_ctx: sdl2::Sdl,
	sdl_video: sdl2::VideoSubsystem,

	pub(crate) event_pump: sdl2::EventPump,
	pub(crate) window: sdl2::video::Window,

	gl_ctx: sdl2::video::GLContext,
}

impl State {
	pub fn backbuffer_size(&self) -> IVec2 {
		let (w, h) = self.window.drawable_size();
		IVec2::new(w as i32, h as i32)
	}
}


pub(crate) fn init() -> anyhow::Result<State> {
	use anyhow::Error;

	// Initial setup of sdl2
	let sdl_ctx = sdl2::init().map_err(Error::msg)?;
	let sdl_video = sdl_ctx.video().map_err(Error::msg)?;
	let event_pump = sdl_ctx.event_pump().map_err(Error::msg)?;

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

	// Require vsync so we don't have to think about timing as much :)
	sdl_video.gl_set_swap_interval(sdl2::video::SwapInterval::VSync).map_err(Error::msg)?;

	// Finally load our gl functions
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

	Ok(State {
		sdl_ctx,
		sdl_video,
		event_pump,

		window,
		gl_ctx,
	})
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