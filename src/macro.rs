
#[cfg(feature = "std")]
#[doc(hidden)]
#[macro_export]
macro_rules! cfg_std {
	[ if #std { $($all_tt:tt)* } $(else {$($eall_tt:tt)*} )? ] => {
		$($all_tt)*
	};
}

#[cfg(not(feature = "std"))]
#[doc(hidden)]
#[macro_export]
macro_rules! cfg_std {
	[ if #std { $($all_tt:tt)* } $(else {$($eall_tt:tt)*} )? ] => {
		$( $($eall_tt)* )?
	};
}

