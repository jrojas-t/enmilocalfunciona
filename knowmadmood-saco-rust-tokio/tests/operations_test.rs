use lib_saco_utils;

#[test]
fn test_addition() {
    // using common code.
    let resp = lib_saco_utils::addition(10, 5);
    assert_eq!(resp, 15);
}

#[test]
fn test_subtraction() {
    // using common code.
    let resp = lib_saco_utils::subtraction(10, 5);
    assert_eq!(resp, 5);
}

#[test]
fn test_multiplication() {
    // using common code.
    let resp = lib_saco_utils::multiplication(10, 5);
    assert_eq!(resp, 50);
}

#[test]
fn test_division() {
    // using common code.
    let resp = lib_saco_utils::division(10, 5);
    assert_eq!(resp, 2);
}
