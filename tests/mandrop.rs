use SafeManuallyDrop::ManuallyDrop;

#[test]
fn test_man_drop() {
	//
	// During tests, `ManuallyDrop` must always be verifiable.
	//
	assert!(ManuallyDrop::is_safe_mode());
}
