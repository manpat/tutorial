#[allow(dead_code)]
pub fn main() {
// ANCHOR: just_window
	// These two calls are more or less equivalent to `SDL_Init(SDL_INIT_VIDEO)`
	let sdl_ctx = sdl2::init().expect("SDL init failed");
	let sdl_video = sdl_ctx.video().expect("video subsystem init failed");

	let _window = sdl_video.window("Tutorial", 1366, 768)
		.position_centered()
		.build()
		.expect("Failed to create window");

	std::thread::sleep(std::time::Duration::from_secs(3));
// ANCHOR_END: just_window
}
