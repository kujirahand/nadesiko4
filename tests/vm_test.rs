/// VM integration tests
use nadesiko4::run_test;

#[test]
fn test_addition() {
    let output = run_test("3 + 5を表示");
    assert_eq!(output.trim(), "8", "3 + 5 should equal 8");
}

#[test]
fn test_subtraction() {
    let output = run_test("10 - 3を表示");
    assert_eq!(output, "7", "10 - 3 should equal 7");
}

#[test]
fn test_multiplication() {
    let output = run_test("6 * 7を表示");
    assert_eq!(output, "42", "6 * 7 should equal 42");
}

#[test]
fn test_division() {
    let output = run_test("20 / 4を表示");
    assert_eq!(output, "5", "20 / 4 should equal 5");
}

#[test]
fn test_division_by_zero() {
    let output = run_test("10 / 0を表示");
    assert!(output.contains("Division by zero"), "Error message should mention division by zero");
}

#[test]
fn test_complex_expression() {
    let output = run_test("2 + 3を表示");
    assert_eq!(output, "5", "2 + 3 should equal 5");
}

#[test]
fn test_string_display() {
    let output = run_test("\"こんにちは\"を表示");
    assert_eq!(output, "こんにちは", "String should be displayed correctly");
    let output = run_test("「こんにちは」を表示");
    assert_eq!(output, "こんにちは", "String should be displayed correctly");
}
#[test]
fn test_expression_with_josi() {
    let output = run_test("3+5を表示");
    assert_eq!(output.trim(), "8", "3+5を表示 should equal 8");
}

#[test]
fn test_complex_expression_with_josi() {
    let output = run_test("2+3*4を表示");
    assert_eq!(output.trim(), "14", "2+3*4を表示 should equal 14 (multiplication first)");
}

#[test]
fn test_let() {
    let output = run_test("A=30; Aを表示");
    assert_eq!(output.trim(), "30", "A=30; Aを表示。");
}
