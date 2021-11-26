use std::{thread, time};
use pogress_bar::{cargo_bar, println, ProgressBar};

fn main() {
	let mut bar = cargo_bar::CargoProgressBar::new(13, 10);
	for num in 0..=10 {
		if num == 8 {
			println("This is some info!".to_string());
		}
		bar.update(num).unwrap();
		thread::sleep(time::Duration::from_millis(100));
	}
}



