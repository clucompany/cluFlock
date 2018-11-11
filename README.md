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

# License

Copyright 2018 #UlinProject Денис Котляров

Licensed under the Apache License, Version 2.0
