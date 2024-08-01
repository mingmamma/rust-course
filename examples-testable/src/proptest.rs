use proptest::prelude::*;

proptest! {
    #[test]
    fn test_add(a in 0..=3, b in 0..=3) {
        prop_assert!(a + b <= 5)
    }
}