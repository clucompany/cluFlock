
/*
	Using only the file pointer for locks. Unsafe is required since it's not 
	a complete file object, just a raw pointer to it.
*/

use cluFlock::SharedFlock;
use cluFlock::rawfile::GetRawFile;
use std::io::Error;
use std::fs::File;

fn main() -> Result<(), Error> {
	let file = File::create("./test_file")?;
	println!("{:?}", file);
	
	let file_ptr = unsafe {
		file.get_raw_file()
	};
	println!("FilePtr0: {:?}", file_ptr);
	let shared0 = SharedFlock::try_lock(file_ptr)?;
	println!("SharedFlock: {:?}", shared0);
	
	drop(file); // <-- Delete the file, note that the lock is still alive,
	// although this is not possible in safe mode.
	
	println!("FilePtr0: {:?}", file_ptr);
	println!("SharedFlock: {:?}", shared0);
	
	let shared1 = SharedFlock::try_lock(file_ptr);
	assert_eq!(shared1.is_err(), true);
	println!("{:?}", shared1); // <-- "Invalid file descriptor" 
	// error because the file was indeed previously closed, 
	// but since we created a RawFile we were able to access that address.
	
	// Case2: random file address
	#[cfg(any(linux, unix, bsd))] {
		use cluFlock::rawfile::RawFile;
		
		let raw_file = unsafe {
			RawFile::from_ptr(2)
		};
		println!("Case2 {:?}", raw_file);
		let shared2 = SharedFlock::try_lock(raw_file);
		println!("{:?}", shared2);
		/* 
			It's funny that you can block input and output streams.
		*/
	}
	
	Ok(())
}
