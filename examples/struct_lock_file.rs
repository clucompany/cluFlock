
extern crate cluFlock;

use cluFlock::ExclusiveLock;
use cluFlock::Flock;
use std::path::Path;
use std::fs::File;
use std::io;

#[derive(Debug)]
pub struct MyLockFile(ExclusiveLock);

impl MyLockFile {
     pub fn new<P: AsRef<Path>>(p: P) -> Result<Self, io::Error> {
          let f = File::create(p)?; 
          let ex_lock = f.file_exclusive_lock()?;

          Ok( MyLockFile(ex_lock) )
     }
}


pub fn main() {
     
}
