# cluFlock

[![Build Status](https://travis-ci.org/clucompany/cluFlock.svg?branch=master)](https://travis-ci.org/clucompany/cluFlock)
[![Apache licensed](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](./LICENSE)
[![crates.io](http://meritbadge.herokuapp.com/cluFlock)](https://crates.io/crates/cluFlock)
[![Documentation](https://docs.rs/cluFlock/badge.svg)](https://docs.rs/cluFlock)

Control of lock of the file using the 'flock' functions.


# Capabilities

1. Convenient and transparent trait of a call of locks.
2. Automatic unlocking of lock.

# Locks

1. ExclusiveFlock - To establish exclusive blocking. Only one process can hold exclusive blocking of the file..
2. SharedFlock - Set a shared lock. A shared lock on a given file can hold more than one process.


# Use
1. Exclusive LockFile


	extern crate cluFlock;

	use cluFlock::ToFlock;
	use std::fs::File;
	use std::io;

	fn main() -> Result<(), io::Error> {
		let file_lock = File::create("/tmp/1")?.wait_exclusive_lock()?;

		println!("{:?}", file_lock);
		
		drop(file_lock); //<-- unlock fn.

		Ok( () )
	}



2. Exclusive LockClosure


	extern crate cluFlock;

	use std::io::Write;
	use cluFlock::ToFlock;
	use std::fs::File;
	use std::io;

	fn main() -> Result<(), io::Error> {
		File::create("/tmp/1")?.wait_exclusive_lock_fn(|mut file| {
			write!(file,  "Test.")
		})??;

		Ok( () )
	}


3. The temporary file for interprogram synchronization


	extern crate cluFlock;

	use cluFlock::ToFlock;
	use cluFlock::FileFlock;
	use std::io::ErrorKind::AlreadyExists;
	use std::path::Path;
	use std::fs;
	use std::io;
	use std::fs::OpenOptions;

	//Example
	//The temporary file for interprogram synchronization.


	#[derive(Debug)]
	pub struct MyLockFile<'a>(FileFlock, Option<&'a Path>);

	impl<'a> MyLockFile<'a> {
		pub fn new(p: &'a Path) -> Result<Self, io::Error> {
			let (lock, path) = match OpenOptions::new().write(true).create_new(true).open(p) {
				Ok(file) => (file.wait_exclusive_lock()?, Some(p)),
				Err(ref e) if e.kind() == AlreadyExists => {
					let f = OpenOptions::new().read(true).open(p)?; 

					let lock = f.try_exclusive_lock()?;

					(lock, None)
				},
				Err(e) => return Err(e),
			};

			Ok( MyLockFile(lock, path) )
		}
	}

	impl<'a> Drop for MyLockFile<'a> {
		fn drop(&mut self) {
			//Not obligatory action.
			//

			//Not to delete the file if it initially existed.

			if let Some(path) = self.1 {
				let _e = fs::remove_file(path);
			}
		}
	}


	pub fn main() -> Result<(), io::Error> {
		let path = Path::new("/tmp/flock.lock");
		println!("LockFile {:?}", path);
		let lock_file = MyLockFile::new(path)?;

		println!("OK! FileFlock {:?}", lock_file);
		for a in 0..4 {
			println!("Sleep {}", a);
			::std::thread::sleep(::std::time::Duration::from_secs(1));
		}

		drop(lock_file);

		Ok( () )
	}




# License

Copyright 2018 #UlinProject Денис Котляров

Licensed under the Apache License, Version 2.0
