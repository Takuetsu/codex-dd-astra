import unittest

from upstream_sync import (
    integration_branch,
    pr_marker,
    release_version,
    select_latest_release,
)


class UpstreamSyncTests(unittest.TestCase):
    def test_selects_highest_official_release_and_ignores_other_tags(self):
        self.assertEqual(
            select_latest_release(
                [
                    "rust-v0.155.0-alpha.16",
                    "rust-v0.155.0",
                    "rust-v0.155.0-alpha.16.1",
                    "rusty-v8-v152.2.0",
                    "rust-vnot-a-version",
                ]
            ),
            "rust-v0.155.0",
        )

    def test_prerelease_order_is_semver_like(self):
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

    def test_duplicate_keys_are_deterministic(self):
        tag = "rust-v0.155.0-alpha.16"
        self.assertEqual(
            integration_branch(tag), "automation/upstream-sync-rust-v0.155.0-alpha.16"
        )
        self.assertEqual(
            pr_marker(tag), "codexdd-upstream-sync: rust-v0.155.0-alpha.16"
        )

    def test_rejects_non_release_tags(self):
        with self.assertRaises(ValueError):
            integration_branch("main")


if __name__ == "__main__":
    unittest.main()
