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

1. Exclusive FileFlock
		
		extern crate cluFlock;

		use cluFlock::Flock;
		use std::fs::File;
		use std::io;

		fn main() -> Result<(), io::Error> {
			let file_lock = File::create("/tmp/1")?.wait_exclusive_lock()?;

			println!("{:?}", file_lock);
			
			drop(file_lock);

			Ok( () )
		}


2. Try SliceFlockFile
		
		extern crate cluFlock;

		use cluFlock::ExclusiveFlock;
		use std::fs::File;
		use std::time::Duration;
		use std::io::ErrorKind;

		fn main() {
			let file: File = match File::create("/tmp/ulin.lock") {
				Ok(a) => a,
				Err(e) => panic!("Panic, err create file {:?}", e),
			};

			println!("Try_Exclusive_Lock, {:?}", file);

			let lock = match ExclusiveFlock::try_lock(&file) {
				//Success, we blocked the file.
				Ok(lock) => {
					println!("File {:?} successfully locked.", file);

					
					lock
				},
				
				//File already locked.
				Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
					println!("ALREADY LOCKED: File {:?}.", file);

					println!("!Exclusive_Lock, {:?}", file);
					
					//Lock the current thread to such an extent until your file is unlocked.
					//&file.wait_exclusive_lock().unwrap()
					ExclusiveFlock::wait_lock(&file).unwrap()
				},
				
				Err(e) => panic!("Panic, err lock file {:?}", e)

			};

			println!("Sleep, 5s");
			::std::thread::sleep(Duration::from_secs(5));

			println!("Unlock, {:?}", file);
			drop(lock);
		}

3. FileFlock (Lock file)

		extern crate cluFlock;

		use cluFlock::Flock;
		use cluFlock::FileFlock;
		use std::io::ErrorKind::AlreadyExists;
		use std::path::Path;
		use std::fs;
		use std::io;
		use std::fs::OpenOptions;

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
