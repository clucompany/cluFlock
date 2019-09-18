
use std::fs::File;
use cluFlock::SharedFlock;
use std::io;

fn main() -> Result<(), io::Error> {
	let file = File::create("./file")?;
	
	let shared = SharedFlock::wait_lock(&file);
	println!("#1shared {:?}", shared);
	let shared2 = SharedFlock::try_lock(&file)?;
	println!("#2shared {:?}", shared2);
	
	
	Ok( () )
}