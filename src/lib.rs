//Copyright 2018 #UlinProject Денис Котляров

//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at

//       http://www.apache.org/licenses/LICENSE-2.0

//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
// limitations under the License.


//#Ulin Project 1718
//

/*!
Control of lock of the file using the 'flock' functions.


# Capabilities

1. Convenient and transparent trait of a call of locks.
2. Automatic unlocking of lock.

# Locks

1. ExclusiveFlock - To establish exclusive blocking. Only one process can hold exclusive blocking of the file..
2. SharedFlock - Set a shared lock. A shared lock on a given file can hold more than one process.

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


2. Exclusive LockClosure

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

3. The temporary file for interprogram synchronization

```
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
```

# License

Copyright 2018 #UlinProject Денис Котляров
Licensed under the Apache License, Version 2.0

*/



#[cfg(unix)]
mod raw;
#[cfg(unix)]
use std::ops::Deref;
use std::fs::File;

#[cfg(unix)]
pub (crate) use self::raw::unix as sys;


use std::fmt::Debug;
use std::io;

mod lock;
pub use self::lock::*;


///Constructor, generalized for 'Flock'
pub trait ToFlock: Debug {

     //exclusive

     #[inline(always)]
     fn wait_exclusive_lock(self) -> Result<Self::ExclusiveLock, io::Error> where Self: ExclusiveFlock + Sized {
          ExclusiveFlock::wait_lock(self)
     }

     
     #[inline(always)]
     fn wait_exclusive_lock_fn<A: FnMut(Self::ExclusiveLockFn) -> R, R>(self, f: A) -> Result<R, io::Error> where Self: ExclusiveFlockFn + Sized {
          ExclusiveFlockFn::wait_lock_fn(self, f)
     }
     

     #[inline(always)]
     fn try_exclusive_lock(self) -> Result<Self::ExclusiveLock, io::Error> where Self: ExclusiveFlock + Sized {
          ExclusiveFlock::try_lock(self)
     }

     
     #[inline(always)]
     fn try_exclusive_lock_fn<A: FnMut(Self::ExclusiveLockFn) -> R, R>(self, f: A) -> Result<R, io::Error> where Self: ExclusiveFlockFn + Sized {
          ExclusiveFlockFn::try_lock_fn(self, f)
     }
     

     //shared

     #[inline(always)]
     fn wait_shared_lock(self) -> Result<Self::SharedLock, io::Error> where Self: SharedFlock + Sized {
          SharedFlock::wait_lock(self)
     }
     
     #[inline(always)]
     fn wait_shared_lock_fn<A: FnMut(Self::SharedLockFn) -> R, R>(self, f: A) -> Result<R, io::Error> where Self: SharedFlockFn + Sized {
          SharedFlockFn::wait_lock_fn(self, f)
     }

     #[inline(always)]
     fn try_shared_lock(self) -> Result<Self::SharedLock, io::Error> where Self: SharedFlock + Sized {
          SharedFlock::try_lock(self)
     }

     #[inline(always)]
     fn try_shared_lock_fn<A: FnMut(Self::SharedLockFn) -> R, R>(self, f: A) -> Result<R, io::Error> where Self: SharedFlockFn + Sized {
          SharedFlockFn::try_lock_fn(self, f)
     }
}

impl<'a, F: ToFlock> ToFlock for &'a F {}
impl<'a, F: ToFlock> ToFlock for &'a mut F {}


///To establish exclusive blocking. Only one process can hold exclusive blocking of the file.
pub trait ExclusiveFlock: Debug {
     type ExclusiveLock: FlockLock;

     fn try_lock(self) -> Result<Self::ExclusiveLock, io::Error>;
     
     fn wait_lock(self) -> Result<Self::ExclusiveLock, io::Error>;
}

pub trait ExclusiveFlockFn: Debug {
     type ExclusiveLockFn: FlockLock;

     fn try_lock_fn<A: FnMut(Self::ExclusiveLockFn) -> R, R>(self, f: A) -> Result<R, io::Error>;
     fn wait_lock_fn<A: FnMut(Self::ExclusiveLockFn) -> R, R>(self, f: A) -> Result<R, io::Error>;
}


///Set a shared lock. A shared lock on a given file can hold more than one process.
pub trait SharedFlock: Debug {
     type SharedLock: FlockLock;

     fn try_lock(self) -> Result<Self::SharedLock, io::Error>;
     fn wait_lock(self) -> Result<Self::SharedLock, io::Error>;     
}

pub trait SharedFlockFn: Debug {
     type SharedLockFn: FlockLock;

     fn try_lock_fn<A: FnMut(Self::SharedLockFn) -> R, R>(self, f: A) -> Result<R, io::Error>;
     fn wait_lock_fn<A: FnMut(Self::SharedLockFn) -> R, R>(self, f: A) -> Result<R, io::Error>;
}



///The trait describing the working `flock` blocking
pub trait FlockLock: Debug + AsRef<File> + Deref<Target = File> {}


///Trait of 'FlockLock' with a possibility of removal of blocking.
pub trait FlockUnlock: FlockLock {
     type ResultUnlock;
     
     fn unlock(self) -> Self::ResultUnlock;
}

