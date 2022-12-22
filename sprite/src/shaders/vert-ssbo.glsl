#version 450


// Uniform buffers must have std140 layout.
// `binding` here is the same value passed to glBindBufferBase.
layout(std140, binding=0) uniform Uniforms {
	mat4 u_projection;
};


struct Sprite {
	mat3 transform;
	vec4 color;
	vec2 uv_scale;
	vec2 uv_offset;
};

layout(std430, binding=0) buffer Sprites {
	Sprite u_sprites[];
};


out vec2 v_uv;
out vec4 v_color;

const vec2[4] g_uvs = {
	{0.0, 0.0},
	{1.0, 0.0},
	{1.0, 1.0},
	{0.0, 1.0},
};

const uint g_indices[6] = {0, 1, 2, 0, 2, 3};

void main() {
	const uint sprite_index = gl_VertexID / 6;
	const Sprite sprite = u_sprites[sprite_index];

	const uint index = g_indices[gl_VertexID % 6];
	const vec2 uv = g_uvs[index];
	const vec2 local_pos = uv - vec2(0.5);

	const vec3 world_pos = sprite.transform * vec3(local_pos, 1.0);

	gl_Position = u_projection * vec4(world_pos, 1.0);
	v_uv = sprite.uv_scale * uv + sprite.uv_offset;
	v_color = sprite.color;
}

