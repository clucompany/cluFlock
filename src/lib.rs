//#![feature(const_fn)]
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


//#Ulin Project 1718
//

/*!
Establishes and safely deletes advisory blocking on the open file.

# Use
1. Exclusive LockFile

```
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
```

2. Exclusive LockFile (FnOnce)

```
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

*/

mod os_release;
mod sys {
	pub use crate::os_release::*;
}

mod to;
pub use crate::sys::FlockElement;
pub use self::to::*;

mod unlock;
pub use self::unlock::*;

mod error;
pub use self::error::*;

mod lock;
pub use self::lock::*;

///Set a shared lock. A shared lock on a given file can hold more than one process.
pub trait SharedFlock: FlockElement + FlockUnlock + Sized {	
	fn try_lock(self) -> Result<FlockLock<Self>, FlockError<Self>>;
	fn wait_lock(self) -> Result<FlockLock<Self>, FlockError<Self>>;
}

pub trait SharedFlockFn: FlockElement + FlockUnlock + Sized  {
	fn try_lock_fn<A: FnOnce(UnlockFlock<Self>) -> R, R>(self, f: A) -> Result<R, FlockError<(Self, A)>>;
	fn wait_lock_fn<A: FnOnce(UnlockFlock<Self>) -> R, R>(self, f: A) -> Result<R, FlockError<(Self, A)>>;
}


///To establish exclusive blocking. Only one process can hold exclusive blocking of the file.
pub trait ExclusiveFlock: FlockElement + FlockUnlock + Sized {	
	fn try_lock(self) -> Result<FlockLock<Self>, FlockError<Self>>;
	fn wait_lock(self) -> Result<FlockLock<Self>, FlockError<Self>>;
}


pub trait ExclusiveFlockFn: FlockElement + FlockUnlock + Sized  {
	fn try_lock_fn<A: FnOnce(UnlockFlock<Self>) -> R, R>(self, f: A) -> Result<R, FlockError<(Self, A)>>;
	fn wait_lock_fn<A: FnOnce(UnlockFlock<Self>) -> R, R>(self, f: A) -> Result<R, FlockError<(Self, A)>>;
}


pub (crate) trait RawConstructorElement {
	type ConstResult;
	type Arg: FlockElement;
	
	fn raw_constructor(t: Self::Arg) -> Self::ConstResult;
}
