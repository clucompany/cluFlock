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

1. ExclusiveLock - Only one process can retain exclusive lock of the file.
2. SharedLock - Can retain the general lock on the given file more than one process.

# Use

```
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
```

```
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
```

*/



#[cfg(unix)]
mod unix;
#[cfg(unix)]
use unix as sys;

use std::fmt::Debug;
use std::fs::File;
use std::io;

mod lock;
pub use self::lock::*;


pub trait Flock<'a>: Debug {
     type ExclusiveLock: FlockLock + 'a;
     type ExclusiveSliceLock: FlockLock + 'a;

     type SharedLock: FlockLock + 'a;
     type SharedSliceLock: FlockLock + 'a;

     fn try_exclusive_lock(&'a self) -> Result<Option<Self::ExclusiveSliceLock>, io::Error>;

     ///Set exclusive lock. Lock current thread in case of file lock. Only one process can retain exclusive lock of the file.
     fn exclusive_lock(&'a self) -> Result<Self::ExclusiveSliceLock, io::Error>;

     fn exclusive_lock_fn<F: FnMut(Self::ExclusiveSliceLock) -> A, A>(&'a self, f: F) -> Result<A, io::Error>;
     
     ///Set exclusive lock. Lock current thread in case of file lock. Only one process can retain exclusive lock of the file.
     fn file_exclusive_lock(self) -> Result<Self::ExclusiveLock, io::Error>;
     fn try_file_exclusive_lock(self) -> Result<Option<Self::ExclusiveLock>, io::Error>;
     

     fn try_shared_lock(&'a self) -> Result<Option<Self::SharedSliceLock>, io::Error>;

     ///Set shared lock. Lock current thread in case of file lock. Can retain the general lock on the given file more than one process.
     fn shared_lock(&'a self) -> Result<Self::SharedSliceLock, io::Error>;

     fn shared_lock_fn<F: FnMut(Self::SharedSliceLock) -> A, A>(&'a self, f: F) -> Result<A, io::Error>;

     ///Set shared lock. Lock current thread in case of file lock. Can retain the general lock on the given file more than one process.
     fn file_shared_lock(self) -> Result<Self::SharedLock, io::Error>;

     fn try_file_shared_lock(self) -> Result<Option<Self::SharedLock>, io::Error>;
}
/*
impl<'l, 'a, F: Flock<'a>> Flock<'a> for &'l F {
     type ExclusiveSliceLock = F::ExclusiveSliceLock;
     type SharedSliceLock = F::SharedSliceLock;

     #[inline(always)]
     fn try_exclusive_lock(&'a self) -> Result<Option<Self::ExclusiveSliceLock>, io::Error> {
          F::try_exclusive_lock(self)
     }

     #[inline(always)]
     fn exclusive_lock(&'a self) -> Result<Self::ExclusiveSliceLock, io::Error> {
          F::exclusive_lock(self)
     }

     #[inline(always)]
     fn exclusive_lock_fn<N: FnMut(Self::ExclusiveSliceLock) -> A, A>(&'a self, f: N) -> Result<A, io::Error> {
          F::exclusive_lock_fn(self, f)
     }

     #[inline(always)]
     fn try_shared_lock(&'a self) -> Result<Option<Self::SharedSliceLock>, io::Error> {
          F::try_shared_lock(self)
     }

     #[inline(always)]
     fn shared_lock(&'a self) -> Result<Self::SharedSliceLock, io::Error> {
          F::shared_lock(self)
     }    

     #[inline(always)]
     fn shared_lock_fn<N: FnMut(Self::SharedSliceLock) -> A, A>(&'a self, f: N) -> Result<A, io::Error> {
          F::shared_lock_fn(self, f)
     }
}

impl<'l, 'a, F: Flock<'a>> Flock<'a> for &'l mut F {
     type ExclusiveSliceLock = F::ExclusiveSliceLock;
     type SharedSliceLock = F::SharedSliceLock;

     #[inline(always)]
     fn try_exclusive_lock(&'a self) -> Result<Option<Self::ExclusiveSliceLock>, io::Error> {
          F::try_exclusive_lock(self)
     }

     #[inline(always)]
     fn exclusive_lock(&'a self) -> Result<Self::ExclusiveSliceLock, io::Error> {
          F::exclusive_lock(self)
     }

     #[inline(always)]
     fn exclusive_lock_fn<N: FnMut(Self::ExclusiveSliceLock) -> A, A>(&'a self, f: N) -> Result<A, io::Error> {
          F::exclusive_lock_fn(self, f)
     }

     #[inline(always)]
     fn try_shared_lock(&'a self) -> Result<Option<Self::SharedSliceLock>, io::Error> {
          F::try_shared_lock(self)
     }

     #[inline(always)]
     fn shared_lock(&'a self) -> Result<Self::SharedSliceLock, io::Error> {
          F::shared_lock(self)
     }    

     #[inline(always)]
     fn shared_lock_fn<N: FnMut(Self::SharedSliceLock) -> A, A>(&'a self, f: N) -> Result<A, io::Error> {
          F::shared_lock_fn(self, f)
     }
}*/

impl<'a> Flock<'a> for File {
     type ExclusiveLock = ExclusiveLock;
     type ExclusiveSliceLock = ExclusiveSliceLock<'a>;

     type SharedLock = SharedLock;
     type SharedSliceLock = SharedSliceLock<'a>;

     fn try_exclusive_lock(&'a self) -> Result<Option<Self::ExclusiveSliceLock>, io::Error> {
          Self::ExclusiveSliceLock::try_lock(self)
     }

     fn exclusive_lock(&'a self) -> Result<Self::ExclusiveSliceLock, io::Error> {
          Self::ExclusiveSliceLock::lock(self)
     }

     fn exclusive_lock_fn<N: FnMut(Self::ExclusiveSliceLock) -> A, A>(&'a self, mut f: N) -> Result<A, io::Error> {
          let lock = match self.exclusive_lock() {
               Ok(a) => a,
               Err(e) => return Err(e),
          };
          
          Ok(f(lock))
     }

     fn file_exclusive_lock(self) -> Result<Self::ExclusiveLock, io::Error> {
          Self::ExclusiveLock::lock(self)
     }
     fn try_file_exclusive_lock(self) -> Result<Option<Self::ExclusiveLock>, io::Error> {
          Self::ExclusiveLock::try_lock(self)
     }

     fn try_shared_lock(&'a self) -> Result<Option<Self::SharedSliceLock>, io::Error> {
          Self::SharedSliceLock::try_lock(self)
     }

     fn shared_lock(&'a self) -> Result<Self::SharedSliceLock, io::Error> {
          Self::SharedSliceLock::lock(self)
     }

     fn shared_lock_fn<N: FnMut(Self::SharedSliceLock) -> A, A>(&'a self, mut f: N) -> Result<A, io::Error> {
          let lock = match self.shared_lock() {
               Ok(a) => a,
               Err(e) => return Err(e),
          };
          
          Ok(f(lock))
     }

     fn file_shared_lock(self) -> Result<Self::SharedLock, io::Error> {
          Self::SharedLock::lock(self)
     }
     fn try_file_shared_lock(self) -> Result<Option<Self::SharedLock>, io::Error> {
          Self::SharedLock::try_lock(self)
     }
}


pub trait FlockLock: Drop + Debug {
     fn unlock(self) where Self: Sized {}
     fn box_unlock(self: Box<Self>) where Self: Sized {}
}


pub (crate) trait InitFlockLock {
     type Lock: FlockLock;
     type Arg;
     
     fn new(f: Self::Arg) -> Self::Lock;
}