	print!("\x1b[s"); // save curs pos

	for i in 1..=10 {
		for (idx, bar) in v.iter_mut().enumerate() {
			print!("\x1b[u"); // restore curs pos
			print!("\x1b[{}B", idx); // go down idx lines
			std::io::stdout().flush();
			
			(*bar).update(i).unwrap();
			print!("\n");

			print!("\x1b[s"); // save curs pos

			thread::sleep(time::Duration::from_millis(200));
		}
	}
