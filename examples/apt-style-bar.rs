use std::{thread, time};
use pogress_bar::apt_bar;

fn main() {
	let mut bar = apt_bar::AptProgressBar::new(15, 10);
	bar.init().unwrap();
	println!(""); // ensure bar is visible
	for num in 0..=10 {
		if num == 8 {
			println!("This is some info!"); // note that escape codes are not needed
		}
		bar.update(num).unwrap();
		thread::sleep(time::Duration::from_millis(100));
	}
	bar.cleanup().unwrap();
}
