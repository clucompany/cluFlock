
use std::fs::File;
use cluFlock::SharedFlock;
use std::io;

fn main() -> Result<(), io::Error> {
	let file = File::create("./test_file")?;
	
	let shared = SharedFlock::wait_lock(&file);
	println!("#1shared {:?}", shared);
	let shared2 = SharedFlock::try_lock(&file);
	println!("#2shared {:?}", shared2);
	
	assert_eq!(shared.is_ok(), true);
	assert_eq!(shared2.is_ok(), true);
	
	// manual or automatic unlock SharedFlock_x2
	// drop(shared);
	// drop(shared2);
	
	Ok( () )
}
