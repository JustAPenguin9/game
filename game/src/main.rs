use std::{error::Error, io::stdout};

use engine::{
	self,
	app::{setup, teardown, App},
	CrosstermBackend, KeyCode, KeyEvent, KeyModifiers, Terminal,
};

mod scenes;
use scenes::*;

struct Data {
	up: KeyEvent,
	down: KeyEvent,
	left: KeyEvent,
	right: KeyEvent,
	forward: KeyEvent,
	back: KeyEvent,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	use KeyCode::*;
	let mut app = App::new(
		50,
		50,
		Data {
			up: KeyEvent::new(Up, KeyModifiers::NONE),
			down: KeyEvent::new(Down, KeyModifiers::NONE),
			left: KeyEvent::new(Left, KeyModifiers::NONE),
			right: KeyEvent::new(Right, KeyModifiers::NONE),
			forward: KeyEvent::new(Char('x'), KeyModifiers::NONE),
			back: KeyEvent::new(Char('z'), KeyModifiers::NONE),
		},
	);
	let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

	app.scenes.push(Box::new(TitleScreen::default()));

	setup()?;
	let status = app.start(terminal).await;
	teardown()?;

	status
}
