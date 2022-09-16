use assert_cmd::Command;
use predicates::prelude::*;
use std::error::Error;
use std::fs;

// Borrowed from commandline rust
type TestResult = Result<(), Box<dyn Error>>;

// Day 1
#[test]
fn test1b() -> TestResult {
    test_match_number("1", "b")
}
// Day 2
#[test]
fn test2a() -> TestResult {
    test_match_number("2", "a")
}
#[test]
fn test2b() -> TestResult {
    test_match_number("2", "b")
}
// Day 3
#[test]
fn test3a() -> TestResult {
    test_match_number("3", "a")
}
#[test]
fn test3b() -> TestResult {
    test_match_number("3", "b")
}
// Day 4
#[test]
fn test4a() -> TestResult {
    test_match_number("4", "a")
}
#[test]
fn test4b() -> TestResult {
    test_match_number("4", "b")
}


// Generic find number in output test
fn test_match_number(day: &str, half: &str) -> TestResult {
    // a test
    let expected = fs::read_to_string(format!("tests/outputs/advent{}{}.out", day, half))?;
    
    Command::cargo_bin(format!("advent{}{}", day, half))?
        .arg(format!("tests/inputs/test{}.dat", day))
        .assert()
        .stdout(predicate::str::contains(expected.trim_end()));

    Ok(())
}