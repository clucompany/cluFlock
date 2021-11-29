# cluFlock

[![Build Status](https://travis-ci.org/clucompany/cluFlock.svg?branch=master)](https://travis-ci.org/clucompany/cluFlock)
[![Platform](https://img.shields.io/badge/platform-unix%20|%20linux%20|%20windows-blue)](https://github.com/clucompany/cluFlock/tree/master/tests)
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
	let file_lock = File::create("./file")?.wait_exclusive_lock()?;
	println!("{:?}", file_lock);
	
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
	File::create("./file")?.wait_exclusive_lock_fn(
		// valid exclusive lock
		|mut file| write!(file, "Test."), // result: Ok(usize)/Err(std::io::Error)
		
		// invalid lock
		|err| Err(err.into_err()) // into_err: FlockErr -> std::io::Error
	)?;
	
	Ok(())
}
```

3. Exclusive LockFile (&File)

```rust
use cluFlock::ExclusiveFlock;
use std::fs::File;

fn main() -> Result<(), std::io::Error> {
	let file = File::create("./file")?;
	
	{
		let file_lock = ExclusiveFlock::wait_lock(&file)?;
		// file_lock, type: FlockLock<&File>

		println!("{:?}", file_lock);
	} // auto unlock ExclusiveFlock

	file.sync_all()?;

	Ok( () )
}
```

4. Shared LockFile (&File)

```rust
use std::fs::File;
use cluFlock::SharedFlock;
use std::io;

fn main() -> Result<(), io::Error> {
	let file = File::create("./test_file")?;
	
	let shared = SharedFlock::wait_lock(&file);
	println!("#1shared {:?}", shared);
	let shared2 = SharedFlock::try_lock(&file);
	println!("#2shared {:?}", shared2);
	
	assert_eq!(shared.is_ok(), true);
	assert_eq!(shared2.is_ok(), true);
	
	// manual or automatic unlock SharedFlock_x2
	// drop(shared);
	// drop(shared2);
	
	Ok( () )
}
```

# Support of platforms:
1. Unix, Linux: Full support: SharedFlock (Wait, Try), ExclusiveFlock (Wait, Try), Unlock (Wait, Try).
1. Windows: Full support: SharedFlock (Wait, Try), ExclusiveFlock (Wait, Try), Unlock (Wait, !Try). Unlock Try is not implemented and is considered additional unsafe functionality.

# Features of platforms:
1. Unix, Linux: The flock system call only works between processes, there are no locks inside the process.
2. Windows: System calls (LockFileEx UnlockFileEx) work between processes and within the current process. If you use Shared and Exclusive locks, you can lock yourself in the same process.

# License

Copyright 2021 #UlinProject Denis Kotlyarov (Денис Котляров)

Licensed under the Apache License, Version 2.0