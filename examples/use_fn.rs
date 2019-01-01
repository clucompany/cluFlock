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