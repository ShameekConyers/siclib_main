use std::process::Command;

fn main() {
    // build run tests on C++ code
    let dir = env!("CARGO_MANIFEST_DIR").to_owned() + "/scripts/test.sh";
    eprintln!("{}", dir);
    let status = Command::new(dir).status().expect("expected success");

    eprintln!("{}", status);
    assert!(status.success());
}
