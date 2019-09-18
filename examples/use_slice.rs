
use cluFlock::ExclusiveFlock;
use std::fs::File;

fn main() -> Result<(), std::io::Error> {
	let file = File::create("./file")?;
	
	{
		let file_lock = ExclusiveFlock::wait_lock(&file)?;
		//lock...

		println!("{:?}", file_lock);
	}

	file.sync_all()?;

	Ok( () )
}