#![allow(dead_code, unused_variables)]
use terminal_size::{terminal_size, Height, Width};
use std::{thread, time};

mod cargo_bar;
mod errors;

fn main() {
	print!("\x1b[?25l"); // hide cursor

	cargo_style_bar();
	
	print!("\x1b[?25h"); // show cursor
}

fn cargo_style_bar() {
	let mut bar = cargo_bar::CargoProgressBar::new(15, 10);
	for num in 0..=10 {
		bar.update(num).unwrap();
		thread::sleep(time::Duration::from_millis(200));
	}
}

// WIP, just ignore
fn apt_prog_bar()
{
	let term_size = terminal_size();
	if let Some((Width(w), Height(h))) = term_size
	{
		print!("\n");
		print!("\x1b[s");
		print!("\x1b[0;{}r]", h-1);
		print!("\x1b[u");
		print!("\x1b[1A");

		for i in 1..=5
		{
			print!("Hello {}", i);
			print!("\x1b[s");
			print!("\x1b[{};0f", h);
			print!("This is the bottom margin!");
			print!("\x1b[u");
			print!("\n");
			thread::sleep(time::Duration::from_millis(500));
		}

		print!("\x1b[s");
		print!("\x1b[0;{}r", h);
		print!("\x1b[{};0f]", h);
		print!("\x1b[0K");
		print!("\x1b[u");
	}
	else
	{
		panic!("Can't get terminal width and height!");
	}
}