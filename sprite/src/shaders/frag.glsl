#version 450

in vec2 v_uv;

out vec4 o_color;

void main() {
	o_color = vec4(v_uv, 0.0, 1.0);	
}