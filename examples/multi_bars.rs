use std::{thread, time};
use pogress_bar::{ChalkProgressBarGroup};
use rand::{thread_rng, Rng};

fn main() {
	let mut bg: ChalkProgressBarGroup = ChalkProgressBarGroup::new(10, 15, 10);
    let mut rng = thread_rng();

	bg.draw();

	for i in 1..=10 {
        let target_bar: usize = rng.gen_range(0..=9);
        let val: u32 = rng.gen_range(0..=10);

        bg.update(target_bar, val);
		
		if i == 8 {
			bg.println("This is some info".to_string());
		}

		thread::sleep(time::Duration::from_millis(100));
	}
}
