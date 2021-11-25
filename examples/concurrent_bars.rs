use std::{thread, time};
use std::io::Write;
use pogress_bar::chalk_bar::ChalkProgressBar;

fn main() {
	print!("\x1b[?25l"); // hide cursor

	let mut v: Vec<ChalkProgressBar> = Vec::new();
	for _ in 1..=10 {
		v.push(ChalkProgressBar::new(15, 10));
	}

	for i in 1..=10 {
		if i == 8 {
			println!("\rThis is some info!\x1b[K");
		}
	
		for bar in v.iter_mut() {
			(*bar).update(i).unwrap();
			print!("\n");
			
			thread::sleep(time::Duration::from_millis(5));
		}
		if i < 10 {
			print!("\x1b[{}A", v.len());
		}
	}

	print!("\x1b[?25h"); // show cursor
}