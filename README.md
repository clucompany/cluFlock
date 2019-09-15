# cluFlock

[![Build Status](https://travis-ci.org/clucompany/cluFlock.svg?branch=master)](https://travis-ci.org/clucompany/cluFlock)
[![Apache licensed](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](./LICENSE)
[![crates.io](http://meritbadge.herokuapp.com/cluFlock)](https://crates.io/crates/cluFlock)
[![Documentation](https://docs.rs/cluFlock/badge.svg)](https://docs.rs/cluFlock)

Installation and subsequent safe removal of `flock` locks for data streams.


# Use
1. Exclusive LockFile

```rust
use cluFlock::ToFlock;
use std::fs::File;
use std::io;

fn main() -> Result<(), io::Error> {
	let file_lock = File::create("./1")?.wait_exclusive_lock()?;

	println!("{:?}", file_lock);
	drop(file_lock); //<-- unlock file

	Ok( () )
}
```

2. Exclusive LockFile (FnOnce)

```rust
use std::io::Write;
use cluFlock::ToFlock;
use std::fs::File;
use std::io;

fn main() -> Result<(), io::Error> {
	File::create("./1")?.wait_exclusive_lock_fn(|mut file| {
		write!(file,  "Test.")
	})??;

	Ok( () )
}
```

3. Exclusive LockFile (&File)

```rust
extern crate cluFlock;

use cluFlock::ExclusiveFlock;
use std::fs::File;

fn main() -> Result<(), std::io::Error> {
	let file = File::create("./1").unwrap();

	let file_lock = ExclusiveFlock::wait_lock(&file)?;
	//lock...

	println!("{:?}", file_lock);
	
	// let file move! 
	drop(file_lock);

	file.sync_all()?;

	Ok( () )
}
```

4. LockFile (use try_exclusive_lock)

```rust
use cluFlock::ExclusiveFlock;
use std::fs::File;
use std::time::Duration;
use std::io::ErrorKind;

fn main() {
	let file: File = match File::create("./ulin.lock") {
		Ok(a) => a,
		Err(e) => panic!("Panic, err create file {:?}", e),
	};

	println!("Try_Exclusive_Lock, {:?}", file);

	let lock = match ExclusiveFlock::try_lock(&file) {
		//Success, we blocked the file.
		Ok(lock) => {
			println!("OK, File {:?} successfully locked.", file);

			
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
```

# Library flags:
1. nightly: Allows you to safely transform the lock into the original data, the night version of the compiler and the cluFullTransmute library are required.


# License

Copyright 2019 #UlinProject Denis Kotlyarov (Денис Котляров)

Licensed under the Apache License, Version 2.0
