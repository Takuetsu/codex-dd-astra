import unittest

from upstream_sync import (
    integration_branch,
    pr_marker,
    release_version,
    select_latest_release,
    stable_release_version,
)


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

    def test_duplicate_keys_are_deterministic(self):
        tag = "rust-v0.155.0"
        self.assertEqual(
            integration_branch(tag), "automation/upstream-sync-rust-v0.155.0"
        )
        self.assertEqual(pr_marker(tag), "codexdd-upstream-sync: rust-v0.155.0")

    def test_rejects_non_release_and_prerelease_tags(self):
        with self.assertRaises(ValueError):
            integration_branch("main")
        with self.assertRaises(ValueError):
            integration_branch("rust-v0.156.0-alpha.2")


if __name__ == "__main__":
    unittest.main()
