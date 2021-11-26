use std::{thread, time};
use pogress_bar::{cargo_bar, println};
use pogress_bar::progress_bar::ProgressBar; // trait

fn main() {
	print!("\x1b[?25l"); // hide cursor

	let mut bar = cargo_bar::CargoProgressBar::new(13, 10);
	for num in 0..=10 {
		if num == 8 {
			println("This is some info!".to_string());
		}
		bar.update(num).unwrap();
		thread::sleep(time::Duration::from_millis(100));
	}
	
	print!("\x1b[?25h"); // show cursor
}



