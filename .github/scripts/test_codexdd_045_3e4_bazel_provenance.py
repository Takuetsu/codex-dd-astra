"""Source-only CodexDD 0.4.5 Bazel CLI version and Windows release wiring guards.

Native Bazel graph loading is evidenced separately by run 39609973163.
These tests must never claim compilation or release acceptance.
"""

from pathlib import Path
import unittest

ROOT = Path(__file__).resolve().parents[2]


def source(path: str) -> str:
    return (ROOT / path).read_text(encoding="utf-8")


class BazelReleaseWiring(unittest.TestCase):
    def test_cli_build_script_data_and_version_env_share_input(self):
        cli = source("codex-rs/cli/BUILD.bazel")
        build_rs = source("codex-rs/cli/build.rs")
        self.assertIn('build_script_data = ["//codex-rs:codexdd-version.txt"]', cli)
        self.assertIn(
            '"CODEXDD_VERSION_FILE": "$(execpath //codex-rs:codexdd-version.txt)"', cli
        )
        self.assertIn('env::var_os("CODEXDD_VERSION_FILE")', build_rs)
        self.assertIn("cargo:rustc-env=CODEXDD_VERSION={version}", build_rs)
        self.assertIn("cargo:rustc-env=CARGO_PKG_VERSION={command_version}", build_rs)

    def test_crate_macro_admits_and_forwards_build_script_env(self):
        defs = source("defs.bzl")
        macro = defs[defs.index("def codex_rust_crate(") :]
        signature, rest = macro.split("    test_env = {", 1)
        self.assertIn("        build_script_env = {},", signature)
        self.assertIn(
            'load("@rules_rust//cargo/private:cargo_build_script_wrapper.bzl", "cargo_build_script")',
            defs,
        )
        build_section = rest.split("        cargo_build_script(", 1)[1].split(
            "        maybe_deps +=", 1
        )[0]
        self.assertIn("data = build_script_data,", build_section)
        self.assertIn("build_script_env = build_script_env,", build_section)

    def test_runtime_archive_keeps_cli_version_data_and_release_target(self):
        cli = source("codex-rs/cli/BUILD.bazel")
        codex_root = source("codex-rs/BUILD.bazel")
        self.assertIn('"codexdd-version.txt"', codex_root)
        self.assertIn('release_binary(\n    binary = ":codex",\n)', cli)
        self.assertIn('"//codex-rs/cli:release_files"', codex_root)
        self.assertIn('name = "runtime_package_combined"', codex_root)

    def test_windows_sqlite_intrinsic_guard_remains_scoped(self):
        module = source("MODULE.bazel")
        self.assertEqual(
            module.count('"LIBSQLITE3_FLAGS": "SQLITE_DISABLE_INTRINSIC"'), 1
        )
        self.assertIn(
            '"x86_64-pc-windows-msvc": \'{"LIBSQLITE3_FLAGS": "SQLITE_DISABLE_INTRINSIC"}\'',
            module,
        )
        self.assertIn('crate = "libsqlite3-sys"', module)
        self.assertIn('bazel_dep(name = "rules_pkg", version = "1.0.1")', module)
        self.assertIn("extra_exec_rustc_flags = MSVC_RUSTC_FLAGS,", module)
        self.assertIn("extra_rustc_flags = MSVC_RUSTC_FLAGS,", module)

    def test_temporary_bazel_write_oracle_retired(self):
        self.assertFalse(
            (ROOT / ".github/workflows/codexdd-045-bazel-sqlite-reconcile.yml").exists()
        )


if __name__ == "__main__":
    unittest.main()
