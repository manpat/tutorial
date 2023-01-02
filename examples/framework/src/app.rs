use crate::State;


pub trait App {
	fn update(&mut self, _: &State) {}
	fn draw(&mut self, _: &State) {}
}



pub fn run<A>(name: &str, make_app: impl FnOnce() -> anyhow::Result<A>)
	where A: App
{
	let mut state = crate::state::init(name).expect("init failed");
	let mut app = make_app().unwrap();

	'main: loop {
		for event in state.event_pump.poll_iter() {
			use sdl2::event::Event;
			use sdl2::keyboard::Scancode;

			match event {
				Event::Quit {..} | Event::KeyDown { scancode: Some(Scancode::Escape), .. } => {
					break 'main
				}

				_ => {}
			}
		}


		app.update(&state);
		app.draw(&state);

		state.window.gl_swap_window();
	}
}
