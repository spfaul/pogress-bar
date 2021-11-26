#![allow(dead_code, unused_variables)]
use terminal_size::{terminal_size, Height, Width};
use std::io::Write;
use colored::*;
use crate::{errors, ProgressBar};

#[derive(Debug, Copy, Clone)]
pub struct AptProgressBar {
	length: u32,
	max_range: u32,
	curr_val: u32
}

impl ProgressBar for AptProgressBar {
	fn new(length: u32, max_range: u32) -> Self {
		return AptProgressBar {
			length,
			max_range,
			curr_val: 0
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
		let (w, h): (u16, u16) = AptProgressBar::get_term_size()?;
		let complete_length: u32 = ((self.curr_val as f32 / self.max_range as f32 * (self.length - 2) as f32).floor()) as u32;
		
		print!("\x1b[s");
		print!("\x1b[{};0f", h);

		let prog_percentage: f32 = (self.curr_val as f32 / self.max_range as f32 * 100 as f32).floor();
		let prog_percentage_str: String = format!("\rProgress: {}%", prog_percentage);
		print!("{}", prog_percentage_str.black().on_green()); 
		print!("{}", " ".repeat((4-prog_percentage.to_string().len()) as usize)); // ensure absolute positioning of bar
		print!("[{}{}]\x1b[K", "#".repeat(complete_length as usize), ".".repeat((self.length-2-complete_length) as usize));

		print!("\x1b[u");
		std::io::stdout().flush().unwrap();

		Ok(())
	}
		

	fn update(&mut self, new_val: u32) -> Result<(), errors::ProgBarError> {
		self.set_value(new_val)?;
		self.draw()?;
		
		Ok(())
	}
}

impl AptProgressBar {
	pub fn init() -> Result<(), errors::ProgBarError> {
		let (w, h) = AptProgressBar::get_term_size()?;
		
		print!("\x1b[s");
		print!("\x1b[0;{}r", h-1);
		print!("\x1b[u");
		print!("\x1b[1A");

		Ok(())
	}
	
	fn get_term_size() -> Result<(u16, u16), errors::ProgBarError> {
		if let Some((Width(w), Height(h))) = terminal_size() {
			return Ok((w, h));
		}
		
		Err(errors::ProgBarError::TermSizeUnknown)		
	}
	
	pub fn cleanup() -> Result<(), errors::ProgBarError> {
		let (w, h) = AptProgressBar::get_term_size()?;
		
		print!("\x1b[s");
		print!("\x1b[0;{}r", h);
		print!("\x1b[{};0f", h);
		print!("\x1b[0K");
		print!("\x1b[u");

		Ok(())
	}
	
}





