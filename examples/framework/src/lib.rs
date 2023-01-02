pub mod prelude {
	pub use anyhow;
	pub use sdl2;
	pub use gl;
	pub use glam;

	pub use glam::IVec2;
}

mod app;
mod state;

pub use app::{App, run};
pub use state::State;

