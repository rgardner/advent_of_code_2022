use std::{env, process::Command};

enum BuildMode {
    Debug,
    Release,
}

fn run_build(mode: BuildMode) {
    let mut cmd = Command::new("cargo");
    cmd.args(["build", "--workspace"]);
    if matches!(mode, BuildMode::Release) {
        cmd.arg("--release");
    }
    let status = cmd.status().unwrap();
    if !status.success() {
        panic!("cargo build failed");
    }
}

fn run_test(mode: BuildMode) {
    let mut cmd = Command::new("cargo");
    cmd.args(["test", "--workspace"]);
    if matches!(mode, BuildMode::Release) {
        cmd.arg("--release");
    }
    let status = cmd.status().unwrap();
    if !status.success() {
        panic!("cargo test failed");
    }
}

enum FmtMode {
    Check,
    Update,
}

fn run_fmt(mode: FmtMode) {
    let mut cmd = Command::new("cargo");
    cmd.args(["fmt", "--all"]);
    if matches!(mode, FmtMode::Check) {
        cmd.arg("--check");
    }
    let status = cmd.status().unwrap();
    if !status.success() {
        panic!("cargo fmt failed");
    }
}

fn run_lint() {
    let status = Command::new("cargo")
        .args(["clippy", "--all", "--", "-D", "warnings"])
        .status()
        .unwrap();
    if !status.success() {
        panic!("cargo clippy failed");
    }
}

fn run_ci() {
    run_fmt(FmtMode::Check);
    run_lint();
    run_build(BuildMode::Release);
    run_test(BuildMode::Release);
}

fn main() {
    match env::args().nth(1).as_deref() {
        Some("build") => run_build(BuildMode::Debug),
        Some("test") => run_test(BuildMode::Debug),
        Some("fmt") => run_fmt(FmtMode::Update),
        Some("lint") => run_lint(),
        Some("ci") => run_ci(),
        x => panic!("invalid argument: {x:?}"),
    }
}
