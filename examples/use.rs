

extern crate cluFlock;

use cluFlock::Flock;
use std::fs::File;
use std::io;

fn main() -> Result<(), io::Error> {
     let file = File::create("/tmp/1")?;

     let file_lock = file.wait_exclusive_lock()?;
     //lock...

     println!("{:?}", file_lock);
     
     // let file move! 
     drop(file_lock);

     Ok( () )
}