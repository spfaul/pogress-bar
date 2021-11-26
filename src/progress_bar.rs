use crate::errors;

pub trait ProgressBar {
	fn new(length: u32, max_range: u32) -> Self;

	fn set_value(&mut self, new_val: u32) -> Result<(), errors::ProgBarError>;

	fn draw(&self) -> Result<(), errors::ProgBarError>;

	fn update(&mut self, new_val: u32) -> Result<(), errors::ProgBarError> {
		self.set_value(new_val)?;
		self.draw()?;
		Ok(())
	}
}