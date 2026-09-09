use std::fs;

use codex_install_context::InstallContext;
use pretty_assertions::assert_eq;
use semver::Version;
use tempfile::tempdir;

use crate::BuildInfo;
use crate::codexdd_compact_identity_for_commit;
use crate::codexdd_version;

const BUILD_COMMIT: &str = "0123456789abcdef0123456789abcdef01234567";

/// A packaged runtime takes its release identity from its package manifest.
#[test]
fn packaged_runtime_uses_manifest_version() {
    let package = tempdir().expect("create runtime package");
    let bin_dir = package.path().join("bin");
    fs::create_dir(&bin_dir).expect("create runtime binary directory");
    let executable = bin_dir.join("codex");
    fs::write(&executable, b"").expect("create runtime binary");
    fs::write(
        package.path().join("codex-package.json"),
        r#"{"version":"1.2.3-alpha.4"}"#,
    )
    .expect("create runtime package manifest");

    let context = InstallContext::from_exe(
        cfg!(target_os = "macos"),
        Some(&executable),
        /*method_override*/ None,
    );

    assert_eq!(
        BuildInfo::resolve(&context, BUILD_COMMIT),
        BuildInfo {
            version: Version::parse("1.2.3-alpha.4").expect("valid release version"),
            build_commit: BUILD_COMMIT.to_string(),
        },
    );
}

/// Unpackaged builds expose their stamped commit and structured source version.
#[test]
fn unpackaged_runtime_uses_build_commit() {
    let context = InstallContext::from_exe(
        cfg!(target_os = "macos"),
        /*current_exe*/ None,
        /*method_override*/ None,
    );

    assert_eq!(
        BuildInfo::resolve(&context, BUILD_COMMIT),
        BuildInfo {
            version: Version::new(0, 0, 0),
            build_commit: BUILD_COMMIT.to_string(),
        },
    );
}

/// Older package layouts without release metadata retain their build identity.
#[test]
fn legacy_package_without_version_uses_build_commit() {
    let package = tempdir().expect("create runtime package");
    let bin_dir = package.path().join("bin");
    fs::create_dir(&bin_dir).expect("create runtime binary directory");
    let executable = bin_dir.join("codex");
    fs::write(&executable, b"").expect("create runtime binary");
    fs::write(package.path().join("codex-package.json"), "{}")
        .expect("create legacy runtime package manifest");

    let context = InstallContext::from_exe(
        cfg!(target_os = "macos"),
        Some(&executable),
        /*method_override*/ None,
    );

    assert_eq!(
        BuildInfo::resolve(&context, BUILD_COMMIT),
        BuildInfo {
            version: Version::new(0, 0, 0),
            build_commit: BUILD_COMMIT.to_string(),
        },
    );
}

/// Invalid package versions cannot override the executable's stamped commit.
#[test]
fn invalid_package_version_uses_build_commit() {
    let package = tempdir().expect("create runtime package");
    let bin_dir = package.path().join("bin");
    fs::create_dir(&bin_dir).expect("create runtime binary directory");
    let executable = bin_dir.join("codex");
    fs::write(&executable, b"").expect("create runtime binary");
    fs::write(
        package.path().join("codex-package.json"),
        r#"{"version":"not-a-release-version"}"#,
    )
    .expect("create runtime package manifest");

    let context = InstallContext::from_exe(
        cfg!(target_os = "macos"),
        Some(&executable),
        /*method_override*/ None,
    );

    assert_eq!(
        BuildInfo::resolve(&context, BUILD_COMMIT),
        BuildInfo {
            version: Version::new(0, 0, 0),
            build_commit: BUILD_COMMIT.to_string(),
        },
    );
}

/// Serializing build information preserves both its release version and commit.
#[test]
fn build_info_serialization_preserves_build_provenance() {
    let build_info = BuildInfo {
        version: Version::parse("1.2.3-alpha.4").expect("valid release version"),
        build_commit: BUILD_COMMIT.to_string(),
    };

    assert_eq!(
        serde_json::to_string(&build_info).expect("serialize build information"),
        format!("{{\"version\":\"1.2.3-alpha.4\",\"build_commit\":\"{BUILD_COMMIT}\"}}"),
    );
    assert_eq!(
        serde_json::from_str::<BuildInfo>(&format!(
            "{{\"version\":\"1.2.3-alpha.4\",\"build_commit\":\"{BUILD_COMMIT}\"}}"
        ))
        .expect("deserialize build information"),
        build_info,
    );
}

#[test]
fn codexdd_product_version_is_independent_of_workspace_version() {
    assert_eq!(codexdd_version(), "0.1.0");
}

#[test]
fn codexdd_compact_identity_preserves_dirty_marker() {
    assert_eq!(
        codexdd_compact_identity_for_commit(BUILD_COMMIT),
        "0.1.0 (0123456789ab)"
    );
    assert_eq!(
        codexdd_compact_identity_for_commit(&format!("{BUILD_COMMIT}-dirty")),
        "0.1.0 (0123456789ab-dirty)"
    );
    assert_eq!(
        codexdd_compact_identity_for_commit("dev"),
        "0.1.0 (dev)"
    );
}
