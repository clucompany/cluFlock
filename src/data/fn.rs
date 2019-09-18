
use crate::data::unlock::WaitFlockUnlock;
use crate::element::FlockElement;
use crate::BehOsRelease;
use crate::err::FlockFnError;
use crate::SafeUnlockFlock;
use core::marker::PhantomData;

pub (crate) struct FlockFnBuilder<D, F, Fr> where D: FlockElement + WaitFlockUnlock, F: FnOnce(SafeUnlockFlock<D>) -> Fr {
	pub (crate) data: D,
	pub (crate) function: F,
	
	_pp: PhantomData<Fr>,
}

impl<D, F, Fr> BehOsRelease for FlockFnBuilder<D, F, Fr> where D: FlockElement + WaitFlockUnlock, F: FnOnce(SafeUnlockFlock<D>) -> Fr {
	type Ok = Fr;
	type Err = FlockFnError<D, F, Fr>;
	type Data = Self;
	
	#[inline(always)]
	fn ok(args: Self::Data) -> Self::Ok {
		args.run()
	}
	
	#[inline(always)]
	fn err(t: Self::Data, err: std::io::Error) -> Self::Err {
		Self::Err::new(t.data, t.function, err)
	}
}

impl<D, F, Fr> FlockFnBuilder<D, F, Fr> where D: FlockElement + WaitFlockUnlock, F: FnOnce(SafeUnlockFlock<D>) -> Fr {
	#[inline]
	pub (crate) fn new(data: D, function: F) -> Self {
		Self {
			data: data,
			function: function,
			
			_pp: PhantomData,
		}
	}
	
	#[inline(always)]
	pub fn run(self) -> Fr {
		(self.function)( unsafe{ SafeUnlockFlock::new(self.data) } )
	}
}
