use ratatui::Frame;

use crate::{app::App, event::Event};

pub enum Action<T> {
	Continue,
	End,
	NewScene(Scene<T>),
}

pub struct Scene<T> {
	pub title: String,
	pub draw_count: u32,
	pub update_count: u32,
	pub draw: fn(&mut Frame),
	pub update: fn(Event, app: &mut App<T>) -> Action<T>,
}
impl<T> Scene<T> {
	pub fn incr_draw(&mut self) {
		self.draw_count += 1;
	}

	pub fn incr_update(&mut self) {
		self.update_count += 1;
	}
}
