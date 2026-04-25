use nadesiko4::bytecode::{ByteCode, ByteCodeKind, NakoSystem};
/// VM integration tests
use nadesiko4::run_test;
use nadesiko4::value::Value;
use nadesiko4::vm;

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
    assert!(
        output.contains("Division by zero"),
        "Error message should mention division by zero"
    );
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
    assert_eq!(
        output.trim(),
        "14",
        "2+3*4を表示 should equal 14 (multiplication first)"
    );
}

#[test]
fn test_let() {
    let output = run_test("A=30; Aを表示");
    assert_eq!(output.trim(), "30", "A=30; Aを表示。");
}

fn run_vm_binary_op(left: f64, right: f64, op: ByteCodeKind) -> String {
    let mut sys = NakoSystem::new();
    sys.const_list.push(Value::from_number(left));
    sys.const_list.push(Value::from_number(right));
    sys.codes
        .push(ByteCode::new(ByteCodeKind::PushConst, 0, 0, 0));
    sys.codes
        .push(ByteCode::new(ByteCodeKind::PushConst, 1, 0, 0));
    sys.codes.push(ByteCode::new(op, 0, 0, 0));
    sys.codes.push(ByteCode::new(ByteCodeKind::Print, 0, 0, 0));
    let ok = vm::run(&mut sys);
    assert!(ok, "VM run should succeed: {}", sys.error_msg);
    assert!(sys.error_msg.is_empty(), "VM should not emit error");
    sys.output.trim().to_string()
}

#[test]
fn test_vm_gt() {
    assert_eq!(run_vm_binary_op(10.0, 3.0, ByteCodeKind::Gt), "1");
    assert_eq!(run_vm_binary_op(2.0, 8.0, ByteCodeKind::Gt), "0");
}

#[test]
fn test_vm_gteq() {
    assert_eq!(run_vm_binary_op(10.0, 10.0, ByteCodeKind::GtEq), "1");
    assert_eq!(run_vm_binary_op(2.0, 8.0, ByteCodeKind::GtEq), "0");
}

#[test]
fn test_vm_lt() {
    assert_eq!(run_vm_binary_op(1.0, 3.0, ByteCodeKind::Lt), "1");
    assert_eq!(run_vm_binary_op(9.0, 2.0, ByteCodeKind::Lt), "0");
}

#[test]
fn test_vm_lteq() {
    assert_eq!(run_vm_binary_op(5.0, 5.0, ByteCodeKind::LtEq), "1");
    assert_eq!(run_vm_binary_op(9.0, 2.0, ByteCodeKind::LtEq), "0");
}

#[test]
fn test_vm_equal() {
    assert_eq!(run_vm_binary_op(7.0, 7.0, ByteCodeKind::Equal), "1");
    assert_eq!(run_vm_binary_op(7.0, 8.0, ByteCodeKind::Equal), "0");
}

#[test]
fn test_vm_not_eq() {
    assert_eq!(run_vm_binary_op(7.0, 8.0, ByteCodeKind::NotEq), "1");
    assert_eq!(run_vm_binary_op(7.0, 7.0, ByteCodeKind::NotEq), "0");
}
