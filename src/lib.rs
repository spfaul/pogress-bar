pub mod progress_bar;
pub mod cargo_bar;
pub mod apt_bar;
pub mod chalk_bar;
pub mod errors;

pub fn print(text: String) {
	print!("\r{}\x1b[K", text); // clear line and print
}

pub fn println(text: String) {
	println!("\r{}\x1b[K", text); // clear line and print
}
