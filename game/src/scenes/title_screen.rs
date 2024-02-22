use engine::{
	event::*,
	ratatui::{
		layout::{Alignment, Constraint, Flex, Layout},
		widgets::*,
	},
	scene::{Action, Scene},
};

use crate::Data;
use crate::Options;

pub struct TitleScreen<'a> {
	pub state: ListState,
	pub selected: usize,
	pub list: [&'a str; 3],
}

impl Default for TitleScreen<'_> {
	fn default() -> Self {
		Self { state: ListState::default(), selected: 0, list: ["Continue", "New Game", "Options"] }
	}
}

impl Scene<Data> for TitleScreen<'_> {
	fn title(&self) -> String {
		"Title Screen".into()
	}

	fn draw(&mut self, _global: &Data, frame: &mut engine::ratatui::prelude::Frame) {
		let title = "this is a title";
		let options_length = self.list.iter().map(|i| i.chars().count()).max().unwrap_or(0);

		let outer = Layout::horizontal([
			Constraint::Fill(1),
			Constraint::Max(3),
			Constraint::Max(70), // inner
			Constraint::Max(3),
			Constraint::Fill(1),
		])
		.split(frame.size());

		let inner = Layout::vertical([
			Constraint::Fill(1),
			Constraint::Min(3), // title
			Constraint::Max(1),
			Constraint::Max(self.list.len() as u16 + 2), // options
			Constraint::Fill(2),
		])
		.split(outer[2]);

		let options_layout = Layout::horizontal([Constraint::Length(options_length as u16 + 6)])
			.flex(Flex::Center)
			.split(inner[3]);

		let list = List::new(self.list)
			.block(Block::default().borders(Borders::ALL))
			.highlight_symbol("> ");
		self.state.select(Some(self.selected));

		// title
		frame.render_widget(
			Paragraph::new(title.repeat(10))
				.wrap(Wrap { trim: true })
				.block(Block::default())
				.alignment(Alignment::Center),
			inner[1],
		);
		// options
		frame.render_stateful_widget(list, options_layout[0], &mut self.state)
	}

	fn update(&mut self, global: &mut Data, event: UpdateEvent) -> engine::scene::Action<Data> {
		match event {
			UpdateEvent::Key(k) => {
				if k == global.up {
					if self.selected == 0 {
						self.selected = self.list.len() - 1;
					} else {
						self.selected -= 1;
					}
				}
				if k == global.down {
					if self.selected == self.list.len() - 1 {
						self.selected = 0;
					} else {
						self.selected += 1;
					}
				}

				if k == global.forward {
					match self.list[self.selected] {
						"Options" => return Action::NewScene(Box::new(Options::default())),
						_ => {}
					}
				}
			}
			_ => {}
		}

		Action::Continue
	}
}
