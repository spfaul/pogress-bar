use std::{thread, time};
use pogress_bar::cargo_bar;

fn main() {
	print!("\x1b[?25l"); // hide cursor

	let mut bar = cargo_bar::CargoProgressBar::new(13, 10);
	for num in 0..=10 {
		if num == 8 {
			println!("\rThis is some info!\x1b[K");
		}
		bar.update(num).unwrap();
		thread::sleep(time::Duration::from_millis(100));
	}
	
	print!("\x1b[?25h"); // show cursor
}



