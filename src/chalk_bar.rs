#![allow(dead_code, unused_variables)]
use std::io::Write;
use colored::*;
use crate::{errors, ProgressBar};

#[derive(Copy, Clone, Debug)]
pub struct ChalkProgressBar {
	length: u32,
	max_range: u32,
	curr_val: u32
}

impl ProgressBar for ChalkProgressBar {
	fn new(length: u32, max_range: u32) -> Self {
		return ChalkProgressBar {
			length,
			max_range,
			curr_val: 0,
		}
	}

	fn set_value(&mut self, new_val: u32) -> Result<(), errors::ProgBarError> {
		if new_val <= self.max_range {
			self.curr_val = new_val;
			Ok(())
		} else {
			Err(errors::ProgBarError::ValueOutOfBounds)
		}
	}

	fn draw(&self) -> Result<(), errors::ProgBarError> {
		let prog_percentage: f32 = (self.curr_val as f32 / self.max_range as f32 * 100 as f32).floor();
		let tail_length: u32 = (self.curr_val as f32 / self.max_range as f32 * (self.length-2) as f32).floor() as u32;
		let bar_string: String = format!("\r|{}{}| {}%\x1b[K", " ".repeat(tail_length as usize).on_white(), " ".repeat((self.length-tail_length-2) as usize), prog_percentage);
		print!("{}", bar_string);
		std::io::stdout().flush().unwrap();

		Ok(())
	}
}
