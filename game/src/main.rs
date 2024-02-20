use std::{collections::HashMap, error::Error, io::stdout};

use engine::{
	self,
	app::{setup, teardown, App},
	event::{Event, UpdateEvent},
	ratatui::widgets::*,
	scene::Scene,
	CrosstermBackend, KeyCode, KeyEvent, KeyModifiers, Terminal,
};

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

	app.scenes.push(Box::new(TitleScreen {
		state: ListState::default(),
		selected: 0,
		list: ["Continue".into(), "New Game".into(), "Options".into()],
	}));

	setup()?;
	let status = app.start(terminal).await;
	teardown()?;

	status
}

struct TitleScreen {
	state: ListState,
	selected: usize,
	// FIXME: this is bad
	list: [String; 3],
}

impl Scene<Data> for TitleScreen {
	fn title(&self) -> String {
		"Title Screen".into()
	}

	fn draw(&mut self, global: &Data, frame: &mut engine::ratatui::prelude::Frame) {
		let list = List::new(self.list.clone())
			.block(Block::default().borders(Borders::ALL))
			.highlight_symbol("> ");

		self.state.select(Some(self.selected.into()));

		frame.render_stateful_widget(list, frame.size(), &mut self.state)
	}

	fn update(&mut self, global: &mut Data, event: UpdateEvent) -> engine::scene::Action<Data> {
		match event {
			UpdateEvent::Key(k) => {
				if k == global.up {
					// FIXME: hard coded numbers
					if self.selected == 0 {
						self.selected = self.list.len() - 1;
					} else {
						self.selected -= 1;
					}
				} else if k == global.down {
					if self.selected == self.list.len() - 1 {
						self.selected = 0;
					} else {
						self.selected += 1;
					}
				}
			}
			_ => {}
		}

		engine::scene::Action::Continue
	}
}
