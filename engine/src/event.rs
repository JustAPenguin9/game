use crossterm::event::{Event as CrosstermEvent, KeyEvent, KeyEventKind, MouseEvent};
use futures::{FutureExt, StreamExt};
use std::time::Duration;
use tokio::{
	sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
	task::JoinHandle,
	time::interval,
};

pub enum Event {
	Error,
	Tick,
	Render,
	Key(KeyEvent),
	Mouse(MouseEvent),
	Paste(String),
	Resize(u16, u16),
}

// most (all) of this is taken from the ratatui tutorial and async template
// https://github.com/ratatui-org/templates/blob/main/simple-async/src/event.rs
// https://ratatui.rs/tutorials/counter-async-app/
pub struct EventHandler {
	handler: JoinHandle<()>,
	sender: UnboundedSender<Event>,
	receiver: UnboundedReceiver<Event>,
}

impl EventHandler {
	pub fn new(tick_rate: Duration, render_rate: Duration) -> Self {
		let (sender, receiver) = mpsc::unbounded_channel();
		let tx = sender.clone();
		let handler = tokio::spawn(async move {
			let mut reader = crossterm::event::EventStream::new();
			let mut tick_interval = interval(tick_rate);
			let mut render_interval = interval(render_rate);

			loop {
				let tick_delay = tick_interval.tick();
				let render_delay = render_interval.tick();
				let crossterm_event = reader.next().fuse();

				tokio::select! {
					maybe_event = crossterm_event => {
						match maybe_event {
							Some(Ok(event)) => {
								match event {
									CrosstermEvent::Key(key) => {
										if key.kind == KeyEventKind::Press {
											tx.send(Event::Key(key)).unwrap();
										}
									},
									CrosstermEvent::Mouse(mouse) => {
										tx.send(Event::Mouse(mouse)).unwrap();
									},
									CrosstermEvent::Resize(x, y) => {
										tx.send(Event::Resize(x, y)).unwrap();
									},
									CrosstermEvent::Paste(s) => {
										// doesnt work on windows?
										tx.send(Event::Paste(s)).unwrap();
									},
									// focus gained and lost not needed
									_ => {},
								}
							}
							Some(Err(_)) => {
								// TODO: this should also send the error upstream
								tx.send(Event::Error).unwrap();
							}
							None => {},
						}
					},
					_ = tick_delay => {
						tx.send(Event::Tick).unwrap();
					},
					_ = render_delay => {
						tx.send(Event::Render).unwrap();
					}
				}
			}
		});

		EventHandler { handler, sender, receiver }
	}

	pub async fn next(&mut self) -> Event {
		// TODO: unwrap bad
		self.receiver.recv().await.unwrap()
	}
}
