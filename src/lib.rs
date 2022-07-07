pub mod progress_bar;
pub mod cargo_bar;
pub mod apt_bar;
pub mod chalk_bar;
pub mod errors;

pub fn print(text: String) {
	print!("\r{}\x1b[K", text); // clear line and print
}

pub fn println(text: String) {
	println!("\r{}\x1b[K", text); // clear line and print
}

pub trait ProgressBar {
	fn new(length: u32, max_range: u32) -> Self;

	fn set_value(&mut self, new_val: u32) -> Result<(), errors::ProgBarError>;

	fn draw(&self) -> Result<(), errors::ProgBarError>;

	fn update(&mut self, new_val: u32) -> Result<(), errors::ProgBarError> {
		self.set_value(new_val)?;
		self.draw()?;
		Ok(())
	}
}

pub struct ChalkProgressBarGroup {
	bars: Vec<chalk_bar::ChalkProgressBar>,
	curs_pos: i16
}

impl ChalkProgressBarGroup {
	pub fn new(num_bars: u32, bar_length: u32, bar_range_max: u32) -> ChalkProgressBarGroup {
		ChalkProgressBarGroup {
			bars: vec![<chalk_bar::ChalkProgressBar>::new(bar_length, bar_range_max); num_bars as usize],
			curs_pos: 0
		}
	}

	pub fn draw(&mut self) {
		for bar in self.bars.iter_mut() {
			(*bar).draw().unwrap();
			println!();
		}
		self.curs_pos += self.bars.len() as i16;
	}

	pub fn update(&mut self, idx: usize, val: u32) {
		self.move_cursor_down(idx as i16 - self.curs_pos);
		self.bars[idx].update(val).unwrap();
		self.move_cursor_down(self.bars.len() as i16 - self.curs_pos);
	}

	pub fn move_cursor_down(&mut self, lines: i16) {
		self.curs_pos += lines;
		if lines < 0 {
	
		print!("\x1b[{}A", -lines);
		} else if lines > 0 {
			print!("\x1b[{}B", lines);
		}
	}

	pub fn println(&mut self, text: String) {
		self.move_cursor_down(-self.curs_pos);
		println!("\r{}\x1b[K", text);
		self.draw();
	}
}
