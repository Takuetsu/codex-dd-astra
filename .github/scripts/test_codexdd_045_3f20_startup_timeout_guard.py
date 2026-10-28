"""Prove 0.163 startup modal timeout quarantine was restored as the only source delta."""

import json
import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "codex-rs/tui/src/app/startup.rs"
LEDGER = ROOT / "docs/upstream-candidates/codexdd-045-reviewed-semantic-corrections.json"
OLD = """        if self.chat_widget.has_active_modal() && self.startup_protected_input_boundary {
            tui.discard_pending_input_before_interactive_screen()?;
            self.startup_pending_protected_request = false;
        }
        Ok(())
    }

    /// Recognizes queued requests before they become visible protected screens."""
NEW = """        if self.chat_widget.has_active_modal() && self.startup_protected_input_boundary {
            self.discard_startup_modal_input(tui)?;
        }
        Ok(())
    }

    /// Keep a visible modal closed to input if the terminal has not settled yet. The next input
    /// event retries the drain instead of acting on the modal; server events can continue meanwhile.
    pub(super) fn discard_startup_modal_input(&mut self, tui: &mut tui::Tui) -> Result<()> {
        match tui.discard_pending_input_before_interactive_screen() {
            Ok(()) => self.startup_pending_protected_request = false,
            Err(err) if err.kind() == std::io::ErrorKind::TimedOut => {
                self.startup_pending_protected_request = true;
                tracing::warn!(%err, "keeping startup modal input quarantined");
            }
            Err(err) => return Err(err.into()),
        }
        Ok(())
    }

    /// Recognizes queued requests before they become visible protected screens."""


def oid(raw: bytes) -> str:
    return subprocess.check_output(
        ["git", "hash-object", "--stdin"], cwd=ROOT, input=raw
    ).decode("ascii").strip()


def git(*args: str) -> str:
    return subprocess.check_output(
        ["git", *args], cwd=ROOT
    ).decode("ascii").strip()


class OriginalUpstreamStartupSafetyRestored(unittest.TestCase):
    def test_timeout_is_quarantined_and_no_other_tui_source_edits_were_made(self):
        data = json.loads(LEDGER.read_text(encoding="utf-8"))
        self.assertEqual(data["contract_version"], 1)
        self.assertEqual(len(data["paths"]), 1)
        item = data["paths"][0]
        self.assertEqual(item["path"], "codex-rs/tui/src/app/startup.rs")
        self.assertEqual(item["before_blob"], "3b111def463e2f52b4786b7a6db8c6e3aa7954f7")
        self.assertEqual(item["after_blob"], "f092c3cf69b8302609f975ee2931e8fd15aa6797")
        self.assertEqual(
            item["official_163_blob"], "4c329f378702414c3c17cc31802c778a8dbb2266"
        )
        source = SOURCE.read_text(encoding="utf-8")
        self.assertEqual(source.count(NEW), 1)
        self.assertEqual(source.count(OLD), 0)
        self.assertEqual(oid(source.encode("utf-8")), item["after_blob"])
        self.assertEqual(git("rev-parse", "HEAD:" + item["path"]), item["after_blob"])
        # Exact inverse reconstructs the whole previously verified formatter
        # blob. No adaptive model, Worker, daemon, authorization, or other
        # startup code was changed alongside the upstream safety correction.
        before = source.replace(NEW, OLD, 1)
        self.assertEqual(oid(before.encode("utf-8")), item["before_blob"])
        self.assertEqual(item["release_status"], "NOT_ACCEPTED")

    def test_startup_color_probe_and_pending_input_guards_still_consult_quarantine(self):
        source = SOURCE.read_text(encoding="utf-8")
        self.assertIn(
            "&& !self.startup_pending_protected_request", source
        )
        self.assertIn(
            "&& !app.startup_pending_protected_request", source
        )
        self.assertIn(
            "Err(err) if err.kind() == std::io::ErrorKind::TimedOut", source
        )
        self.assertIn("Ok(()) => self.startup_pending_protected_request = false", source)
        self.assertIn("Err(err) => return Err(err.into())", source)
        self.assertEqual(
            (ROOT / "codex-rs/codexdd-version.txt").read_text().strip(), "0.4.4"
        )


if __name__ == "__main__":
    unittest.main()
