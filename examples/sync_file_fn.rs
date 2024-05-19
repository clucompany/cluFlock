use cluFlock::ExclusiveFlock;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::Duration;

fn main() -> Result<(), std::io::Error> {
	//Two and more applications consistently write down data in the file.

	let program_pid = platform_methods::get_pid();
	println!("[{}] Init...", program_pid);

	let mut file = OpenOptions::new()
		.append(true)
		.create(true)
		.open("./async_file")?;
	let mut metadata = file.metadata()?;

	let mut new_len;
	let mut old_len = metadata.len();
	for num in 0..200 {
		println!("[{}][{}] Wait Mod file, {:?}", program_pid, num, file);
		while old_len == {
			new_len = {
				metadata = file.metadata()?;
				metadata.len()
			};
			new_len
		} {
			std::thread::sleep(Duration::from_millis(200));
		}

		println!("[{}][{}] WaitLock file, {:?}", program_pid, num, file);

		let result = ExclusiveFlock::wait_lock_fn(
			&mut file,
			|mut file| {
				println!("[{}][{}] Write file, {:?}", program_pid, num, file);

				writeln!(file, "[{}][{}]{}->{}", program_pid, num, old_len, new_len)?;
				file.sync_all()?;

				metadata = file.metadata()?;
				new_len = metadata.len();
				old_len = new_len;

				Ok(())
			},
			|err| Err(err.into_err()),
		);

		println!("{:?}", result);
	}

	Ok(())
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
