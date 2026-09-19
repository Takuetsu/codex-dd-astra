import subprocess
import tempfile
import unittest
from pathlib import Path


from upstream_sync import (
    conflict_issue_title,
    conflict_marker,
    integration_branch,
    next_patch_version,
    pr_marker,
    release_version,
    rewrite_version_test,
    select_latest_release,
    stable_release_version,
)


def git(repo: Path, *args: str, input_text: str | None = None, check: bool = True):
    return subprocess.run(
        ["git", *args],
        cwd=repo,
        input=input_text,
        text=True,
        capture_output=True,
        check=check,
    )


def commit_all(repo: Path, message: str) -> str:
    git(repo, "add", "-A")
    git(repo, "commit", "-q", "-m", message)
    return git(repo, "rev-parse", "HEAD").stdout.strip()


class UpstreamSyncTests(unittest.TestCase):
    def test_selects_highest_stable_release_and_ignores_prereleases(self):
        self.assertEqual(
            select_latest_release(
                [
                    "rust-v0.156.0-alpha.2",
                    "rust-v0.155.0-beta.1",
                    "rust-v0.155.0-rc.1",
                    "rust-v0.155.0",
                    "rust-v0.154.0",
                    "rusty-v8-v152.2.0",
                    "rust-vnot-a-version",
                ]
            ),
            "rust-v0.155.0",
        )

    def test_rejects_prereleases_when_no_stable_release_exists(self):
        with self.assertRaises(ValueError):
            select_latest_release(
                [
                    "rust-v0.156.0-alpha.2",
                    "rust-v0.155.0-beta.1",
                    "rust-v0.155.0-rc.1",
                ]
            )

    def test_prerelease_order_is_semver_like_for_historical_comparison(self):
        self.assertLess(
            release_version("rust-v0.155.0-alpha.16"),
            release_version("rust-v0.155.0-beta.1"),
        )
        self.assertLess(
            release_version("rust-v0.155.0-beta.1"),
            release_version("rust-v0.155.0-rc.1"),
        )
        self.assertLess(
            release_version("rust-v0.155.0-rc.1"), release_version("rust-v0.155.0")
        )

    def test_stable_release_version_rejects_prereleases(self):
        self.assertEqual(stable_release_version("rust-v0.155.0"), (0, 155, 0))
        self.assertIsNone(stable_release_version("rust-v0.156.0-alpha.2"))
        self.assertIsNone(stable_release_version("rust-v0.155.0-beta.1"))
        self.assertIsNone(stable_release_version("rust-v0.155.0-rc.1"))

    def test_duplicate_and_conflict_keys_are_deterministic(self):
        tag = "rust-v0.155.0"
        self.assertEqual(
            integration_branch(tag), "automation/upstream-sync-rust-v0.155.0"
        )
        self.assertEqual(pr_marker(tag), "codexdd-upstream-sync: rust-v0.155.0")
        self.assertEqual(
            conflict_issue_title(tag), "codexdd: resolve upstream rust-v0.155.0 conflicts"
        )
        self.assertEqual(
            conflict_marker(tag), "codexdd-upstream-conflict: rust-v0.155.0"
        )

    def test_rejects_non_release_and_prerelease_tags(self):
        for helper in (
            integration_branch,
            pr_marker,
            conflict_issue_title,
            conflict_marker,
        ):
            with self.assertRaises(ValueError):
                helper("main")
            with self.assertRaises(ValueError):
                helper("rust-v0.156.0-alpha.2")

    def test_next_patch_version_is_deterministic(self):
        self.assertEqual(next_patch_version("0.2.1"), "0.2.2")
        self.assertEqual(next_patch_version("1.9.99"), "1.9.100")
        with self.assertRaises(ValueError):
            next_patch_version("0.2.1-alpha.1")

    def test_rewrite_version_test_updates_exactly_one_prefix(self):
        source = '.stdout(predicate::str::starts_with("codexdd 0.2.1+g"))\n'
        self.assertEqual(
            rewrite_version_test(source, "0.2.1", "0.2.2"),
            '.stdout(predicate::str::starts_with("codexdd 0.2.2+g"))\n',
        )
        with self.assertRaises(ValueError):
            rewrite_version_test(source, "0.2.0", "0.2.1")
        with self.assertRaises(ValueError):
            rewrite_version_test(source + source, "0.2.1", "0.2.2")

    def test_synthetic_delta_survives_squashed_upstream_history(self):
        with tempfile.TemporaryDirectory() as temp_dir:
            repo = Path(temp_dir)
            git(repo, "init", "-q")
            git(repo, "config", "user.name", "sync-test")
            git(repo, "config", "user.email", "sync-test@example.invalid")

            (repo / "shared.txt").write_text("old upstream\n")
            root = commit_all(repo, "old base")

            git(repo, "switch", "-q", "-c", "upstream-v1", root)
            (repo / "shared.txt").write_text("upstream v1\n")
            upstream_v1 = commit_all(repo, "upstream v1")

            git(repo, "switch", "-q", "-c", "production", root)
            (repo / "shared.txt").write_text("upstream v1\n")
            (repo / "codexdd.txt").write_text("adaptive customization\n")
            production = commit_all(repo, "squashed upstream v1 plus codexdd")

            ancestry = git(
                repo,
                "merge-base",
                "--is-ancestor",
                upstream_v1,
                production,
                check=False,
            )
            self.assertEqual(ancestry.returncode, 1)

            git(repo, "switch", "-q", "-c", "upstream-v2", upstream_v1)
            (repo / "shared.txt").write_text("upstream v2\n")
            (repo / "new-upstream.txt").write_text("new stable behavior\n")
            upstream_v2 = commit_all(repo, "upstream v2")

            product_tree = git(repo, "rev-parse", f"{production}^{{tree}}").stdout.strip()
            synthetic = git(
                repo,
                "commit-tree",
                product_tree,
                "-p",
                upstream_v1,
                input_text="codexdd synthetic delta\n",
            ).stdout.strip()

            git(repo, "switch", "-q", "--detach", upstream_v2)
            git(repo, "cherry-pick", "-q", synthetic)

            self.assertEqual((repo / "shared.txt").read_text(), "upstream v2\n")
            self.assertEqual(
                (repo / "codexdd.txt").read_text(), "adaptive customization\n"
            )
            self.assertEqual(
                (repo / "new-upstream.txt").read_text(), "new stable behavior\n"
            )

    def test_synthetic_delta_exposes_real_overlap_as_conflict(self):
        with tempfile.TemporaryDirectory() as temp_dir:
            repo = Path(temp_dir)
            git(repo, "init", "-q")
            git(repo, "config", "user.name", "sync-test")
            git(repo, "config", "user.email", "sync-test@example.invalid")

            (repo / "shared.txt").write_text("base\n")
            root = commit_all(repo, "base")

            git(repo, "switch", "-q", "-c", "upstream-v1", root)
            (repo / "shared.txt").write_text("stable v1\n")
            upstream_v1 = commit_all(repo, "upstream v1")

            git(repo, "switch", "-q", "-c", "production", root)
            (repo / "shared.txt").write_text("codexdd override\n")
            production = commit_all(repo, "squashed product")

            git(repo, "switch", "-q", "-c", "upstream-v2", upstream_v1)
            (repo / "shared.txt").write_text("stable v2 changed same line\n")
            upstream_v2 = commit_all(repo, "upstream v2")

            product_tree = git(repo, "rev-parse", f"{production}^{{tree}}").stdout.strip()
            synthetic = git(
                repo,
                "commit-tree",
                product_tree,
                "-p",
                upstream_v1,
                input_text="codexdd synthetic delta\n",
            ).stdout.strip()

            git(repo, "switch", "-q", "--detach", upstream_v2)
            cherry_pick = git(repo, "cherry-pick", synthetic, check=False)
            self.assertNotEqual(cherry_pick.returncode, 0)
            conflicts = git(
                repo, "diff", "--name-only", "--diff-filter=U"
            ).stdout.splitlines()
            self.assertEqual(conflicts, ["shared.txt"])
            git(repo, "cherry-pick", "--abort")


if __name__ == "__main__":
    unittest.main()
