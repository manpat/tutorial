#version 450


// Uniform buffers must have std140 layout.
// `binding` here is the same value passed to glBindBufferBase.
layout(std140, binding=0) uniform Uniforms {
	mat4 u_projection;
};


layout(location=0) in vec3 a_position;
layout(location=1) in vec2 a_uv;

out vec2 v_uv;
out vec4 v_color;


void main() {
	gl_Position = u_projection * vec4(a_position, 1.0);
	v_uv = a_uv;
	v_color = vec4(1.0);
}

