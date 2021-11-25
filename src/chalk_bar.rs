#![allow(dead_code, unused_variables)]
use std::io::Write;
use colored::*;
use crate::errors;

#[derive(Copy, Clone, Debug)]
pub struct ChalkProgressBar {
	length: u32,
	max_range: u32,
	curr_val: u32
}

impl ChalkProgressBar {
	pub fn new(length: u32, max_range: u32) -> Self {
		return ChalkProgressBar {
			length,
			max_range,
			curr_val: 0,
		}
	}

	pub fn set_value(&mut self, new_val: u32) -> Result<(), errors::ProgBarError> {
		if new_val <= self.max_range {
			self.curr_val = new_val;
			Ok(())
		} else {
			Err(errors::ProgBarError::ValueOutOfBounds)
		}
	}

	pub fn draw(&mut self) {
		let prog_percentage: f32 = (self.curr_val as f32 / self.max_range as f32 * 100 as f32).floor();
		let tail_length: u32 = (self.curr_val as f32 / self.max_range as f32 * (self.length-2) as f32).floor() as u32;
		let bar_string: String = format!("\r|{}{}| {}%", " ".repeat(tail_length as usize).on_white(), " ".repeat((self.length-tail_length-2) as usize), prog_percentage);
		print!("{}", bar_string);
		std::io::stdout().flush().unwrap();
	}

	pub fn update(&mut self, new_val: u32) -> Result<(), errors::ProgBarError> {
		self.set_value(new_val)?;
		self.draw();
		Ok(())
	}
}
