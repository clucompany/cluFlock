#![allow(non_snake_case)]


//Copyright 2019 #UlinProject Денис Котляров

//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at

//	http://www.apache.org/licenses/LICENSE-2.0

//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
// limitations under the License.


//#Ulin Project 1819
//

/*!
Installation and subsequent safe removal of `flock` locks for data streams.

# Use
1. Exclusive LockFile

```
use cluFlock::ToFlock;
use std::fs::File;
use std::io;

fn main() -> Result<(), io::Error> {
	let file_lock = File::create("/tmp/1")?.wait_exclusive_lock()?;

	println!("{:?}", file_lock);
	drop(file_lock); //<-- unlock file

	Ok( () )
}
```

2. Exclusive LockFile (FnOnce)

```
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
```

3. Exclusive LockFile (&File)

```
extern crate cluFlock;

use cluFlock::ExclusiveFlock;
use std::fs::File;

fn main() -> Result<(), std::io::Error> {
	let file = File::create("/tmp/1").unwrap();

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

```
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


*/

#![cfg_attr(nightly, feature(nightly))]

//os_release
mod os_release {
	#[cfg(unix)]
	pub mod unix;
	
	pub mod r#dyn;
}
pub (crate) use os_release::r#dyn::*;

#[doc(hidden)]
pub (crate) mod sys {
	#[cfg(unix)]
	pub use crate::os_release::unix::*;
}

mod data {
	pub mod err;
	//pub use self::err::*;
	
	mod lock;
	pub use self::lock::*;
	
	mod unlock;
	pub use self::unlock::*;
}
pub use self::data::*;


use sys::FlockElement;
use crate::err::FlockError;
use crate::data::err::FlockFnError;


mod to;
pub use self::to::*;

mod r#fn;
pub (crate) use self::r#fn::*;


/// Initialize general lock. General blocking of a data stream may contain several processes.
pub trait SharedFlock where Self: FlockElement + FlockUnlock + Sized {	
	fn try_lock(self) -> Result<FlockLock<Self>, FlockError<Self>>;
	fn wait_lock(self) -> Result<FlockLock<Self>, FlockError<Self>>;
	
	fn try_lock_fn<Fn: FnOnce(SafeUnlockFlock<Self>) -> Fr, Fr>(self, f: Fn) -> Result<Fr, FlockFnError<Self, Fn, Fr>>;
	fn wait_lock_fn<Fn: FnOnce(SafeUnlockFlock<Self>) -> Fr, Fr>(self, f: Fn) -> Result<Fr, FlockFnError<Self, Fn, Fr>>;
}


/// Set exclusive lock. Only one process can hold a data flow lock.
pub trait ExclusiveFlock where Self: FlockElement + FlockUnlock + Sized {	
	fn try_lock(self) -> Result<FlockLock<Self>, FlockError<Self>>;
	fn wait_lock(self) -> Result<FlockLock<Self>, FlockError<Self>>;
	
	fn try_lock_fn<Fn: FnOnce(SafeUnlockFlock<Self>) -> Fr, Fr>(self, f: Fn) -> Result<Fr, FlockFnError<Self, Fn, Fr>>;
	fn wait_lock_fn<Fn: FnOnce(SafeUnlockFlock<Self>) -> Fr, Fr>(self, f: Fn) -> Result<Fr, FlockFnError<Self, Fn, Fr>>;
}




