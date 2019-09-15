
use std::path::Path;
#[cfg(windows)]
use std::fs::File;
#[cfg(windows)]
use cluFlock::ExclusiveFlock;

struct AutoRemoveFile<'a>(&'a Path);
impl<'a> Drop for AutoRemoveFile<'a> {
	#[inline]
	fn drop(&mut self) {
		std::fs::remove_file(self.0).unwrap();
	}
}


#[cfg(windows)]
#[test]
fn windows_two_lock_behavior_onprocess() {
	let path = AutoRemoveFile(Path::new("./del_windows_two_lock_behavior_onprocess"));
	let file = File::create(path.0).unwrap();
	
	if let Err(e) = ExclusiveFlock::wait_lock(&file) {
		panic!("Different platform behavior expected. {:?}", e);
	}
	
	let one_exclusive = ExclusiveFlock::wait_lock(&file);
	let one_exclusive2 = ExclusiveFlock::try_lock(&file);
	let one_exclusive3 = ExclusiveFlock::try_lock(&file);
	
	if let Err(e) = one_exclusive { //one_exclusive: Ok
		panic!("Different platform behavior expected. {:?}", e);
	}
	if let Ok(e) = one_exclusive2 { //one_exclusive2: Err! the ok!
		panic!("Different platform behavior expected. {:?}", e);
	}
	if let Ok(e) = one_exclusive3 { //one_exclusive2: Err! the ok!
		panic!("Different platform behavior expected. {:?}", e);
	}
	
	// 1: Ok, 2: Err, 3: Err !
	
	drop(one_exclusive);
	drop(one_exclusive2);
	drop(one_exclusive3);
}