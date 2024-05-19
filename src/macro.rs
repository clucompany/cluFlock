#[cfg(feature = "std")]
macro_rules! cfg_std {
	[ if #std { $($all_tt:tt)* } $(else {$($eall_tt:tt)*} )? ] => {
		$($all_tt)*
	};
}

#[cfg(not(feature = "std"))]
macro_rules! cfg_std {
	[ if #std { $($all_tt:tt)* } $(else {$($eall_tt:tt)*} )? ] => {
		$( $($eall_tt)* )?
	};
}

#[allow(unused_imports)]
pub(crate) use cfg_std;
