

///Generic is required to describe the internal behavior of a library under a specific operating system.
pub trait BehOsRelease {
	type Ok;
	
	
	type Err;
	/// Default:	`FlockError<Self::Arg>`
	/// FnOnce:		`FlockFnBuilder<Self, Fn, Fr>` //Fn: FnOnce(...) -> Fr
	/// 
	
	type Data;
	/// Maybe: ()
	/// 
	
	fn ok(t: Self::Data) -> Self::Ok;
	fn err(t: Self::Data, err: std::io::Error) -> Self::Err;
}

