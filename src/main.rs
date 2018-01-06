/// spoj-runner - automate testing your programs for SPOJ
///
/// Usage:
///   Run this application in root directory of your project.
///   It have to contain sub-directory 'tests' where each 
///   subdirectory is a black-box test of your application.
///   These directories has to contains two files: in.txt 
///   and out.txt. in.txt contains input to your application.
///   out.txt contains expected result. spoj-runner automates 
///   boring manual testing by running given tests and shows 
///   tests status in nicely formatted way.

use std::fs;
use std::env;

fn main() {
    let mut tests_dir = env::current_dir().unwrap();
    tests_dir.push("tests");
    let tests = fs::read_dir(tests_dir).unwrap();
    for test in tests {
        let test = test.unwrap();
        if !test.file_type().unwrap().is_dir() {
            println!("{:?} is not a directory - skipping", test.path());
            continue;
        }

        for test_file in fs::read_dir(test.path()).unwrap() {
            println!("{}", test_file.unwrap().path().to_str().unwrap());
        }
    }
}
