use cluFlock::ToFlock;
use std::fs::File;
use std::io;
use std::io::Write;

fn main() -> Result<(), io::Error> {
	File::create("./file")?.wait_exclusive_lock_fn(
		// valid exclusive lock
		|mut file| write!(file, "Test."), // result: Ok(usize)/Err(std::io::Error)
		// invalid lock
		|err| Err(err.into_err()), // into_err: FlockErr -> std::io::Error
	)?;

	Ok(())
}
