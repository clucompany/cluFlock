use cluFlock::SharedFlock;
use std::fs::File;
use std::io;

fn main() -> Result<(), io::Error> {
	let file = File::create("./test_file")?;

	let shared = SharedFlock::wait_lock(&file);
	println!("#1shared {:?}", shared);
	let shared2 = SharedFlock::try_lock(&file);
	println!("#2shared {:?}", shared2);

	assert!(shared.is_ok());
	assert!(shared2.is_ok());

	// manual or automatic unlock SharedFlock_x2
	// drop(shared);
	// drop(shared2);

	Ok(())
}
