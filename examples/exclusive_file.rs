use cluFlock::ToFlock;
use std::fs::File;
use std::io;

fn main() -> Result<(), io::Error> {
	let file_lock = File::create("./file")?.wait_exclusive_lock()?;
	println!("{:?}", file_lock);

	Ok(())
}

/*
/usr/bin/flock -w 600 ./1 /bin/bash -c "echo Start; sleep 5; echo End; "
cargo run --example use
*/
