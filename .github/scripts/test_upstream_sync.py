import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

from upstream_sync import (
    build_candidate_manifest,
    build_delta_inventory,
    candidate_branch,
    candidate_key,
    candidate_manifest_json,
    candidate_state,
    candidate_state_after_identity_check,
    classify_sensitive_overlaps,
    git_tree_entries,
    parse_validation_receipt_json,
    release_version,
    reconcile_workspace_lockfile,
    select_latest_release,
    select_official_stable_release,
    stable_release_version,
    stale_identity_fields,
    tree_delta_paths,
    validate_validation_receipt,
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


def manifest_kwargs() -> dict[str, object]:
    return {
        "discovery_timestamp": "2026-10-07T20:00:00Z",
        "production_branch": "dd/astra-policy-v2",
        "production_sha": "1" * 40,
        "production_tree": "2" * 40,
        "codexdd_product_version": "0.4.1",
        "tracked_upstream_tag": "rust-v0.159.2",
        "tracked_upstream_sha": "3" * 40,
        "tracked_upstream_tree": "4" * 40,
        "target_upstream_tag": "rust-v0.160.1",
        "target_upstream_sha": "5" * 40,
        "target_upstream_tree": "6" * 40,
        "ancestry_status": "diverged",
        "merge_base_sha": "7" * 40,
        "tracked_entries": {
            "shared.txt": "100644 blob aaa",
            "codex-rs/tui/src/status/card.rs": "100644 blob bbb",
        },
        "production_entries": {
            "shared.txt": "100644 blob aaa",
            "codex-rs/tui/src/status/card.rs": "100644 blob ccc",
            "codexdd-only.txt": "100644 blob ddd",
        },
        "target_entries": {
            "shared.txt": "100644 blob eee",
            "codex-rs/tui/src/status/card.rs": "100644 blob fff",
        },
    }


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

    def test_explicit_pinned_tag_does_not_advance_to_newer_stable(self):
        official = [
            "rust-v0.162.0",
            "rust-v0.165.0",
            "rust-v0.164.0-rc.1",
        ]
        self.assertEqual(
            select_official_stable_release(official, "rust-v0.162.0"),
            "rust-v0.162.0",
        )
        self.assertEqual(
            select_official_stable_release(official),
            "rust-v0.165.0",
        )

    def test_explicit_tag_rejects_missing_prerelease_and_malformed_input(self):
        official = ["rust-v0.162.0", "rust-v0.163.0-rc.1"]
        for invalid in (
            "rust-v0.161.0",
            "rust-v0.163.0-rc.1",
            "rust-v0.162.0 ",
            "rust-v0.162.0; echo untrusted",
            "rust-vnot-a-version",
        ):
            with self.subTest(requested=invalid):
                with self.assertRaises(ValueError):
                    select_official_stable_release(official, invalid)

    def test_pinned_cli_selects_only_official_tags_and_fails_closed(self):
        script = Path(__file__).with_name("upstream_sync.py")
        official = "rust-v0.162.0\nrust-v0.165.0\n"
        command = [sys.executable, str(script), "--latest-from-stdin"]
        pinned = subprocess.run(
            [*command, "--target-tag", "rust-v0.162.0"],
            input=official,
            text=True,
            capture_output=True,
            check=True,
        )
        self.assertEqual(pinned.stdout.strip(), "rust-v0.162.0")

        scheduled = subprocess.run(
            command,
            input=official,
            text=True,
            capture_output=True,
            check=True,
        )
        self.assertEqual(scheduled.stdout.strip(), "rust-v0.165.0")

        missing = subprocess.run(
            [*command, "--target-tag", "rust-v0.161.0"],
            input=official,
            text=True,
            capture_output=True,
            check=False,
        )
        self.assertNotEqual(missing.returncode, 0)
        self.assertEqual(missing.stdout, "")
        self.assertIn("not an official upstream tag", missing.stderr)

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

    def test_candidate_keys_and_branches_are_deterministic(self):
        tag = "rust-v0.155.0"
        self.assertEqual(
            candidate_key(tag, "a" * 40),
            f"{tag}@{'a' * 40}",
        )
        self.assertEqual(
            candidate_branch(tag, "a" * 40),
            f"automation/upstream-candidate-{tag}-{'a' * 12}",
        )

    def test_candidate_identity_rejects_prerelease_and_invalid_sha(self):
        with self.assertRaises(ValueError):
            candidate_key("rust-v0.156.0-rc.1", "a" * 40)
        with self.assertRaises(ValueError):
            candidate_key("rust-v0.156.0", "not-a-sha")
        with self.assertRaises(ValueError):
            candidate_branch("rust-v0.156.0", "not-a-sha")

    def test_reconcile_workspace_lockfile_updates_only_local_packages(self):
        source = """# header
[[package]]
name = "codex-cli"
version = "0.155.0"
dependencies = []

[[package]]
name = "external"
version = "0.155.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "abc"
"""
        updated, changed = reconcile_workspace_lockfile(source, "0.155.1")
        self.assertEqual(changed, 1)
        self.assertIn('name = "codex-cli"\nversion = "0.155.1"', updated)
        self.assertIn(
            'name = "external"\nversion = "0.155.0"\nsource = "registry+',
            updated,
        )

    def test_tree_delta_paths_detects_add_delete_content_and_mode_changes(self):
        base = {
            "same.txt": "100644 blob aaa",
            "changed.txt": "100644 blob bbb",
            "mode.txt": "100644 blob ccc",
            "deleted.txt": "100644 blob ddd",
        }
        other = {
            "same.txt": "100644 blob aaa",
            "changed.txt": "100644 blob eee",
            "mode.txt": "100755 blob ccc",
            "added.txt": "100644 blob fff",
        }
        self.assertEqual(
            tree_delta_paths(base, other),
            ("added.txt", "changed.txt", "deleted.txt", "mode.txt"),
        )

    def test_git_tree_entries_reads_exact_recursive_tree_identity(self):
        with tempfile.TemporaryDirectory() as temp_dir:
            repo = Path(temp_dir)
            git(repo, "init", "-q")
            git(repo, "config", "user.name", "sync-test")
            git(repo, "config", "user.email", "sync-test@example.invalid")
            nested = repo / "nested"
            nested.mkdir()
            (repo / "root.txt").write_text("root\n")
            (nested / "child.txt").write_text("child\n")
            commit = commit_all(repo, "tree")

            entries = git_tree_entries(repo, commit)

            self.assertEqual(set(entries), {"nested/child.txt", "root.txt"})
            self.assertRegex(
                entries["root.txt"],
                r"^100644 blob [0-9a-f]{40}$",
            )

    def test_delta_inventory_computes_exact_overlap_and_sensitive_categories(self):
        tracked = {
            "shared.txt": "base",
            "codex-rs/tui/src/status/card.rs": "base-status",
            "codex-rs/core/src/session/session.rs": "base-session",
        }
        production = {
            "shared.txt": "base",
            "codex-rs/tui/src/status/card.rs": "codexdd-status",
            "codex-rs/core/src/session/session.rs": "codexdd-session",
            "codexdd-only.txt": "custom",
        }
        target = {
            "shared.txt": "upstream-change",
            "codex-rs/tui/src/status/card.rs": "upstream-status",
            "codex-rs/core/src/session/session.rs": "base-session",
        }

        inventory = build_delta_inventory(tracked, production, target)

        self.assertEqual(
            inventory.customization_paths,
            (
                "codex-rs/core/src/session/session.rs",
                "codex-rs/tui/src/status/card.rs",
                "codexdd-only.txt",
            ),
        )
        self.assertEqual(
            inventory.upstream_change_paths,
            ("codex-rs/tui/src/status/card.rs", "shared.txt"),
        )
        self.assertEqual(
            inventory.overlap_paths,
            ("codex-rs/tui/src/status/card.rs",),
        )
        self.assertEqual(
            inventory.sensitive_overlap_categories["status_rendering"],
            ("codex-rs/tui/src/status/card.rs",),
        )
        self.assertEqual(
            inventory.sensitive_overlap_paths,
            ("codex-rs/tui/src/status/card.rs",),
        )

    def test_sensitive_overlap_classification_covers_codexdd_risk_surfaces(self):
        classified = classify_sensitive_overlaps(
            [
                ".github/workflows/blocking-ci.yml",
                "codex-rs/Cargo.lock",
                "codex-rs/app-server-protocol/schema/typescript/v2/index.ts",
                "codex-rs/core/src/session/session.rs",
                "codex-rs/tui/src/adaptive_policy.rs",
                "codex-rs/tui/src/app.rs",
                "codex-rs/tui/src/app/session_lifecycle.rs",
                "codex-rs/tui/src/status/card.rs",
                "codex-rs/core/src/thread_manager.rs",
                "codex-rs/app-server/src/message_processor.rs",
                "codex-rs/config/src/config_toml.rs",
                "codex-rs/upstream-codex-release.txt",
            ]
        )
        self.assertIn("adaptive_routing", classified)
        self.assertIn("tui_runtime", classified)
        self.assertIn("core_runtime", classified)
        self.assertIn("app_server_runtime", classified)
        self.assertIn("configuration", classified)
        self.assertIn(
            "codex-rs/tui/src/app.rs",
            classified["tui_runtime"],
        )
        self.assertIn(
            "codex-rs/core/src/thread_manager.rs",
            classified["core_runtime"],
        )
        self.assertIn("worker_lifecycle", classified)
        self.assertIn("persistence_resume_fork", classified)
        self.assertIn("status_rendering", classified)
        self.assertIn("generated_protocol", classified)
        self.assertIn("version_provenance", classified)
        self.assertIn("build_release_ci", classified)

    def test_candidate_state_distinguishes_textual_and_semantic_risk(self):
        self.assertEqual(candidate_state("not_run", 8), "discovered")
        self.assertEqual(
            candidate_state("conflict", 0),
            "blocked_transplant_conflict",
        )
        self.assertEqual(candidate_state("clean", 0), "preparation_ready")
        self.assertEqual(
            candidate_state("clean", 1),
            "manual_semantic_review_required",
        )
        with self.assertRaises(ValueError):
            candidate_state("unknown", 0)

    def test_manifest_is_immutable_identity_evidence_without_next_product_version(self):
        kwargs = manifest_kwargs()
        manifest = build_candidate_manifest(
            **kwargs,
            transplant_result="clean",
        )
        self.assertEqual(manifest.contract_version, 1)
        self.assertEqual(
            manifest.candidate_key,
            f"rust-v0.160.1@{'5' * 40}",
        )
        self.assertEqual(manifest.codexdd_product_version, "0.4.1")
        self.assertEqual(manifest.customization_path_count, 2)
        self.assertEqual(manifest.upstream_change_path_count, 2)
        self.assertEqual(manifest.overlap_path_count, 1)
        self.assertEqual(
            manifest.overlap_paths,
            ("codex-rs/tui/src/status/card.rs",),
        )
        self.assertEqual(manifest.sensitive_overlap_count, 1)
        self.assertEqual(
            manifest.state,
            "manual_semantic_review_required",
        )

        encoded = candidate_manifest_json(manifest)
        decoded = json.loads(encoded)
        self.assertEqual(decoded["target_upstream_sha"], "5" * 40)
        self.assertNotIn("next_product_version", decoded)
        self.assertNotIn("product_version_target", decoded)
        self.assertEqual(encoded, candidate_manifest_json(manifest))

    def test_manifest_rejects_non_newer_or_prerelease_target(self):
        kwargs = manifest_kwargs()
        kwargs["target_upstream_tag"] = "rust-v0.159.2"
        with self.assertRaisesRegex(ValueError, "strictly newer"):
            build_candidate_manifest(**kwargs)

        kwargs = manifest_kwargs()
        kwargs["target_upstream_tag"] = "rust-v0.161.0-rc.1"
        with self.assertRaisesRegex(ValueError, "invalid target stable"):
            build_candidate_manifest(**kwargs)

    def test_stale_identity_check_blocks_changed_production_or_upstream_identity(self):
        manifest = build_candidate_manifest(**manifest_kwargs())
        self.assertEqual(
            stale_identity_fields(
                manifest,
                production_sha="1" * 40,
                tracked_upstream_tag="rust-v0.159.2",
                tracked_upstream_sha="3" * 40,
                target_upstream_tag="rust-v0.160.1",
                target_upstream_sha="5" * 40,
            ),
            (),
        )
        state, stale = candidate_state_after_identity_check(
            manifest,
            production_sha="8" * 40,
            tracked_upstream_tag="rust-v0.159.2",
            tracked_upstream_sha="3" * 40,
            target_upstream_tag="rust-v0.160.1",
            target_upstream_sha="9" * 40,
        )
        self.assertEqual(state, "blocked_stale_base")
        self.assertEqual(stale, ("production_sha", "target_upstream_sha"))

    def test_validation_receipt_is_bound_to_candidate_identity_and_native_pass(self):
        receipt = {
            "contract_version": 1,
            "receipt_type": "codexdd_upstream_local_validation",
            "candidate_key": f"rust-v0.160.1@{'5' * 40}",
            "candidate_branch": f"automation/upstream-candidate-rust-v0.160.1-{'5' * 12}",
            "candidate_head_sha": "8" * 40,
            "production_sha": "1" * 40,
            "target_upstream_tag": "rust-v0.160.1",
            "target_upstream_sha": "5" * 40,
            "manifest_path": "docs/upstream-candidates/codexdd-upstream-rust-v0.160.1-555555555555.json",
            "manifest_sha256": "a" * 64,
            "profile": "work-packet",
            "validation_contract_version": 1,
            "validation_status": "pass",
            "completed_stages": 6,
            "completed_at": "2026-10-07T20:00:00Z",
        }

        validate_validation_receipt(receipt)
        parsed = parse_validation_receipt_json(json.dumps(receipt))
        self.assertEqual(parsed["candidate_head_sha"], "8" * 40)

        bad = dict(receipt)
        bad["candidate_branch"] = "automation/upstream-candidate-wrong"
        with self.assertRaisesRegex(ValueError, "branch"):
            validate_validation_receipt(bad)

        bad = dict(receipt)
        bad["validation_status"] = "fail"
        with self.assertRaisesRegex(ValueError, "not PASS"):
            validate_validation_receipt(bad)

        bad = dict(receipt)
        bad["manifest_path"] = "../receipt.json"
        with self.assertRaisesRegex(ValueError, "candidate area"):
            validate_validation_receipt(bad)

        bad = dict(receipt)
        bad["manifest_sha256"] = "not-a-digest"
        with self.assertRaisesRegex(ValueError, "SHA-256"):
            validate_validation_receipt(bad)

    def test_workflow_boundaries_remain_fail_closed(self):
        repository_root = Path(__file__).resolve().parents[2]
        discovery = (repository_root / ".github/workflows/upstream-sync.yml").read_text(
            encoding="utf-8"
        )
        preparation = (
            repository_root / ".github/workflows/upstream-prepare.yml"
        ).read_text(encoding="utf-8")
        promotion = (
            repository_root / ".github/workflows/upstream-promote.yml"
        ).read_text(encoding="utf-8")

        self.assertIn("permissions:\n  contents: read", discovery)
        self.assertNotIn("contents: write", discovery)
        self.assertNotIn("pull-requests: write", discovery)
        self.assertNotIn("issues: write", discovery)
        self.assertNotIn("git push", discovery)
        self.assertNotIn("gh pr create", discovery)
        self.assertIn("git ls-remote --tags --refs upstream", discovery)
        self.assertIn("expected_target_sha:", discovery)
        self.assertIn(
            '--latest-from-stdin --target-tag "$REQUESTED_TARGET_TAG"', discovery
        )
        self.assertIn('canonical_oid="$(git ls-remote --exit-code', discovery)
        self.assertIn(
            'selected_commit="$(git rev-parse "${latest_tag}^{commit}")"', discovery
        )
        self.assertNotIn("git tag --list", discovery)

        self.assertIn("permissions:\n  contents: write", preparation)
        self.assertNotIn("pull-requests: write", preparation)
        self.assertNotIn("issues: write", preparation)
        self.assertNotIn("gh pr create", preparation)
        self.assertNotIn('codexdd-version.txt" >', preparation)
        self.assertIn("git push --force-with-lease=", preparation)
        self.assertIn("current_tracked_sha=", preparation)
        self.assertIn('for tag in "$TRACKED_TAG" "$TARGET_TAG"; do', preparation)

        self.assertIn("contents: read", promotion)
        self.assertIn("pull-requests: write", promotion)
        self.assertNotIn("contents: write", promotion)
        self.assertNotIn("issues: write", promotion)
        self.assertIn("gh pr create", promotion)
        self.assertNotIn("gh pr merge", promotion)
        self.assertIn("semantic_review_accepted", promotion)
        self.assertIn("SEMANTIC_REVIEW_EVIDENCE", promotion)
        self.assertIn('LOCAL_VALIDATION_PROFILE"] != "release"', promotion)

        combined = discovery + preparation + promotion
        self.assertNotIn("--next-patch", combined)
        self.assertNotIn("next_product_version", combined)
        self.assertNotIn("gh issue", combined)

    def test_candidate_validation_script_emits_native_identity_bound_receipt(self):
        repository_root = Path(__file__).resolve().parents[2]
        script = (
            repository_root / "scripts/codexdd-validate-upstream-candidate.ps1"
        ).read_text(encoding="utf-8")

        self.assertIn("git status --porcelain", script)
        self.assertIn("git merge-base --is-ancestor", script)
        self.assertIn("CODEXDD_VALIDATION_JSON ", script)
        self.assertIn("CODEXDD_UPSTREAM_VALIDATION_RECEIPT ", script)
        self.assertIn("manifest_sha256", script)
        self.assertIn("candidate_head_sha", script)
        self.assertNotIn("[IO.Path]::GetRelativePath", script)
        self.assertIn(').Replace("\\", "/")', script)
        self.assertIn("Candidate manifest is outside the repository root.", script)
        self.assertIn("docs/upstream-candidates/", script)

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

            product_tree = git(
                repo, "rev-parse", f"{production}^{{tree}}"
            ).stdout.strip()
            synthetic = git(
                repo,
                "commit-tree",
                product_tree,
                "-p",
                upstream_v1,
                input_text="codexdd synthetic delta\n",
            ).stdout.strip()

            git(repo, "switch", "-q", "--detach", upstream_v2)
            git(repo, "cherry-pick", synthetic)

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

            product_tree = git(
                repo, "rev-parse", f"{production}^{{tree}}"
            ).stdout.strip()
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
