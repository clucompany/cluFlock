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
	File::create("./file")?.wait_exclusive_lock_fn(|mut file| {
		write!(file,  "Test.")
	})??;
	
	Ok( () )
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
		//lock...

		println!("{:?}", file_lock);
	}

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
	let file = File::create("./file")?;
	
	let shared = SharedFlock::wait_lock(&file);
	println!("#1shared {:?}", shared);
	
	let shared2 = SharedFlock::try_lock(&file)?;
	println!("#2shared {:?}", shared2);
	
	Ok( () )
}
```

# Support of platforms:
1. Unix, Linux: Full support: SharedFlock (Wait, Try), ExclusiveFlock (Wait, Try), Unlock (Wait, Try).
1. Windows: Full support: SharedFlock (Wait, Try), ExclusiveFlock (Wait, Try), Unlock (Wait, !Try). Unlock Try is not implemented and is considered additional unsafe functionality.

# Features of platforms:
1. Unix, Linux: The flock system call only works between processes, there are no locks inside the process.
2. Windows: System calls (LockFileEx UnlockFileEx) work between processes and within the current process. If you use Shared and Exclusive locks, you can lock yourself in the same process.

# Library flags:
1. nightly: Allows you to safely transform the lock into the original data, the night version of the compiler and the cluFullTransmute library are required.

# License

Copyright 2019 #UlinProject Denis Kotlyarov (Денис Котляров)
Licensed under the Apache License, Version 2.0


*/

#![cfg_attr(nightly, feature(nightly))]

use crate::data::err::FlockError;
use crate::data::unlock::WaitFlockUnlock;
use crate::element::FlockElement;
use crate::data::unlock::SafeUnlockFlock;

//os_release
mod os_release {
	#[cfg(unix)]
	pub mod unix;
	
	#[cfg(windows)]
	pub mod windows;
	
	pub mod r#dyn;
}
pub (crate) use os_release::r#dyn::*;

#[doc(hidden)]
pub (crate) mod sys {
	#[cfg(unix)]
	pub use crate::os_release::unix::*;
	
	#[cfg(windows)]
	pub use crate::os_release::windows::*;
}

mod data {
	pub mod err;
	pub mod unlock;
	
	mod lock;
	pub use self::lock::*;
	
	mod r#fn;
	pub (crate) use self::r#fn::*;
}
pub use self::data::*;

use crate::data::err::FlockFnError;


mod to;
pub use self::to::*;

pub mod element;


/// Set exclusive lock. Only one process can hold a data flow lock.
pub trait ExclusiveFlock where Self: FlockElement + WaitFlockUnlock + Sized {	
	fn try_lock(self) -> Result<FlockLock<Self>, FlockError<Self>>;
	fn wait_lock(self) -> Result<FlockLock<Self>, FlockError<Self>>;
	
	fn try_lock_fn<Fn: FnOnce(SafeUnlockFlock<Self>) -> Fr, Fr>(self, f: Fn) -> Result<Fr, FlockFnError<Self, Fn, Fr>>;
	fn wait_lock_fn<Fn: FnOnce(SafeUnlockFlock<Self>) -> Fr, Fr>(self, f: Fn) -> Result<Fr, FlockFnError<Self, Fn, Fr>>;
}


/// Set common lock, common locks can be many. An exclusive lock will wait for all shared locks to complete.
pub trait SharedFlock where Self: FlockElement + WaitFlockUnlock + Sized {	
	fn try_lock(self) -> Result<FlockLock<Self>, FlockError<Self>>;
	fn wait_lock(self) -> Result<FlockLock<Self>, FlockError<Self>>;
	
	fn try_lock_fn<Fn: FnOnce(SafeUnlockFlock<Self>) -> Fr, Fr>(self, f: Fn) -> Result<Fr, FlockFnError<Self, Fn, Fr>>;
	fn wait_lock_fn<Fn: FnOnce(SafeUnlockFlock<Self>) -> Fr, Fr>(self, f: Fn) -> Result<Fr, FlockFnError<Self, Fn, Fr>>;
}


