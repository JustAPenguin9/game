use ratatui::Frame;

use crate::event::{Event, UpdateEvent};

pub enum Action<T> {
	Continue,
	End,
	NewScene(Box<dyn Scene<T>>),
}

pub trait Scene<T> {
	fn title(&self) -> String;

	/// self should never be mutated when drawn
	/// this is just here for stateful widgets
	fn draw(&mut self, global: &T, frame: &mut Frame);

	fn update(&mut self, global: &mut T, event: UpdateEvent) -> Action<T>;
}
