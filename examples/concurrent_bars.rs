use std::{thread, time};
use pogress_bar::{chalk_bar, println};
use pogress_bar::progress_bar::ProgressBar;

fn main() {
	// print!("\x1b[?25l"); // hide cursor

	let mut v: Vec<chalk_bar::ChalkProgressBar> = Vec::new();
	for _ in 1..=10 {
		v.push(chalk_bar::ChalkProgressBar::new(15, 10));
	}

	draw_all(&mut v);

	let mut rel_cursor_height: i16 = 0;

	for i in 1..=10 {
		if i == 3 {
			update_bar(0, &mut v[0], 5, &mut rel_cursor_height);
		}

		if i == 6 {
			update_bar(3, &mut v[3], 6, &mut rel_cursor_height);
		}
		
		if i == 8 {
			// println!("\n{}", rel_cursor_height-(-rel_cursor_height-10));
			move_cursor_down(-rel_cursor_height-10, &mut rel_cursor_height);
			println("This is some info!".to_string());
			draw_all(&mut v);
		}
		
		thread::sleep(time::Duration::from_millis(200));
	}

	// print!("\x1b[?25h"); // show cursor
}

fn update_bar(idx: usize, bar: &mut chalk_bar::ChalkProgressBar, val: u32, curs_height: &mut i16) {
	move_cursor_down((idx as i16 - 10) - (*curs_height), curs_height);
	(*bar).update(val).unwrap();
}

fn draw_all(v: &mut Vec<chalk_bar::ChalkProgressBar>) {
	for bar in v.iter_mut() {
		(*bar).draw().unwrap();
		println!("");
	}
}
	
fn move_cursor_down(lines: i16, curs_height: &mut i16) {
	*curs_height += lines;
	if lines < 0 {
		print!("\x1b[{}A", -lines);
	} else if lines > 0 {
		print!("\x1b[{}B", lines);
	}
}