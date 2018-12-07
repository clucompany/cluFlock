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

1. ExclusiveLock - Only one process can retain exclusive lock of the file.
2. SharedLock - Can retain the general lock on the given file more than one process.

# Use

1. LockFile
		
		extern crate cluFlock;

		use cluFlock::Flock;
		use std::fs::File;

		fn main() {
			let file = File::create("/tmp/1").unwrap();

			let lock = file.exclusive_lock();
			//Only one process can retain exclusive lock of the file.

			println!("{:?}", lock);

			drop(lock);
		}


2. TryLockFile
		
		extern crate cluFlock;

		use cluFlock::Flock;
		use std::fs::File;
		use std::time::Duration;

		fn main() {
			let file = match File::create("/tmp/ulin.lock") {
				Ok(a) => a,
				Err(e) => panic!("Panic, err create file {:?}", e),
			};

			println!("Try_Exclusive_Lock, {:?}", file);
			let lock = match file.try_exclusive_lock() {
				//Success, we blocked the file.
				Ok(Some(lock)) => {
					println!("File {:?} successfully locked.", file);
					lock
				},

				//File already locked.
				Ok(None) => {
					println!("File {:?} already locked.", file);

					println!("!Exclusive_Lock, {:?}", file);
					//Lock the current thread to such an extent until your file is unlocked.
					file.exclusive_lock().unwrap()
				},

				Err(e) => panic!("Panic, err lock file {:?}", e)

			};

			println!("Sleep, 5s");
			::std::thread::sleep(Duration::from_secs(5));

			println!("Unlock, {:?}", file);
			drop(lock);
		}

3. FileFlock (BufLockFile + try_lock)

		extern crate cluFlock;

		use std::io::ErrorKind::AlreadyExists;
		use cluFlock::ExclusiveLock;
		use cluFlock::Flock;
		use std::path::Path;
		use std::fs;
		use std::io;
		use std::io::Error;
		use std::io::ErrorKind;
		use std::fs::OpenOptions;

		#[derive(Debug)]
		pub struct MyLockFile<'a>(ExclusiveLock, Option<&'a Path>);

		impl<'a> MyLockFile<'a> {
			pub fn new(p: &'a Path) -> Result<Self, io::Error> {
				let (lock, path) = match OpenOptions::new().write(true).create_new(true).open(p) {
					Ok(file) => (file.file_exclusive_lock()?, Some(p)),
					Err(ref e) if e.kind() == AlreadyExists => {
						let f = OpenOptions::new().read(true).open(p)?; 

						match f.try_file_exclusive_lock() {
							Ok(Some(lock)) => (lock, None),
							Ok(None) => return Err(Error::new(ErrorKind::Other, "the file is already locked")),
							Err(e) => return Err(e),
						}
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
