
use cluFlock::ExclusiveFlock;
use std::fs::File;

fn main() -> Result<(), std::io::Error> {
	let file = File::create("./file")?;
	
	{
		let file_lock = ExclusiveFlock::wait_lock(&file)?;
		// file_lock, type: FlockLock<&File>

		println!("{:?}", file_lock);
	} // auto unlock ExclusiveFlock

	file.sync_all()?;

	Ok( () )
}