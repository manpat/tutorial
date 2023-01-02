use framework::prelude::*;


fn main() {
	framework::run("ch01 - speedrun a rectangle", Example::new);
}



struct Example {
	main_shader: u32,
}

impl Example {
	fn new() -> anyhow::Result<Example> {
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

		Ok(Example {
			main_shader,
		})
	}
}


impl framework::App for Example {
	fn draw(&mut self, state: &framework::State) {
		unsafe {
			let size = state.backbuffer_size();
			gl::Viewport(0, 0, size.x, size.y);

			gl::ClearColor(0.1, 0.1, 0.1, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT);

			// ANCHOR: final_render
			// This corresponds to the number of indices in our vertex shader.
			let num_vertices = 6;

			gl::UseProgram(self.main_shader);
			gl::DrawArrays(gl::TRIANGLES, 0, num_vertices);
			// ANCHOR_END: final_render
		}
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
