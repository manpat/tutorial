#version 450

// Our vertex positions - in clip space.
const vec2[4] g_positions = {
	{-0.5,-0.5},
	{ 0.5,-0.5},
	{ 0.5, 0.5},
	{-0.5, 0.5},
};

// Indices into g_positions. Each three indices defines a triangle
// in counter-clockwise winding order.
const uint g_indices[6] = {0, 1, 2, 0, 2, 3};

// The entry point for our shader.
// It will be called once for every vertex.
void main() {
	const uint position_index = g_indices[gl_VertexID];
	const vec2 position = g_positions[position_index];
	gl_Position = vec4(position, 0.0, 1.0);
}
