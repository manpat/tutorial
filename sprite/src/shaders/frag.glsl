#version 450

// `binding` here refers to the texture unit, and corresponds to the value passed to `glBindTextureUnit`
layout(binding=0) uniform sampler2D u_texture;


in vec2 v_uv;
in vec4 v_color;

layout(location=0) out vec4 o_color;




const float[] c_bayer_2x2 = float[](0.25, 0.75, 1.0, 0.5);

void dither2x2_discard(float value) {
	ivec2 position = ivec2(gl_FragCoord.xy);
	int x = position.x % 2;
	int y = position.y % 2;
	int index = x + y * 2;

	float limit = c_bayer_2x2[index];

	// + 0.125 (half the distance between bayer thresholds) makes this behave more like a round.
	// the small offset on top of that works around a weird noise seemingly introduced by vertex interpolation?
	// Its very strange and I would like to know why this noise exists, but for now at least this makes it
	// harder to accidentally encounter.
	if (value + 0.1250001 < limit) {
		discard;
	}
}



void main() {
	o_color = texture(u_texture, v_uv) * v_color;
	dither2x2_discard(o_color.a);
}