use assert_cmd::Command;
use predicates::prelude::*;
use std::error::Error;
use std::fs;

// Borrowed from commandline rust
type TestResult = Result<(), Box<dyn Error>>;

struct TestDay {
    day: String,
    args: Vec<String>,
    half: String
}

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
// Day 5
#[test]
fn test5() -> TestResult {
    test_match_number("5", "")
}
#[test]
fn test5b() -> TestResult {
    test_match_number("5", "b")
}
// Day 6
#[test]
fn test6() -> TestResult {
    test_match_number_arg("6", "", "80")
}
#[test]
fn test6b() -> TestResult {
    test_match_number_arg("6", "b", "256")
}
// Day 7
#[test]
fn test7() -> TestResult {
    test_match_number("7", "")
}
#[test]
fn test7b() -> TestResult {
    test_match_number("7", "b")
}
// Day 8
#[test]
fn test8() -> TestResult {
    test_match_number("8", "")
}
#[test]
fn test8b() -> TestResult {
    test_match_number("8", "b")
}
// Day 9
#[test]
fn test9() -> TestResult {
    test_match_number("9", "")
}
#[test]
fn test9b() -> TestResult {
    test_match_number("9", "b")
}
// Day 10
#[test]
fn test10() -> TestResult {
    test_match_number("10", "")
}
#[test]
fn test10b() -> TestResult {
    test_match_number("10", "b")
}
// Day 11
#[test]
fn test11() -> TestResult {
    test_match_number("11", "")
}
#[test]
fn test11b() -> TestResult {
    test_match_number("11", "b")
}
// Day 21
#[test]
fn test21() -> TestResult {
    TestDay::day("21")
        .half("")
        .arg("4")
        .arg("8")
        .assert()
}
#[test]
fn test21b() -> TestResult {
    TestDay::day("21")
        .half("b")
        .arg("4")
        .arg("8")
        .assert()
}

impl TestDay {
    pub fn day(day: &str) -> TestDay {
        TestDay {
            day: day.to_string(),
            args: Vec::new(),
            half: String::from(""),
        }
    }

    // add argument
    pub fn arg<'a>(&'a mut self, arg: &str) -> &'a mut TestDay {
        self.args.push(arg.to_string());
        self
    }

    // add test filename (optional)
    pub fn test_file<'a>(&'a mut self) -> &'a mut TestDay {
        self.args.push(format!("tests/inputs/test{}.dat", self.day));
        self
    }

    // add a,b half of day designator (optional)
    pub fn half<'a>(&'a mut self, half: &str) -> &'a mut TestDay {
        self.half.push_str(half);
        self
    }

    // Do the test run
    pub fn assert<'a>(&'a mut self) -> TestResult {
        let expected = fs::read_to_string(format!("tests/outputs/advent{}{}.out", self.day, self.half))?;

        Command::cargo_bin(format!("advent{}{}", self.day, self.half))?
            .args(&self.args)
            .assert()
            .stdout(predicate::str::contains(expected.trim_end()));

        Ok(())
    }
}

// Generic find number in output test
fn test_match_number(day: &str, half: &str) -> TestResult {
    let expected = fs::read_to_string(format!("tests/outputs/advent{}{}.out", day, half))?;
    
    Command::cargo_bin(format!("advent{}{}", day, half))?
        .arg(format!("tests/inputs/test{}.dat", day))
        .assert()
        .stdout(predicate::str::contains(expected.trim_end()));

    Ok(())
}

// Find number in output test with additional argument
fn test_match_number_arg(day: &str, half: &str, arg: &str) -> TestResult {
    // a test
    let expected = fs::read_to_string(format!("tests/outputs/advent{}{}.out", day, half))?;
    
    Command::cargo_bin(format!("advent{}", day))?
        .arg(format!("tests/inputs/test{}.dat", day))
        .arg(arg)
        .assert()
        .stdout(predicate::str::contains(expected.trim_end()));

    Ok(())
}