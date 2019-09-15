
use std::io::Write;
use cluFlock::ToFlock;
use std::fs::File;
use std::io;

fn main() -> Result<(), io::Error> {
	File::create("./file")?.wait_exclusive_lock_fn(|mut file| { //let file: cluFlock::UnlockFlock<std::fs::File>
		write!(file,  "Test.")
	})??;
	
	Ok( () )
}
