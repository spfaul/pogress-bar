use std::{thread, time};
use colored::*;
use pogress_bar::{ProgressBar, apt_bar, println};

fn main() {
	let mut bar = apt_bar::AptProgressBar::new(15, 10);
	apt_bar::AptProgressBar::init().unwrap();
	
	// ensure apt bar is visible at the bottom 
	println!("");
	
	for num in 0..=10 {
		if num == 8 {
			println("This is some info!".to_string());
		}
		bar.update(num).unwrap();
		thread::sleep(time::Duration::from_millis(100));
	}
	apt_bar::AptProgressBar::cleanup().unwrap();
}
