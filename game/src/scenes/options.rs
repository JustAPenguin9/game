use engine::{
	event::*,
	ratatui::widgets::*,
	scene::{Action, Scene},
};

use crate::Data;

#[derive(Default)]
pub struct Options {}

impl Scene<Data> for Options {
	fn title(&self) -> String {
		"options".into()
	}

	fn draw(&mut self, _global: &Data, frame: &mut engine::ratatui::prelude::Frame) {
		frame.render_widget(Paragraph::new("options"), frame.size());
	}

	fn update(&mut self, global: &mut Data, event: UpdateEvent) -> Action<Data> {
		match event {
			UpdateEvent::Key(k) => {
				if k == global.back {
					return Action::End;
				}
			}
			_ => {}
		}

		Action::Continue
	}
}
