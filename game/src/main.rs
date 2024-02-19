use std::{error::Error, io::stdout};

use engine::{
	self,
	app::{setup, teardown, App},
	CrosstermBackend, Terminal,
};

struct Data {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let app = App::new(50, 50, Data {});
	let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

	setup()?;
	let status = app.start(terminal).await;
	teardown()?;

	status?;
	Ok(())
}
