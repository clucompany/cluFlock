use cluFlock::ExclusiveFlock;
use std::fs::File;
use std::time::Duration;

fn main() {
	let file: File = match File::create("./test_file") {
		Ok(a) => a,
		Err(e) => panic!("Panic, err create file {:?}", e),
	};

	println!("Try_Exclusive_Lock, {:?}", file);
	let lock = match ExclusiveFlock::try_lock(&file) {
		//Success, we blocked the file.
		Ok(lock) => {
			println!("OK, File {:?} successfully locked.", file);

			lock
		}
		// File already locked.
		Err(ref e) if e.is_already_lock() => {
			println!("ALREADY LOCKED: File {:?}.", file);

			println!("!Exclusive_Lock, {:?}", file);

			// Lock the current thread to such an extent until your file is unlocked.
			// &file.wait_exclusive_lock().unwrap()
			ExclusiveFlock::wait_lock(&file).unwrap()
		}
		Err(e) => panic!("Panic, err lock file {:?}", e),
	};

	println!("Sleep, 5s");
	std::thread::sleep(Duration::from_secs(5));

	println!("Unlock, {:?}", file);
	drop(lock);
}
