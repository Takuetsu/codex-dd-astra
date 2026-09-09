use std::env;
use std::fs;
use std::process::Command;

fn main() {
    if env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("macos") {
        println!("cargo:rustc-link-arg=-ObjC");
    }

    stamp_codexdd_build_identity();
}

fn stamp_codexdd_build_identity() {
    const VERSION_FILE: &str = "../codexdd-version.txt";

    println!("cargo:rerun-if-changed={VERSION_FILE}");
    track_git_head();

    let version = fs::read_to_string(VERSION_FILE)
        .expect("codexdd version file must be readable")
        .trim()
        .to_string();
    assert!(!version.is_empty(), "codexdd version must not be empty");

    let commit = git_output(&["rev-parse", "HEAD"]).unwrap_or_else(|| "dev".to_string());
    let dirty = git_output(&["status", "--porcelain", "--untracked-files=no"])
        .is_some_and(|status| !status.is_empty());
    let build_commit = if dirty && commit != "dev" {
        format!("{commit}-dirty")
    } else {
        commit
    };

    let profile = env::var("PROFILE").unwrap_or_else(|_| "unknown".to_string());
    let target = env::var("TARGET").unwrap_or_else(|_| "unknown".to_string());

    println!("cargo:rustc-env=CODEXDD_VERSION={version}");
    println!("cargo:rustc-env=CODEXDD_GIT_COMMIT={build_commit}");
    println!("cargo:rustc-env=CODEXDD_BUILD_PROFILE={profile}");
    println!("cargo:rustc-env=CODEXDD_BUILD_TARGET={target}");
    // Existing codex-build-info initialization consumes this at the final executable call site.
    println!("cargo:rustc-env=STABLE_GIT_COMMIT={build_commit}");
}

fn track_git_head() {
    if let Some(head_path) = git_output(&["rev-parse", "--git-path", "HEAD"]) {
        println!("cargo:rerun-if-changed={head_path}");
    }

    if let Some(symbolic_head) = git_output(&["symbolic-ref", "-q", "HEAD"])
        && let Some(ref_path) = git_output(&["rev-parse", "--git-path", &symbolic_head])
    {
        println!("cargo:rerun-if-changed={ref_path}");
    }
}

fn git_output(args: &[&str]) -> Option<String> {
    let output = Command::new("git").args(args).output().ok()?;
    if !output.status.success() {
        return None;
    }
    Some(String::from_utf8(output.stdout).ok()?.trim().to_string())
}
