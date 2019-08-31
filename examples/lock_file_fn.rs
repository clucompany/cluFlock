
use std::io::Write;
use std::io;
use std::fs::OpenOptions;
use cluFlock::ExclusiveFlock;

fn main() -> Result<(), io::Error> {

	//Two and more applications consistently write down data in the file.
	
	let program_pid = unsafe{ libc::getpid() };
	println!("[{}] Init...", program_pid);
	

	let mut file = OpenOptions::new().write(true).append(true).create(true).open("/tmp/async_file")?;
	let mut metadata = file.metadata()?;

	let mut new_len;
	let mut old_len = metadata.len();
	'begin: for num in 0..200 {
		println!("[{}][{}] Wait Mod file, {:?}", program_pid, num, file);
		while old_len == {new_len = {metadata = file.metadata()?; metadata.len()}; new_len} {
			#[allow(deprecated)]
			::std::thread::sleep_ms(200);
		}
		
		println!("[{}][{}] WaitLock file, {:?}", program_pid, num, file);

		
		let result = ExclusiveFlock::wait_lock_fn(&mut file, |mut file| {
			println!("[{}][{}] Write file, {:?}", program_pid, num, file);

			let result = match write!(file, "[{}][{}]{}->{}\n", program_pid, num, old_len, new_len) {
				Ok(a) => {
					file.sync_all()?; 

					Ok(a)
				},
				a => a,
			};

			new_len = {metadata = file.metadata()?; metadata.len()};
			old_len = new_len;

			result
		})?;

		//Alternative
		/*
			let result = (&mut file).wait_exclusive_lock_fn(|mut file| {
					write!(file, "123")
			})?;
		*/

		println!("{:?}", result);
	}

	Ok( () )
}

