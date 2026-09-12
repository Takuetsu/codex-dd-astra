use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    if env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("macos") {
        println!("cargo:rustc-link-arg=-ObjC");
    }

    stamp_codexdd_build_identity();
}

fn stamp_codexdd_build_identity() {
    let version_file = env::var_os("CODEXDD_VERSION_FILE")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("../codexdd-version.txt"));

    println!("cargo:rerun-if-env-changed=CODEXDD_VERSION_FILE");
    println!("cargo:rerun-if-changed={}", version_file.display());
    track_git_head();

    let version = fs::read_to_string(&version_file)
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
    let command_version = command_version(&version, &build_commit);

    let profile = env::var("PROFILE").unwrap_or_else(|_| "unknown".to_string());
    let target = env::var("TARGET").unwrap_or_else(|_| "unknown".to_string());

    println!("cargo:rustc-env=CODEXDD_VERSION={version}");
    println!("cargo:rustc-env=CODEXDD_GIT_COMMIT={build_commit}");
    println!("cargo:rustc-env=CODEXDD_BUILD_PROFILE={profile}");
    println!("cargo:rustc-env=CODEXDD_BUILD_TARGET={target}");
    // Clap's existing `version` derive reads these Cargo package variables at compile time.
    // Override them only for the codex-cli final package so the custom binary identifies itself
    // without changing the upstream workspace's 0.0.0 package version.
    println!("cargo:rustc-env=CARGO_PKG_NAME=codexdd");
    println!("cargo:rustc-env=CARGO_PKG_VERSION={command_version}");
    // Existing codex-build-info initialization consumes this at the final executable call site.
    println!("cargo:rustc-env=STABLE_GIT_COMMIT={build_commit}");
}

fn command_version(version: &str, build_commit: &str) -> String {
    if build_commit == "dev" || build_commit == "unknown" {
        return version.to_string();
    }

    let (commit, dirty) = build_commit
        .strip_suffix("-dirty")
        .map_or((build_commit, false), |commit| (commit, true));
    let short: String = commit.chars().take(12).collect();
    if dirty {
        format!("{version}+g{short}.dirty")
    } else {
        format!("{version}+g{short}")
    }
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

#[cfg(test)]
mod tests {
    use super::command_version;

    #[test]
    fn command_version_includes_short_commit_and_dirty_marker() {
        let commit = "0123456789abcdef0123456789abcdef01234567";
        assert_eq!(command_version("0.1.0", commit), "0.1.0+g0123456789ab");
        assert_eq!(
            command_version("0.1.0", &format!("{commit}-dirty")),
            "0.1.0+g0123456789ab.dirty"
        );
        assert_eq!(command_version("0.1.0", "dev"), "0.1.0");
    }
}
