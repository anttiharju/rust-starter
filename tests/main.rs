use assert_cmd::cargo::cargo_bin_cmd;

fn run() -> (String, String) {
    let output = cargo_bin_cmd!("rust-starter").output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    (stdout, stderr)
}

#[test]
fn test_changes_output() {
    let (stdout, stderr) = run();

    for expected in ["Hello world!"] {
        assert!(stdout.contains(expected), "Expected output to contain '{}'", expected);
        assert!(!stderr.contains(expected), "Did not expect '{}' in stderr", expected);
    }
}
