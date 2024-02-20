use std::error::Error;
use std::io::stdout;
use std::{
	io::Stdout,
	time::{Duration, Instant},
};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::execute;
use ratatui::widgets::Clear;
use ratatui::widgets::Paragraph;
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::event::EventHandler;
use crate::event::{Event, UpdateEvent};
use crate::scene::Action;
use crate::scene::Scene;

pub struct App<T> {
	active: bool,
	pub global: T,
	start_time: Instant,
	pub tick_rate: Duration,
	pub render_rate: Duration,
	// u64 is kinda unnecessary
	tick_count: u64,
	render_count: u64,
	pub scenes: Vec<Box<dyn Scene<T>>>,
}

impl<T> App<T> {
	pub fn new(tick_rate: u64, render_rate: u64, data: T) -> Self {
		Self {
			active: true,
			global: data,
			start_time: Instant::now(),
			tick_rate: Duration::from_millis(tick_rate),
			render_rate: Duration::from_millis(render_rate),
			tick_count: 0,
			render_count: 0,
			scenes: vec![],
		}
	}

	pub async fn start(
		mut self,
		mut terminal: Terminal<CrosstermBackend<Stdout>>,
	) -> Result<(), Box<dyn Error>> {
		let mut events = EventHandler::new(self.tick_rate, self.render_rate);

		while self.active {
			match events.next().await {
				// TODO: panic
				Event::Error => panic!(),
				Event::RenderEvent(event) => {
					self.render_count += 1;
					// render
					terminal.draw(|f| {
						// base scene
						f.render_widget(
							Paragraph::new(format!(
								"ticks: {}, renders: {}",
								self.tick_count, self.render_count
							)),
							f.size(),
						);

						// top scene
						if let Some(scene) = self.scenes.last_mut() {
							// clear screen
							f.render_widget(Clear, f.size());
							// render scene
							scene.draw(&self.global, f);
						}
					})?;
				}
				Event::UpdateEvent(event) => {
					// quit
					if let UpdateEvent::Key(k) = event {
						if k == KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL) {
							self.active = false;
						}
					}

					self.tick_count += 1;

					// update
					if let Some(scene) = self.scenes.last_mut() {
						match scene.update(&mut self.global, event) {
							Action::Continue => {}
							Action::End => {
								// there will always be a scene to pop
								// so ignore option
								self.scenes.pop();
							}
							Action::NewScene(new_scene) => {
								self.scenes.push(new_scene);
							}
						}
					}
				}
			}
		}

		Ok(())
	}
}

// TODO: remove all the box dyn errors
pub fn setup() -> Result<(), Box<dyn Error>> {
	execute!(
		stdout(),
		crossterm::terminal::EnterAlternateScreen,
		crossterm::event::EnableMouseCapture,
		// crossterm::event::EnableBracketedPaste,
	)?;
	crossterm::terminal::enable_raw_mode()?;
	Ok(())
}

pub fn teardown() -> Result<(), Box<dyn Error>> {
	execute!(
		stdout(),
		crossterm::terminal::LeaveAlternateScreen,
		crossterm::event::DisableMouseCapture,
		// crossterm::event::DisableBracketedPaste,
	)?;
	crossterm::terminal::disable_raw_mode()?;
	Ok(())
}
