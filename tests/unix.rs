
use std::path::Path;
#[cfg(unix)]
use std::fs::File;
#[cfg(unix)]
use cluFlock::ExclusiveFlock;
#[cfg(unix)]
use std::process::Command;
#[cfg(unix)]
use std::time::Duration;

struct AutoRemoveFile<'a>(&'a Path);
impl<'a> Drop for AutoRemoveFile<'a> {
	#[inline]
	fn drop(&mut self) {
		std::fs::remove_file(self.0).unwrap();
	}
}


#[cfg(unix)]
#[test]
fn unix_two_lock_behavior_onprocess() {
	let path = AutoRemoveFile(Path::new("./del_unix_two_lock_behavior"));
	let file = File::create(path.0).unwrap();
	
	let one_exclusive = ExclusiveFlock::try_lock(&file);
	let one_exclusive2 = ExclusiveFlock::try_lock(&file);
	let one_exclusive3 = ExclusiveFlock::try_lock(&file);
	
	if let Err(e) = one_exclusive {
		panic!("Different platform behavior expected. {:?}", e);
	}
	if let Err(e) = one_exclusive2 {
		panic!("Different platform behavior expected. {:?}", e);
	}
	if let Err(e) = one_exclusive3 {
		panic!("Different platform behavior expected. {:?}", e);
	}
	
	// 1: Ok, 2: Ok, 3: Ok !
	
	// This behavior is found only in unix, 
	// be careful not to use double exclusive locks 
	// in the same process. It 'is the specifics of the platform.
	
	drop(one_exclusive);
	drop(one_exclusive2);
	drop(one_exclusive3);
}


#[cfg(unix)]
#[test]
fn unix_check() {
	let str_path = "./del_unix_check";
	let path = AutoRemoveFile(Path::new(str_path));
	let file = File::create(path.0).unwrap();
	
	match ExclusiveFlock::wait_lock(&file) {
		Ok(_a) => {},
		Err(e) => panic!("Strange behavior, failed to make a primary lock, {:?}", e),
	}
	if let Err(e) = ExclusiveFlock::try_lock(&file) {
		panic!("Strange behavior, failed to make a primary lock, {:?}", e);
	}
	
	
	let output = Command::new("flock").arg(str_path).arg("sleep").arg("4").spawn().expect("Failed to execute command");
	
	std::thread::sleep(Duration::from_secs(2));
	
	if let Ok(a) = ExclusiveFlock::try_lock(&file) {
		panic!("Strange behavior, we 've already made a lock in another process.., {:?}", a);
	}

	output.wait_with_output().expect("command wasn't running");
	//
	
	// The process holding the lock had
	// to die, we check the work.
	
	if let Err(e) = ExclusiveFlock::try_lock(&file) {
		panic!("Strange behavior, failed to make a primary lock, {:?}", e);
	}
	
	drop(file);
}