use iota::iota;

#[test]
fn sequential() {
	assert_eq!(iota!(), 0);
	assert_eq!(iota!(), 1);
	assert_eq!(iota!(), 2);
	assert_eq!(iota!(), 3);
	assert_eq!(iota!(), 4);
	assert_eq!(iota!(), 5);
	assert_eq!(iota!(), 6);
	assert_eq!(iota!(), 7);
	assert_eq!(iota!(), 8);
}
