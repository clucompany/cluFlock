
use std::io::Write;
use std::fs::OpenOptions;
use cluFlock::ExclusiveFlock;

fn main() -> Result<(), std::io::Error> {
	//Two and more applications consistently write down data in the file.
	
	let program_pid = platform_methods::get_pid();
	println!("[{}] Init...", program_pid);
	

	let mut file = OpenOptions::new().write(true).append(true).create(true).open("./async_file")?;
	let mut metadata = file.metadata()?;

	let mut new_len;
	let mut old_len = metadata.len();
	for num in 0..200 {
		println!("[{}][{}] Wait Mod file, {:?}", program_pid, num, file);
		while old_len == {new_len = {metadata = file.metadata()?; metadata.len()}; new_len} {
			#[allow(deprecated)]
			::std::thread::sleep_ms(200);
		}
		
		println!("[{}][{}] WaitLock file, {:?}", program_pid, num, file);

		
		let result = ExclusiveFlock::wait_lock_fn(&mut file, 
			|mut file| {
				println!("[{}][{}] Write file, {:?}", program_pid, num, file);
				
				if let Err(e) = write!(file, "[{}][{}]{}->{}\n", program_pid, num, old_len, new_len) {
					return Err(e);
				}
				
				if let Err(e) = file.sync_all() {
					return Err(e);
				}
				
				metadata = file.metadata()?;
				new_len = metadata.len();
				old_len = new_len;

				Ok(())
			},
			|err| Err(err.into_err())
		)?;

		println!("{:?}", result);
	}

	Ok( () )
}

pub mod platform_methods {
	#[inline(always)]
	#[cfg(unix)]
	pub fn get_pid() -> i32 {
		unsafe { libc::getpid() }
	}

	#[inline(always)]
	#[cfg(not(unix))]
	pub fn get_pid() -> i32 {
		0
	}
}

