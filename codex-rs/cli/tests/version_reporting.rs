use predicates::prelude::*;

#[test]
fn version_reports_codexdd_product_and_git_build() -> anyhow::Result<()> {
    let mut cmd = assert_cmd::Command::new(codex_utils_cargo_bin::cargo_bin("codex")?);
    cmd.arg("--version");

    cmd.assert()
        .success()
        .stdout(predicate::str::starts_with("codexdd 0.1.0+g"))
        .stdout(predicate::str::contains("codex-cli 0.0.0").not());
    Ok(())
}
