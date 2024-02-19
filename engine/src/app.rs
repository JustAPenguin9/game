use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::execute;
use ratatui::widgets::Paragraph;
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::event::Event;
use crate::event::EventHandler;
use crate::scene::Action;
use crate::scene::Scene;

use std::error::Error;
use std::io::stdout;
use std::{
	io::Stdout,
	time::{Duration, Instant},
};

pub struct App<T> {
	active: bool,
	global: T,
	start_time: Instant,
	tick_rate: Duration,
	render_rate: Duration,
	// u64 is kinda unnecessary
	tick_count: u64,
	render_count: u64,
	scenes: Vec<Scene<T>>,
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
				Event::Error => panic!(),
				Event::Render | Event::Resize(_, _) => {
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
							scene.incr_draw();
							(scene.draw)(f);
						}
					})?;
				}
				event => {
					if let Event::Key(k) = event {
						if k == KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL) {
							self.active = false;
						}
					}

					self.tick_count += 1;
					// update
					if let Some(scene) = self.scenes.last_mut() {
						scene.incr_update();
						match (scene.update)(event, &mut self) {
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
