use std::{thread, time};
use pogress_bar::{ChalkProgressBarGroup};

fn main() {
	let mut bg = ChalkProgressBarGroup::new(10, 15, 10);

	bg.draw();

	for i in 1..=10 {
		if i == 3 {
			bg.update(0, 5); // update bar at index 0 to value 5
		}
		if i == 6 {
			bg.update(3, 6);
		}
		if i == 8 {
			bg.println("This is some info".to_string());
		}

		thread::sleep(time::Duration::from_millis(100));
	}
}
