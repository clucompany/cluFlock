#[cfg(feature = "std")]
#[cfg(windows)]
mod windows {
	use cluFlock::ExclusiveFlock;
	use core::ops::Deref;
	use std::fs::File;
	use std::path::Path;

	struct AutoRemoveFile<'a>(&'a Path, File);

	impl<'a> AutoRemoveFile<'a> {
		pub fn file_create(path: &'a Path) -> Self {
			AutoRemoveFile(path, File::create(path).unwrap())
		}
	}

	impl<'a> Deref for AutoRemoveFile<'a> {
		type Target = File;

		#[inline(always)]
		fn deref(&self) -> &Self::Target {
			&self.1
		}
	}

	impl<'a> Drop for AutoRemoveFile<'a> {
		#[inline]
		fn drop(&mut self) {
			std::fs::remove_file(self.0).unwrap();
		}
	}

	#[test]
	fn windows_two_lock_behavior_onprocess() {
		let file =
			AutoRemoveFile::file_create(Path::new("./del_windows_two_lock_behavior_onprocess"));

		if let Err(e) = ExclusiveFlock::wait_lock(&*file) {
			panic!("Different platform behavior expected. {:?}", e);
		}

		let one_exclusive = ExclusiveFlock::wait_lock(&*file);
		let one_exclusive2 = ExclusiveFlock::try_lock(&*file);
		let one_exclusive3 = ExclusiveFlock::try_lock(&*file);

		if let Err(e) = one_exclusive {
			//one_exclusive: Ok
			panic!("Different platform behavior expected. {:?}", e);
		}
		if let Ok(e) = one_exclusive2 {
			//one_exclusive2: Err! the ok!
			panic!("Different platform behavior expected. {:?}", e);
		}
		if let Ok(e) = one_exclusive3 {
			//one_exclusive2: Err! the ok!
			panic!("Different platform behavior expected. {:?}", e);
		}

		// 1: Ok, 2: Err, 3: Err !

		drop(one_exclusive);
		drop(one_exclusive2);
		drop(one_exclusive3);
	}
}
