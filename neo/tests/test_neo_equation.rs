
// --------------------- Binary integration tests ---------------------
// Testing framework: Rust built-in test harness (std::test) with #[test]

#[test]
fn run_neo_equation_binary_prints_expected_output() {
    let exe = std::env::var("CARGO_BIN_EXE_neo_equation")
        .expect("CARGO_BIN_EXE_neo_equation not set; ensure [[bin]] neo_equation exists in Cargo.toml");

    let output = std::process::Command::new(exe)
        .output()
        .expect("failed to run neo_equation binary");

    assert\!(
        output.status.success(),
        "binary exited with non-zero status: {:?}",
        output.status
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert\!(
        stdout.contains("The conceptual Neo value is: 583"),
        "stdout missing expected value; got:\n{}",
        stdout
    );
    assert\!(
        stdout.contains("Note: M_p values are placeholders"),
        "stdout missing explanatory note; got:\n{}",
        stdout
    );
}

#[test]
fn run_neo_equation_binary_emits_no_stderr() {
    let exe = std::env::var("CARGO_BIN_EXE_neo_equation")
        .expect("CARGO_BIN_EXE_neo_equation not set; ensure [[bin]] neo_equation exists in Cargo.toml");

    let output = std::process::Command::new(exe)
        .output()
        .expect("failed to run neo_equation binary");

    assert\!(
        output.stderr.is_empty(),
        "stderr should be empty; got:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}