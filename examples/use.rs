
extern crate cluFlock;

use cluFlock::ToFlock;
use std::fs::File;
use std::io;

fn main() -> Result<(), io::Error> {
     let file_lock = File::create("/tmp/1")?.wait_exclusive_lock()?;

     println!("{:?}", file_lock);
     
     drop(file_lock);

     Ok( () )
}