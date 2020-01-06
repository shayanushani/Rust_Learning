// Test defines test_gcd as a test function
// will be skipped in normal compilations 
// included and called if cargo test is run 
// test function = attribute 
#[test]
fn test_gcd() {
    assert_eq!(gcd(2 * 5 * 11 * 17,
                   3 * 7 * 13 * 19),
1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17,
                   3 * 7 * 11 * 13 * 19),
3 * 11);
}