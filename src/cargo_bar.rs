#![allow(dead_code, unused_variables)]
use std::io::Write;
use crate::errors;

pub struct CargoProgressBar {
	length: i32,
	max_range: i32,
	curr_val: i32
}

impl CargoProgressBar {
	pub fn new(length: i32, max_range: i32) -> Self {
		return CargoProgressBar {
			length,
			max_range,
			curr_val: 0,
		}
	}

	pub fn set_value(&mut self, new_val: i32) -> Result<(), errors::ProgBarError> {
		if new_val >= 0 && new_val <= self.max_range {
			self.curr_val = new_val;
			Ok(())
		} else {
			Err(errors::ProgBarError::ValueOutOfBounds)
		}
	}

	pub fn update(&mut self, new_val: i32) -> Result<(), errors::ProgBarError> {
		self.set_value(new_val)?;

		let tail_length = (self.curr_val as f32 / self.max_range as f32 * (self.length-3) as f32).floor() as i32;
		let bar_string: String = format!("\r|{}>{}| {}", "=".repeat(tail_length as usize), " ".repeat((self.length-tail_length-3) as usize), self.curr_val);
		print!("{}", bar_string);
		std::io::stdout().flush().unwrap();

		Ok(())	
	}
}
