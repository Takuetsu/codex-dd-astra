"""CodexDD 0.4.5 3C.9c: upstream-owned TUI source staging.

Offline static provenance checks. This does not imply 44 CodexDD TUI
conflicts are resolved, native Rust compiles, or release is approved.
"""
from __future__ import annotations

import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PRODUCTION = "b25550dcb2c6e84a84074b603086c9db6f32a63b"
STAGED = {
    "codex-rs/tui/src/activation_key_guard.rs": [
        "",
        "80de51f14b53af9dad2e2ab17b23c2a256502b7b"
    ],
    "codex-rs/tui/src/activation_key_guard_tests.rs": [
        "",
        "4dc347dbafb306e76f1b929bf2bdb1fc4b4c568d"
    ],
    "codex-rs/tui/src/analytics/chats.rs": [
        "b1ffadbb70883a64e3e6d0abd89d10b80bf60b13",
        "50d7c8167a4aee39b65b66319ad31466c14edbed"
    ],
    "codex-rs/tui/src/app/agent_center_tests.rs": [
        "409523e3582512217d3175829d832098dd271aec",
        "e8c3022b84407ec09856f9a9acaa9f2fcd438176"
    ],
    "codex-rs/tui/src/app/agent_picker.rs": [
        "4f515392d6c2234c143633591fbca3aaa9ef01fb",
        "085fc26657936b0f966470fda849164d64e1784e"
    ],
    "codex-rs/tui/src/app/agents_overview.rs": [
        "f72a9caaa9a9e7b070f12f8505d6e720be3b2e62",
        "ed8b8f0081c80d8a96e4e7c9684e80a2826f002b"
    ],
    "codex-rs/tui/src/app/agents_overview_actions_tests.rs": [
        "5098d0c28b81ee3e18c7e2dd612295a019a39301",
        "c741db36585313c83b4cea09690c2f1810cad1ba"
    ],
    "codex-rs/tui/src/app/agents_overview_details.rs": [
        "03b3c9b14c9a0c67a516b9d0eba350df89ebacf1",
        "1a86a59098f1d5c92cc2cea85592e4136af18d67"
    ],
    "codex-rs/tui/src/app/agents_overview_discovery.rs": [
        "d14c4c01ce9fe0b969800190e3f0eb53208361ec",
        "af02a2197f105836d1ec1ef8ddc189179c1811d4"
    ],
    "codex-rs/tui/src/app/agents_overview_errors.rs": [
        "3f94131b8c27d5d21b9a1f1ce3b6547a212c306e",
        "b37fa086b6f6a2c7f3e72ce44ff417881842ebea"
    ],
    "codex-rs/tui/src/app/agents_overview_new.rs": [
        "2d6465a5e4f64311024e2c54eb5099fcdb23b917",
        "0072408f4203e74729c6ec17ef6004c55d3c6d13"
    ],
    "codex-rs/tui/src/app/agents_overview_tests.rs": [
        "f5313a25c31ad3f06d32a2784efd5f902540d89a",
        "78c018f14db4ca16eac52c91593108c1d8a9e946"
    ],
    "codex-rs/tui/src/app/agents_overview_view.rs": [
        "094bd63c1135225555383fb1df37a74310546c9f",
        "4beec412374581c3a5b1eeb55a14a3ef73444971"
    ],
    "codex-rs/tui/src/app/app_server_events.rs": [
        "560f5d564b1ac4e32fc52d17a58969829e4dfdc8",
        "2821d6d242d25a3bd9825374388df758b9570e21"
    ],
    "codex-rs/tui/src/app/app_server_requests.rs": [
        "1c6a9b00f65710643a292366a154d0b853d0af6a",
        "be5c7760857da682fec5883b35d22845a766ccb0"
    ],
    "codex-rs/tui/src/app/clipboard.rs": [
        "0ecad1d8b2b6b39c921a51575b47d8f291072ae6",
        "a4359cd3e4b37cb005c136b6223bff9ebef05b49"
    ],
    "codex-rs/tui/src/app/config_panel.rs": [
        "",
        "f1fb01ff12deb14fd99f402812839721fd1ef88b"
    ],
    "codex-rs/tui/src/app/config_panel_tests.rs": [
        "",
        "e5f1c349a09c2a448ef24a78034d5339c7009358"
    ],
    "codex-rs/tui/src/app/daemon_menu.rs": [
        "d5a354a0e95f73fc32b30ab003c552c62ce6d93b",
        "ada9e88080288debba15945286061c6b14c56103"
    ],
    "codex-rs/tui/src/app/daemon_menu_tests.rs": [
        "0928bf31f05a0d0d3c99dc884153fc8817a3a39a",
        "0498495092a94fe97b683f199f01501e9bde21f5"
    ],
    "codex-rs/tui/src/app/daybreak.rs": [
        "89cc191e42653f5953c9a4a16382edaae0e0375b",
        "31a70f2a9dab830896b8c2239f51b7a86dd674d8"
    ],
    "codex-rs/tui/src/app/footer_selection.rs": [
        "",
        "436300bb22192ebb620302fc2537df80fac69cbc"
    ],
    "codex-rs/tui/src/app/history_ui.rs": [
        "dcacf8da017807b9d6a35a8c375f5879a4caa52b",
        "ca29929c1350cdad2a9ba465ffafc27dd59a96cb"
    ],
    "codex-rs/tui/src/app/input.rs": [
        "2a8938dd3df41ab4dca8db3411230af33c0cab4a",
        "f1c6dec7052b64b6ff15876cae29ab275df8bf4c"
    ],
    "codex-rs/tui/src/app/misalignment_policy.rs": [
        "6db5022463267f525f0a340180124da0fc58f554",
        "fe10a086d479c23a4b821ed93ddf073d3a2a7699"
    ],
    "codex-rs/tui/src/app/owned_transcript.rs": [
        "3fcd0cd23cb8903df0c399f93fee5cbbb616f938",
        "8c42d78f1cd5c4f99e7e30586668066c543774c5"
    ],
    "codex-rs/tui/src/app/owned_transcript_input_tests.rs": [
        "4c4119b344111b7600600872ad886117d3413101",
        "15cf0bf8c914a916bcc34fcdad8d42e3d684fea0"
    ],
    "codex-rs/tui/src/app/pets.rs": [
        "a21fe1a7f14b3d315e6710bfba3d1eddd4355563",
        "4253fe95634e979c375e57a2208529502b13c207"
    ],
    "codex-rs/tui/src/app/recap_tests.rs": [
        "437cb66bb848129fd8babe92e6c094fa2bf3aed9",
        "8990301727de756dfec82965db89383c37a1c02b"
    ],
    "codex-rs/tui/src/app/reconnect.rs": [
        "b7d6e2637e550c5fb4cd014761732681b840ed29",
        "97e8d65d11f1722955e17ddf92080a9b00191fef"
    ],
    "codex-rs/tui/src/app/resume_config.rs": [
        "30add17ec1ae4088bc97359e966c2786cd10be1d",
        "1cfe074c075a833d6b558eda68284aac60baa3df"
    ],
    "codex-rs/tui/src/app/retained_reply_tests.rs": [
        "",
        "2a95b2a7190cc7314e0ee078b14839d3586446aa"
    ],
    "codex-rs/tui/src/app/right_click_paste.rs": [
        "53361e86840999a32aed7fbea301d9a34294a594",
        "632e134a4356d7cd63fed3898d4fe48e1cfe80af"
    ],
    "codex-rs/tui/src/app/right_click_paste_tests.rs": [
        "21cb0477cb97ef9525d7d144ca27676b3e84d9f5",
        "b24cad29acfa89d3386ca36fe436f0f3e92bde7e"
    ],
    "codex-rs/tui/src/app/safety_buffering.rs": [
        "e03ea7689f7a0488a885b2cfb1187f2b0f0a1960",
        "9c14dde984d5d4cb34b3883e65b56c793571780d"
    ],
    "codex-rs/tui/src/app/side.rs": [
        "150583dc29187c2b3908bd05cd40f8495e04ed71",
        "8ac0adfeab6708624d89b5dd55d5cdfa8703fc7c"
    ],
    "codex-rs/tui/src/app/tests/agents_navigation_tests.rs": [
        "e9424e8c9bbf431af517de7a9970191c0eb742e0",
        "2dbee5f9c813fecc4245d6dd199572a4ec85847c"
    ],
    "codex-rs/tui/src/app/tests/background_task_defaults_tests.rs": [
        "16862db0fa3b83188de5725776c0c5ba28851cfe",
        "438ce7605ee845c241a7e2456b0fbc6819e37dbf"
    ],
    "codex-rs/tui/src/app/tests/fork_workspace_roots_tests.rs": [
        "b6e4a48dbb452b2c51aa37ff78039b31a3c137c3",
        "762c26ac891a16cfc4a37aff7160ca280ac6416d"
    ],
    "codex-rs/tui/src/app/tests/key_chords.rs": [
        "35d8d52cf6837ae0c3cbec4ba55370404f152adf",
        "6c1321ad5262def0af21bc1377885def1cb2a397"
    ],
    "codex-rs/tui/src/app/tests/misalignment_policy_tests.rs": [
        "c3b8283fc08ee2f0efdb8fe42615fb63f849ab4d",
        "b54c38edf4f922963763501c0fd15381e0b681f7"
    ],
    "codex-rs/tui/src/app/tests/new_session_tests.rs": [
        "c816f2dc9425b8039f9a30811de4caecf6336d65",
        "9f03f668a2e7a68e509972dcaf7034af320770a8"
    ],
    "codex-rs/tui/src/app/tests/realtime_start.rs": [
        "6c75e0d57208e74c745de4182dd7a19ef9582ec9",
        "295067c3b7a895659c5c265d99287aa90d81fc01"
    ],
    "codex-rs/tui/src/app/tests/user_verification_routes_tests.rs": [
        "c288b03a3effa108abac1aa82c155821124a1110",
        "d3d07eded45a11695491dd57a73815de5dc1407e"
    ],
    "codex-rs/tui/src/app/turn_tips_tests.rs": [
        "070cfe534ae2a0f2091bf3614f7395c4ee4794e7",
        "8d79bdf06a4ab960a0ab42f7a832ecea01e9d0ba"
    ],
    "codex-rs/tui/src/app/user_verification.rs": [
        "5b34f22e1c896881df511ae367dbaba482bb6e22",
        "23cd49af46663e9cda4cc67e230ee4aee91d4e5a"
    ],
    "codex-rs/tui/src/app/user_verification_errors.rs": [
        "19a1aee82785f1cc77b06275b1bde9bc5afe8f89",
        "23e6072d12652aec6bbd0148bd9679652e8006a2"
    ],
    "codex-rs/tui/src/app/user_verification_errors_tests.rs": [
        "89393f1cbaff6960417fb4d75bfdc7ed2ddfb682",
        "9e7bb5a9663fad39e1af543864d17ae53e59f0e5"
    ],
    "codex-rs/tui/src/app/user_verification_requests.rs": [
        "daf6ab00245e4e40bb5a3c976f1db6b9a3f67138",
        "c9c81c54f1a47888710038d50bf3892346dda371"
    ],
    "codex-rs/tui/src/app/user_verification_requests_tests.rs": [
        "2a4cd410db09b1aef3f5f1771bccab1cc8a9c494",
        "654b5b5272e2355e1d9af76cd6117c63da3e951a"
    ],
    "codex-rs/tui/src/app/user_verification_tests.rs": [
        "64d1e97cedf3af0b28265dc5e83d349c99f36f26",
        "ed1cd6d0a02f18ab450e44de8cc726601dbb6846"
    ],
    "codex-rs/tui/src/app/voice_owner.rs": [
        "5200cf6a6e7029e6da64889d1ab82ad370a3ac26",
        "67ca6107ec598ab90e0aaf5b8f59c6d18e0ff7ea"
    ],
    "codex-rs/tui/src/app_backtrack.rs": [
        "f55b75c1bb4e48afafac1ffbad89eb3b57a980f6",
        "f962f3ba8e4256de7ec2962b48d0f4c58fc7fb5c"
    ],
    "codex-rs/tui/src/app_command.rs": [
        "990361d1ec426679c6f4f2273f3f252fca1ab4ba",
        "f1f3cb2e149c05cf64058f4a1b642c39ed9d95d1"
    ],
    "codex-rs/tui/src/app_event_sender.rs": [
        "7d064523c99cd1289da9d2f4b8bee0f86f5dff97",
        "dac6b8cbe86ab836c5458bf7c1de36c84ac60aa4"
    ],
    "codex-rs/tui/src/app_server_session/daybreak_api_key_compat.rs": [
        "",
        "0f44c5692440fca4686a6955b94428a4e0cfc21c"
    ],
    "codex-rs/tui/src/app_server_session/thread_list_tests.rs": [
        "7ce2c5509a69c43a83bc5d41737aaad483677551",
        "fe1ae9ee02f93b4d0646a51e10f4d2bf31405d12"
    ],
    "codex-rs/tui/src/bottom_pane/async_questions/mod.rs": [
        "7753a3a6778304f1a29898d469c1fe25710b2b4f",
        "de4232ef8f76b07550819e17f59afd69c76a569e"
    ],
    "codex-rs/tui/src/bottom_pane/async_questions/state.rs": [
        "96c4c2536b8020a8f8f94693caa4d19bc8216468",
        "d48fd99da7587093632e8948fefe3165d0548a8b"
    ],
    "codex-rs/tui/src/bottom_pane/async_questions/state_tests.rs": [
        "1345dd543fb70a2d4546752e11eaddc6279f539e",
        "713ac5194a661a5960a52402d0de63a950198037"
    ],
    "codex-rs/tui/src/bottom_pane/bottom_pane_view.rs": [
        "951e80706b9dd0863bc661861447d7fd5da64ea2",
        "6e3726352afb15a40c3de0de1558bc2b59def9cc"
    ],
    "codex-rs/tui/src/bottom_pane/chat_composer.rs": [
        "c919e31c92ed420704de33ba627127cbada50711",
        "2ad3f1cd95ea3b5cb55326c2f56be31b76eeb761"
    ],
    "codex-rs/tui/src/bottom_pane/chat_composer/agents_navigation.rs": [
        "6375a01f329498d9e177ae54d60562c4301b33a9",
        "40d474812313ce76470f7875ce175ea3fb1f4730"
    ],
    "codex-rs/tui/src/bottom_pane/chat_composer/blockquote_paste_tests.rs": [
        "fcd393739ab002068fa52097adf683b121e5691b",
        "55d67c23b01380bfe3de9d54f32db5d89e1976e4"
    ],
    "codex-rs/tui/src/bottom_pane/chat_composer/footer_state.rs": [
        "5b8784a4faecde5d9ee31e327c5376620bd7b1fb",
        "bcc92a3ed5762c077dd15fc3ef5aafab655975b5"
    ],
    "codex-rs/tui/src/bottom_pane/chat_composer/history_search.rs": [
        "b5bd3e696ec93275ee6d45381b1f660e49339f9f",
        "312798f84280e80f7ae110eae0ef774b7f1650db"
    ],
    "codex-rs/tui/src/bottom_pane/chat_composer/history_search_paste_tests.rs": [
        "9039124265be7d636fda18fb77ba4811d17354bc",
        "2c9392fb911e5e039de0fe4e6a700f47c0f08249"
    ],
    "codex-rs/tui/src/bottom_pane/chat_composer/mouse.rs": [
        "ec95356ef5e303ab3693816707bdf8c3343c36f7",
        "65a5926135ecd4d211f0edf54f834e4878f52a08"
    ],
    "codex-rs/tui/src/bottom_pane/chat_composer/mouse_tests.rs": [
        "ad266284f5bcef90ecf7e667a9c786309a7e78fa",
        "920395ab1724fc35c83b22a422ac5658486c1d08"
    ],
    "codex-rs/tui/src/bottom_pane/chat_composer/paste_input.rs": [
        "b3c77397765e9295ce9d9652c7b771e126510353",
        "78db041eb2a6b1781574b5babe4995145a61d74c"
    ],
    "codex-rs/tui/src/bottom_pane/chat_composer/reconnect.rs": [
        "ac64a4ea6a651707bc859d086f0ae3a1bd70b36c",
        "0cefc44b11517f8acb2eeface2ae170fa281e737"
    ],
    "codex-rs/tui/src/bottom_pane/chat_composer/reconnect_tests.rs": [
        "d1d7cd7bced7aae3b071e983860f1a3f52d76e6f",
        "66dc657c71d3b60efe85745dca250ea543bccbc6"
    ],
    "codex-rs/tui/src/bottom_pane/chat_composer/snapshot_tests.rs": [
        "2a22f909a2b96bee771118342898a5d24ad43842",
        "f15a360a2afc1ab8796cab2a83ec03b14f074f2e"
    ],
    "codex-rs/tui/src/bottom_pane/config_panel.rs": [
        "",
        "02134151f198d6e13a5d3b1033a9990424c56b95"
    ],
    "codex-rs/tui/src/bottom_pane/list_selection_view.rs": [
        "64f660f8acac035107ef505d7cb8dc3b36fd4c81",
        "19f365028c605762e30481d9e5c833f30c0b8000"
    ],
    "codex-rs/tui/src/bottom_pane/mentions_v2/search_catalog.rs": [
        "113f59ac0bc505d865e42e5ed060702d60e39465",
        "3b9fa6686d8e7f16a3ea717f0928b9406f26eb87"
    ],
    "codex-rs/tui/src/bottom_pane/mod.rs": [
        "c0e390289e8eda4c8ffd27eeaac705507107e158",
        "ae9405e9ff3718aa8d3c6c3dfac487a3c2688823"
    ],
    "codex-rs/tui/src/bottom_pane/paste_burst.rs": [
        "8084f6b9febd36a0b02156bcb3576b2602655a8c",
        "c07712b272f4a9a8b69eb637e2c7eaa697194956"
    ],
    "codex-rs/tui/src/bottom_pane/request_user_input/mod.rs": [
        "66c4a1c9ddad7ff00ef1b93737df7f35316628cf",
        "51dab27b649f58c9ada41b2c9517a227875290d8"
    ],
    "codex-rs/tui/src/bottom_pane/selection_popup_common.rs": [
        "fa2c9f21964fbf6893095daceeb91cfef88d395a",
        "0059d561249030bda99b3c90cee1d5d99d391d94"
    ],
    "codex-rs/tui/src/bottom_pane/selection_row_layout.rs": [
        "50678b8e39594e73355877e17a73c055fd9bdfd0",
        "573f015b65ee778b31ad2d5667c2c7b69a562589"
    ],
    "codex-rs/tui/src/bottom_pane/shortcut_overlay.rs": [
        "b77657323eb0f2d519b727e841b68f1a3a9bdfb0",
        "f90edc70087c9acb1a82b6823ccc8e7f316fa063"
    ],
    "codex-rs/tui/src/bottom_pane/status_line_setup.rs": [
        "3d20f5cb93fdfed6daf7e59862abd01e280b85a8",
        "9567424fbb9b79eba31388a1e1449c500d36fb59"
    ],
    "codex-rs/tui/src/bottom_pane/status_line_style.rs": [
        "2fd4b2c85fa0180774933bdd9ff80b2dc8ba32d8",
        "6ef9003c5887ed3cf8f614d3079f95fa96e61ae8"
    ],
    "codex-rs/tui/src/bottom_pane/textarea/editing.rs": [
        "9cb7f826023d581cf3899ace6153bcb03c8d9b57",
        "6418c3f28927a8ec2a69070b7cb4297a852191b7"
    ],
    "codex-rs/tui/src/bottom_pane/textarea/vim_commands.rs": [
        "143054f49524cb402c66588a59a4936bb4562d41",
        "9e798f84ac1a7f40d81ed0748503eb1ead5bd668"
    ],
    "codex-rs/tui/src/bottom_pane/title_setup.rs": [
        "ebfc53ca971f00da11ab852345614ad27f169cf4",
        "145938baf4c87d68553075c227bb96dd96912a7a"
    ],
    "codex-rs/tui/src/bottom_pane/view_stack.rs": [
        "0f90ee53bb4b48a986c50f950c5e78d47de72a8d",
        "fb1d91a126173a69311270e3fa739c24303b53ec"
    ],
    "codex-rs/tui/src/bottom_pane/voice_strip_tests.rs": [
        "0eeaca7fb4cd5e6315707a8adf546070ef3452ca",
        "208544c86fc26bb2ad839c4e755597a28eb45caa"
    ],
    "codex-rs/tui/src/bottom_pane/warnings.rs": [
        "522856b48e9ecdfe3aabb3d6bcd9d6bc61c2c9ca",
        "31b371af18ca4390b6325e509ec5d9913641e56c"
    ],
    "codex-rs/tui/src/bottom_pane/warnings_view.rs": [
        "3dfe27e4066548bce6d43fb30e4a13f63b555616",
        "3d3bb8966d68694b20f3186469403b1d59ce7a55"
    ],
    "codex-rs/tui/src/bottom_pane/warnings_view_render.rs": [
        "98557c86a5dac7050725cd47be4c59150e2462a6",
        "171f88666c67603d37e9424c2b20463fedcc465a"
    ],
    "codex-rs/tui/src/chatwidget/clipboard.rs": [
        "ec512cdb44b3cbaac750feec48a4a1e45920fceb",
        "3d8bfa69d7fa82a7583e3f0b5d8f067d7c679905"
    ],
    "codex-rs/tui/src/chatwidget/config_panel.rs": [
        "",
        "ea8cd8c7a5e1e33a097669343995e2735558d118"
    ],
    "codex-rs/tui/src/chatwidget/keymap_picker.rs": [
        "1db2956369ebb13db1dcfa4990e13027775d8ad2",
        "80c62f7e8fcceec0cbc13bd29aa41e2c220a2d60"
    ],
    "codex-rs/tui/src/chatwidget/model_popup_state.rs": [
        "bee3cdba8ad40a13eb5be897fe518dea679bb96f",
        "15f99f85bbe4d903ebed8560ce733a0ecadb41d4"
    ],
    "codex-rs/tui/src/chatwidget/notifications.rs": [
        "2668e962a4bdf6ee082a4cf2fa84e3537cfd1dc5",
        "92ef60bada2767a6678d19f8b2f6588889c10d55"
    ],
    "codex-rs/tui/src/chatwidget/pets.rs": [
        "dc9bfa6121d6a4cc2d2e24de0d9df75bfe611cf1",
        "90be6317da80afcf6bba036e3fde62bcb31bb11a"
    ],
    "codex-rs/tui/src/chatwidget/pets_tests.rs": [
        "796339acfc6d091c0876ef35cfd9f04f10935a4e",
        "61ac065692a3835dbed0ccbb7b095ed1abd2d92b"
    ],
    "codex-rs/tui/src/chatwidget/plugin_catalog.rs": [
        "017c4afcd8272617d8d30a7dc4ec3d492882a1b5",
        "79cec717e72fef6e52b8ffed086019b88fccdab5"
    ],
    "codex-rs/tui/src/chatwidget/realtime.rs": [
        "6215611a0829d130f1c909c7f84e54998b5103a9",
        "bb2a48e527c6b03d5ed5a521b26d6012f8585791"
    ],
    "codex-rs/tui/src/chatwidget/realtime/failure_metrics.rs": [
        "",
        "e2c1227004473dd4531f83afffa75298eeea3a52"
    ],
    "codex-rs/tui/src/chatwidget/realtime/recording_controls.rs": [
        "7e84dee420097a67dcb57a3feb6d6299bb222d8b",
        "e299500840cff2ae9413f24b42a5ac616f1d692d"
    ],
    "codex-rs/tui/src/chatwidget/realtime/recording_controls_tests.rs": [
        "cbe21174aebbaca205a75d9e850d90c3ed7a1c02",
        "f0efff94d216efbd3fd902a773420d2e88206ffe"
    ],
    "codex-rs/tui/src/chatwidget/realtime/session_metrics_tests.rs": [
        "07c60bb34cecd98ccd3db1d1965ede9736d319c6",
        "1604b410cee47d5ae9baa6e2439c2c394e12b877"
    ],
    "codex-rs/tui/src/chatwidget/realtime_tests.rs": [
        "78c53630dfe3d9ed257f5a8aa062141af698a07e",
        "0ada7c233c7ed616d7a963ef6448380b9c8041d1"
    ],
    "codex-rs/tui/src/chatwidget/realtime_tests/lifecycle.rs": [
        "bf7d71e29352e8f7cfab17402cca18ef5ae7f4a7",
        "bbeb21683cca8c0f358b329003d72cd8f8817af9"
    ],
    "codex-rs/tui/src/chatwidget/reconnect.rs": [
        "4cde2de18f1f67ed53967687c7cb27e83da7b928",
        "d50d9d5a431f19d282a87abac991c7d24b1207fb"
    ],
    "codex-rs/tui/src/chatwidget/rendering.rs": [
        "adfe1cee5c4c1199df4eb9454a3f1ec9306ee9ae",
        "eee242df2ed8321534a9b4a65bc60e7f374198de"
    ],
    "codex-rs/tui/src/chatwidget/status_surfaces.rs": [
        "46727cfc3210134991a4a8919c481b977a280554",
        "a3946abd13fb0cc06f1fddd80b42e815fe35e8aa"
    ],
    "codex-rs/tui/src/chatwidget/tests/bedrock_catalog_tests.rs": [
        "1d34fe6b1a3979c036233abb385212585a8f7c43",
        "e45c306eff5bf714cc2ce84c829b7381785bd020"
    ],
    "codex-rs/tui/src/chatwidget/tests/helpers.rs": [
        "1db139d7311ce97dd2b91f85ee33c63d6e90583c",
        "9ce2635d3b32a8480cbff8eb308fcd1a7c437d34"
    ],
    "codex-rs/tui/src/chatwidget/tests/popups_and_settings.rs": [
        "4018a2539627427fb43f2dd0925168b3db1f5de2",
        "d64fecc640865fe6565a2113cc4c2515a82835e2"
    ],
    "codex-rs/tui/src/chatwidget/tests/status_command_tests.rs": [
        "75685a2581ff4fdc8d72c94dc1b0b0df85329497",
        "fb7a21c25c25bdb13943c7335a3d50a1b69d1035"
    ],
    "codex-rs/tui/src/chatwidget/tests/status_surface_previews.rs": [
        "8c9358de9435e5264f3349ad7e6aa2cbe8d38cb2",
        "085d51619ea4d62ec7f2e17ffa7a3e8316cbb200"
    ],
    "codex-rs/tui/src/chatwidget/tests/terminal_program_status.rs": [
        "",
        "3911129ba36e91536e815ead60e8a01e59f75bfc"
    ],
    "codex-rs/tui/src/chatwidget/warnings.rs": [
        "580082e3992e7286d2c7d3ca67daf25b44ef4196",
        "31c442a47fb29dc0e526165f57b0fa15609a7804"
    ],
    "codex-rs/tui/src/clipboard_copy.rs": [
        "cedbd5d54580974ff2ea384fa79d24bb32c6e683",
        "4d229364bc85e78700c3ddfb628de7bc95a02e74"
    ],
    "codex-rs/tui/src/clipboard_copy/routing_tests.rs": [
        "3638e3494ead422f18f3d94c8341f4f1e2ce0c79",
        "a697a619a56c6d588d58b1f75e1f0a9457cafc77"
    ],
    "codex-rs/tui/src/clipboard_copy/worker.rs": [
        "b86047441b2742230c27d727664a2ed2f9d0ed9e",
        "5cdeed4a20af276c7961416c35b666379b44d75b"
    ],
    "codex-rs/tui/src/clipboard_copy/worker_tests.rs": [
        "1805813f53061fd0be98f2d559a34eeb861ab85d",
        "ed48e126d0b94f8c2ea8fad909ffe64b1470e6db"
    ],
    "codex-rs/tui/src/clipboard_copy/wsl.rs": [
        "",
        "d1c89cc9e30986b30e52117c4f9c033e93d48a18"
    ],
    "codex-rs/tui/src/clipboard_copy/wsl_tests.rs": [
        "",
        "895fcc7616bed948643a69b08d8c959de6e35e5a"
    ],
    "codex-rs/tui/src/clipboard_paste.rs": [
        "cc5489d6e0789f6453bddd1da5419120cead80db",
        "ff291801ea12ede0d7bec749f644fd3d52077961"
    ],
    "codex-rs/tui/src/clipboard_paste/text.rs": [
        "32fcb1b85539efd566092cab846cbd83c5f7e27b",
        "17eee61566d4b5947ea602bcc35d8d47db9f4701"
    ],
    "codex-rs/tui/src/clipboard_paste/text_tests.rs": [
        "387cc39472bf79535c3fe309c446cbf8988addcd",
        "23006894d0b0f079650f3f729a307c3c4e95de0b"
    ],
    "codex-rs/tui/src/config_panel/config_panel_tests.rs": [
        "",
        "32e1c67c9f1224d03604ffeaa36ed321735e5d50"
    ],
    "codex-rs/tui/src/config_panel/input.rs": [
        "",
        "a3202b8b1b0eed4656f5d4a7f210630cf52381e0"
    ],
    "codex-rs/tui/src/config_panel/mod.rs": [
        "",
        "ebc5799be2b87909ea321c0b9c0718a362b6a7bc"
    ],
    "codex-rs/tui/src/config_panel/render.rs": [
        "",
        "28b037ee5fdd949d95379d54adad91b4e8a1be6e"
    ],
    "codex-rs/tui/src/config_panel/toggle.rs": [
        "",
        "71dd1fa4a8002e8923295382113a3e2c9566f71f"
    ],
    "codex-rs/tui/src/custom_terminal.rs": [
        "60081516bdf8c17a5dbe15e7c74bf1ec8e77da5a",
        "c2a6f058951aadea24b6545007c31f701ee83a68"
    ],
    "codex-rs/tui/src/daemon_recovery.rs": [
        "2c9959201eb204d160cb1bb9055a20019192b294",
        "c04823d6a67e70bd4872deb6a23f72d5850daddf"
    ],
    "codex-rs/tui/src/daybreak.rs": [
        "687a1e8c66102f5bb5e7973082dbf1d3c6e4138a",
        "a47a837b33ea19bbaa66e8762884135651e4eea5"
    ],
    "codex-rs/tui/src/daybreak_tests.rs": [
        "ee86b68d8c8262f25b68573c79b05d9e06cd1828",
        "1d1b7bf45854d7bc9c9521fb219a71180fd6ec97"
    ],
    "codex-rs/tui/src/debug_config.rs": [
        "2840e99609fd54fc43d1509d11baad2f99c79dbf",
        "f1fc03a58ff3294e9376d773605b3209f0d47519"
    ],
    "codex-rs/tui/src/dynamic_tools.rs": [
        "662ceac09e1fcfb67265328dd306becd51779bdf",
        "dd1897bf177814089663a0cb6dd0592c8424e894"
    ],
    "codex-rs/tui/src/dynamic_tools_mcp.rs": [
        "67fed87b4ccc42babb2ce1c1fecb487347f4a148",
        "71914649a4e86724747d04e1292c1f929b2c161d"
    ],
    "codex-rs/tui/src/experimental_features.rs": [
        "b3c360a036ba9092c1e1f4c2914a089d4d8ef6c6",
        "270a24c4f8fc823907f42ffda1c03e0cfbe27b4a"
    ],
    "codex-rs/tui/src/footer_selection_tests.rs": [
        "",
        "18937c06796275dffaf1ac2a9bce20bcfce60d72"
    ],
    "codex-rs/tui/src/insert_history.rs": [
        "d07d6cbe3a6f04478808f1820e0407adae75a401",
        "afa649f70a5a187aec312c135139df5913eb7b6a"
    ],
    "codex-rs/tui/src/iterm_session_status_tests.rs": [
        "",
        "12f35136f61fe690da7c02f4eba354fdc2d74585"
    ],
    "codex-rs/tui/src/key_hint.rs": [
        "4460b50a8ba7f2908bc5e18cab01b63df8212af3",
        "001d40104657f522ca1896ce708f3757aca86642"
    ],
    "codex-rs/tui/src/keymap.rs": [
        "047473ad51517875d58d4f077fcc6a566870ef6a",
        "841b3fef7051c4bbbdcdb257c26bd4f7cd9a9032"
    ],
    "codex-rs/tui/src/keymap/bindings.rs": [
        "3e044f2437a048d2d1e208faa4add8c25e9b182a",
        "08c7b0930ba8a4268de650d2a9f71a3433b40fed"
    ],
    "codex-rs/tui/src/keymap/chords.rs": [
        "0ad0f5e524111d916388399a9313ba35becf0757",
        "fc6855fba5f98ff5fcbced571bbef742de1d970f"
    ],
    "codex-rs/tui/src/keymap/leader.rs": [
        "",
        "6bbe0de5cf82e6175259bfc0e0e8271740c109e3"
    ],
    "codex-rs/tui/src/keymap/voice_tests.rs": [
        "4bbea95eb7f058c0cd8e3b98a58635999e228c58",
        "6e5bc12a3b5e43f4505df9534a86050af401e546"
    ],
    "codex-rs/tui/src/keymap_setup.rs": [
        "98c8dd7b3f2ca66f4c0b4b4997ed20d7f447b81e",
        "e9bba1e9dd798b20c6c8094b54fb36f64a599bc1"
    ],
    "codex-rs/tui/src/keymap_setup/actions.rs": [
        "46ec0ac847a7d2d3c413ff26fa8b8b4b3e460f3e",
        "3f9efbd3575dfab492c9192257ff2a7bfe5f21d3"
    ],
    "codex-rs/tui/src/keymap_setup/capture.rs": [
        "a3409287dd0e7bed6fdbb5f4aa18732351fd4742",
        "9f9af5d5d2625de4b8e61b7e5c5c6c6c54ff968a"
    ],
    "codex-rs/tui/src/keymap_setup/debug.rs": [
        "0bcd64d6a256ae710530db4709ce60f99a94b32e",
        "0415086a566da94ac1f7d196a37ddf6da6a43a34"
    ],
    "codex-rs/tui/src/keymap_setup/picker.rs": [
        "e962b3cace9451e27bfb59350245dc484d331ad6",
        "975a06c1b2699e7a21b90d17e6665c6d0892f1e6"
    ],
    "codex-rs/tui/src/local_settings.rs": [
        "9529ba092fabdf38a793149dcb0d28d02ca539c2",
        "a9bed997964720888482ca91205246f0ece50a93"
    ],
    "codex-rs/tui/src/local_settings_tests.rs": [
        "17c6a5310c8694af49659067b170fecdff19e72d",
        "39c238e1f22ea75bb2c7652dc0794fc33c78ad36"
    ],
    "codex-rs/tui/src/main.rs": [
        "5a196d614a92b240758962ce0f07dfa60d8497be",
        "789c7f15f208e183d8863cb11b72bd36795230f5"
    ],
    "codex-rs/tui/src/managed_worktree_model_tests.rs": [
        "ae36b611a5b8bddd60ba53041f10e2cc25a75cf6",
        "2f66b7cf689ef656c76303a6c2cbee37a1bc98e7"
    ],
    "codex-rs/tui/src/managed_worktree_tools_tests.rs": [
        "e51bff0586a347a0f62111116f8efe1da4fad083",
        "27904dc9f63d61d4c8ade24e62c944aca1a76477"
    ],
    "codex-rs/tui/src/markdown_render/local_links.rs": [
        "4fee5da46aae6d01acdcb266d5951921f47c1089",
        "70c30b763d11c7d59606f0cd70b7e71eddf1854e"
    ],
    "codex-rs/tui/src/markdown_render/local_links_tests.rs": [
        "d54d982f27970e2a98f9052b7a35365aa11b6d6b",
        "c6ed70bfc9d8dbe4ffeca2e8fc35196571e32272"
    ],
    "codex-rs/tui/src/named_session_lookup.rs": [
        "7fc102cd6f4d06f75e61d2f796e51e98713dca24",
        "0f1a1044e913b771a3460650621b470567aad757"
    ],
    "codex-rs/tui/src/onboarding/bedrock.rs": [
        "be6fe7151d890c38c913a0aaca12802bd08bbee5",
        "d1ea4d24640c3d681efe1f2737bc525a85ab2616"
    ],
    "codex-rs/tui/src/onboarding/bedrock_tests.rs": [
        "9d18a8259c8644ad02ff4ffeea3f923b05a67246",
        "8f9f240587ac2e3a840308041cf35d81aa94393e"
    ],
    "codex-rs/tui/src/onboarding/mod.rs": [
        "6a9ea961244412ee67a72a62e636317a639ff114",
        "b1301991f1756c7e8435d1580729a95d7e817029"
    ],
    "codex-rs/tui/src/onboarding/onboarding_screen.rs": [
        "0ec1006eca414c9b37b3a98f1bea029a655b0fec",
        "626e54563741ce04c09198d22f1ee80a38e20485"
    ],
    "codex-rs/tui/src/pager_overlay.rs": [
        "685fff820e520b8fa48c99f2a59fee4086dc1393",
        "b20061fc63a81939a56cb3bd5b376ab270fc8521"
    ],
    "codex-rs/tui/src/pager_overlay/transcript.rs": [
        "508d5c34f4e97893ced33b91415953a42ef82724",
        "4aaae519b65e1a9f1c3429d3ad2e12bc910efc19"
    ],
    "codex-rs/tui/src/pets/ambient.rs": [
        "3427f0a4c02a8f99ec245da661f7b81d00641fce",
        "e5507ce5c9e8779baa47136d895a0bb1dd372fd7"
    ],
    "codex-rs/tui/src/rendered_selection.rs": [
        "",
        "4b076a23784cc8c1e27481e0241bcb57536af5f8"
    ],
    "codex-rs/tui/src/resume_picker.rs": [
        "40b6d7440262da6c22262b84bbcc6a498b3f4a7a",
        "7424757fe33fa4cc4759d492bc1e00ca9719603b"
    ],
    "codex-rs/tui/src/resume_picker/archive.rs": [
        "875e74b1926539a6d60eb6dbf0c41ae4edb95990",
        "a3ccf5431277d68b37a7b117f7ec621b5d7b6711"
    ],
    "codex-rs/tui/src/session_archive_commands.rs": [
        "ea37b0fe209c4d1585510ca2ce01ae2724961ae3",
        "47e4608d7d6676bce1947fe43601980091ed7b63"
    ],
    "codex-rs/tui/src/startup_draft_input.rs": [
        "491b99c7fc2e47baa606bb028bd27ea9d584a3a2",
        "b37e70957ecf6471e70cc1566f9ea44f2662ac75"
    ],
    "codex-rs/tui/src/startup_hooks_review.rs": [
        "7b39a4de46155df9f3fd2898b8df2d6cbb8b1d7f",
        "6ad297e20c7cac0e381de1e1934ec1bc7e61e748"
    ],
    "codex-rs/tui/src/startup_recovery_tests.rs": [
        "a0afc87d0e39545a42911115c09be084d83bc3a2",
        "57cedd8bde3dd2df767eb998d86f8bf49bae21a9"
    ],
    "codex-rs/tui/src/status/remote_connection.rs": [
        "6ed6fdd411283035fc63dd17dd30a617737c430f",
        "2ee0395438048edd9f2db15f0cb90161a93386f3"
    ],
    "codex-rs/tui/src/status_indicator_widget.rs": [
        "b3e5edec9801bcb5689a1a3ddc82e57c8520cb5a",
        "3cb54b2cf73b665e2606e99b827e06973e695ac1"
    ],
    "codex-rs/tui/src/task_mentions.rs": [
        "5c1f7900a974e6e45f55a34d930fb546d804a60b",
        "364e33abcd1e41bac2098cb12ddfe869a30b929f"
    ],
    "codex-rs/tui/src/terminal_hyperlinks.rs": [
        "8457b6e4470ef59fbd8d7b86d06faf2e2ff0316a",
        "83d949b3b5253e3ebf0665d99aa6b0416876fc34"
    ],
    "codex-rs/tui/src/terminal_hyperlinks_tests.rs": [
        "ead3f3662b79819e954df0e1cb3a2a16ac8ba226",
        "dc3c66b423ee170bd9cc30f207975ae40bc88e91"
    ],
    "codex-rs/tui/src/terminal_palette.rs": [
        "7ff9b968f936091faddc9e398f5f8298289dd2b2",
        "8364c6b7d75ecf6515c167c3ad5c795f050c1310"
    ],
    "codex-rs/tui/src/terminal_program_status.rs": [
        "",
        "9379f960a6f551da0f5f6ea42f1f69ed28e08ab6"
    ],
    "codex-rs/tui/src/terminal_title.rs": [
        "ff2195ee5086080910ddf90f77da2980e80d0b1f",
        "37a47b2e1e014aa9f5d21e3d339a85b804204116"
    ],
    "codex-rs/tui/src/test_support.rs": [
        "44e989937970d941edaeae3f235b924c55c1ff71",
        "a83764e1bb192cf115eb64b6f97d9c6502205369"
    ],
    "codex-rs/tui/src/theme_picker.rs": [
        "562997efee8e166541a20d344377d5cfe4b7f135",
        "50dec0b9f00babf91697f287946f45475324c743"
    ],
    "codex-rs/tui/src/transcript_view.rs": [
        "31ebfec5eec00f1c792b88bc0582a64eb0c326b8",
        "4a2b803537a83298d8d6a83cf225f7624529707a"
    ],
    "codex-rs/tui/src/transcript_view/copy_on_select_tests.rs": [
        "6bfd888e9580d15ea435423bdc017a3e5195be62",
        "69ff193c3b90cdbf418a775c275d5c665da0ffa6"
    ],
    "codex-rs/tui/src/transcript_view/input.rs": [
        "ced88a72b0e12b63648d964a479e61d5b100c258",
        "f5bf11940250eed9707e008248ebebba0c869110"
    ],
    "codex-rs/tui/src/transcript_view/prompt_header.rs": [
        "b2d6953b97eb6595ebb829fcc392ba4fdd1fea44",
        "b427277761a84f4e2b0878d770fdb71c4f77d1c7"
    ],
    "codex-rs/tui/src/transcript_view/prompt_header_tests.rs": [
        "3915436844621163d7d67f52fc945248d95d21f9",
        "66be2e2435151c3f9167ba0d1890c27361b01f51"
    ],
    "codex-rs/tui/src/transcript_view/select_mode.rs": [
        "",
        "562a812b3b2598e128f2a21c4c75b4c894837d6f"
    ],
    "codex-rs/tui/src/transcript_view/select_mode/motion.rs": [
        "",
        "d9198b93483afd7268692ba055500fa94bf70811"
    ],
    "codex-rs/tui/src/transcript_view/select_mode_tests.rs": [
        "",
        "61eed90e33b6bdad22b7c67ca2b90bf2c348f8c4"
    ],
    "codex-rs/tui/src/transcript_view/selection.rs": [
        "8e62595d3a42b320e8e5cc934709f7b55dcb7f52",
        "b6cae028953ffa5c2ae88da8d702bb38e9940043"
    ],
    "codex-rs/tui/src/transcript_view/text.rs": [
        "282925dfd40c54f82d6f664e7456a47ccfd75940",
        "4aac01ddda978ef6142b8bcff95b909de9ae07c9"
    ],
    "codex-rs/tui/src/tui.rs": [
        "9f49da4adb2a39e96c1210e2763e216346bb261f",
        "f17c2d0462f80df9eec027916f4275987441bc93"
    ],
    "codex-rs/tui/src/tui/job_control.rs": [
        "7cb12c874b41a56e2aa945fed3646cc6fcda2ede",
        "4c5a8e8331baa5109cbacfedf18d9deb7bc5ddfa"
    ],
    "codex-rs/tui/src/tui/notification_preview.rs": [
        "",
        "9ed9494c22b40e5a366b45daa106c53412456903"
    ],
    "codex-rs/tui/src/tui/notification_preview_tests.rs": [
        "",
        "5167b4f22e05e5259dbc2417edbafcbe092c3bc2"
    ],
    "codex-rs/tui/src/turn_tip.rs": [
        "2005a9a8595ff65eb2457a50ae30caba4c641f32",
        "de437e32512135206bd3ac17f1e674fff4267af1"
    ]
}
PRESERVED_FORK = {
    "codex-rs/tui/src/app.rs": "b76ee1107fabcc98625fc663c7967e0dc6e906a5",
    "codex-rs/tui/src/app/app_server_event_targets.rs": "521afcee6d57e0adfdb6ffdf89aad77f6bc7c11e",
    "codex-rs/tui/src/app/config_persistence.rs": "a4a84eb699475a4042a3e7b038d2cc323e079c48",
    "codex-rs/tui/src/app/event_dispatch.rs": "86f62f5f80ea9691d96c62ba1151cd87f0c57c1e",
    "codex-rs/tui/src/app/owned_transcript_tests.rs": "28dbe32a743e3608225af6cfeb6ae036b30d74bf",
    "codex-rs/tui/src/app/session_lifecycle.rs": "3954842d391ae36715e335aba3ff9117fb630ae1",
    "codex-rs/tui/src/app/startup.rs": "86f1afec7ce715799c9fa62859e53f312918d067",
    "codex-rs/tui/src/app/test_support.rs": "3007b910d9f155d68faf9f4c2e19346ef1dc848e",
    "codex-rs/tui/src/app/tests.rs": "1a1d42c2cb3586e19aa22ec71f8d264d97ef2909",
    "codex-rs/tui/src/app/tests/disconnect_tests.rs": "e8506446b855b07a5ae44453c0a730bec627fc5d",
    "codex-rs/tui/src/app/tests/navigation_reconnect_tests.rs": "6d23894a630247d8e9ea9be436ecf9a10b733d32",
    "codex-rs/tui/src/app/tests/session_lifecycle_requests.rs": "03f32cbd9806398ce05c1e54d08bf8931434a68c",
    "codex-rs/tui/src/app/tests/startup.rs": "2202f6cc661d54d6950e517e8f50adc4edb50a88",
    "codex-rs/tui/src/app/tests/startup_defaults_tests.rs": "0ac1ba8cd4878e9cb61d412c4e0cb88d629b0398",
    "codex-rs/tui/src/app/thread_routing.rs": "46584906c9c228bf20c7a1c1b676b2940b8f707c",
    "codex-rs/tui/src/app_event.rs": "44cd4adbcc90db4f55500d9f10b3bd8e3220364c",
    "codex-rs/tui/src/app_server_session.rs": "c95f1d080836e934d83296ebab20b01abf1af5be",
    "codex-rs/tui/src/app_server_session/workspace_roots_tests.rs": "d0e8abc5667e513b233d87f9ad7449ca1cc11fc5",
    "codex-rs/tui/src/chatwidget.rs": "e1a042122e0df76b990d4b2f6668de666fe3beab",
    "codex-rs/tui/src/chatwidget/constructor.rs": "ab181ba6d60317df316bd2fcf70ab2385ef1db16",
    "codex-rs/tui/src/chatwidget/input_flow.rs": "a0a393269cf36201c43a901f778bf48823d34d9a",
    "codex-rs/tui/src/chatwidget/input_submission.rs": "c7e06368cf7336bfee8ceaf1ab9ee1913790639a",
    "codex-rs/tui/src/chatwidget/interaction.rs": "f8ec1053365881baae512dc9b1575032bec99ac6",
    "codex-rs/tui/src/chatwidget/protocol.rs": "26e2c0f1500ef11f401c4fd3c92f228a19f9f85b",
    "codex-rs/tui/src/chatwidget/replay.rs": "8b079cdb3863f0d613ec1f9eb702c142c6445133",
    "codex-rs/tui/src/chatwidget/session_flow.rs": "73d7f75b26e5de276be3357a5fda836c24b4090b",
    "codex-rs/tui/src/chatwidget/settings.rs": "3779d5e6c557e3108f32a2ae329fb6a1e684bb10",
    "codex-rs/tui/src/chatwidget/slash_dispatch.rs": "63d8b4b41cc237167a6c32ace5eb60cb193e4c46",
    "codex-rs/tui/src/chatwidget/status_controls.rs": "b815e819fb0742d339d074a2dce2f98577c7c5a4",
    "codex-rs/tui/src/chatwidget/tests.rs": "28a23fc322abc198ee684f6132e7384e672538d2",
    "codex-rs/tui/src/chatwidget/tests/app_server.rs": "8c419bc8d3b1ae418b46a9db1dc3cba640cc8e3a",
    "codex-rs/tui/src/chatwidget/tests/composer_submission.rs": "80a1b2d3022ea6d989b1b25122dfb786106b3c2f",
    "codex-rs/tui/src/chatwidget/tests/history_replay.rs": "f9d994657238fca820e1062e239bb23c51cd2dbe",
    "codex-rs/tui/src/chatwidget/tests/misalignment_policy_tests.rs": "70cd5dfffc22f6f7ae1bdb66643a21a61c5cdc44",
    "codex-rs/tui/src/chatwidget/tests/slash_commands.rs": "d5405150df0ed218cb347010c9a9d817ba1ff3fc",
    "codex-rs/tui/src/chatwidget/tests/status_and_layout.rs": "0d1f1f28923ca09382c0cb2afc9bcf3b2248dbe9",
    "codex-rs/tui/src/cli.rs": "841e9e282941bc856c8c7acf62f094c03c274b58",
    "codex-rs/tui/src/daemon_startup.rs": "2a4c1190086bee4b16f86252ab802bf06e4a0361",
    "codex-rs/tui/src/daemon_startup_tests.rs": "f103b69a4629f9d019cd2a39c3823ae29dc646bf",
    "codex-rs/tui/src/history_cell/tests.rs": "c09c80cbfcea36a41d839822fe15ea9abb3514b5",
    "codex-rs/tui/src/lib.rs": "83ad7124e531be410477bd097cde8950bca40ed2",
    "codex-rs/tui/src/slash_command.rs": "edd98b26ffdfba98d095ca6631830f2571928fe7",
    "codex-rs/tui/src/startup_orchestration.rs": "e03aa30b4723717cfc72935d118e074fbf4f2aae",
    "codex-rs/tui/src/status/tests.rs": "a3091a07c8d8927f7fc8fbeba2ed44434487d1fd"
}
REVIEWED_FORK = {
    "codex-rs/tui/src/cli.rs": "da5c0e4f1ba02b7ff68811323945b0fd4d26f697",
    "codex-rs/tui/src/slash_command.rs": "36e5def7042e71baeeb87945de4fa0b029cd177a",
    "codex-rs/tui/src/daemon_startup.rs": "9e2b577a840d56efee13856ad9f11fd83a936d0d",
    "codex-rs/tui/src/daemon_startup_tests.rs": "4340c3c42ee4dcd4f5c5e06b918648bee4da51a8",
    "codex-rs/tui/src/lib.rs": "7b0746fe66b55537d3c445bfac194cbdb005b2b2",
    "codex-rs/tui/src/app/startup.rs": "3b111def463e2f52b4786b7a6db8c6e3aa7954f7",
    "codex-rs/tui/src/startup_orchestration.rs": "6b3bf4a9cf3d639107126b29992690b19bf4c932",
    "codex-rs/tui/src/chatwidget/replay.rs": "d17ff1c77f63c8c57e0a847c58377999fda78480",
    "codex-rs/tui/src/app/app_server_event_targets.rs": "16d6b02da4469a7f1ef2d82916a48d04719124d5",
    "codex-rs/tui/src/chatwidget/session_flow.rs": "17500409f70dfda7fa1c489258fa19ab01bed747",
    "codex-rs/tui/src/app.rs": "7d9da0b198cfac44c0c06933c622b7c1515a613b",
    "codex-rs/tui/src/chatwidget.rs": "76ae0ab2d341188a6deed0c4347b489fa93f29e4",
    "codex-rs/tui/src/chatwidget/constructor.rs": "a584d5513ea40321e920d795bbb93b8d96e8f03b",
    "codex-rs/tui/src/app_event.rs": "5f43579e23f87a94786ae9803df8dba40060a84e",
    "codex-rs/tui/src/app/event_dispatch.rs": "81df877569e0c205e2a4dec5e9e790f7c7993b93",
    "codex-rs/tui/src/app/session_lifecycle.rs": "fe32d7d8175e6fb039cfa2b21b52b1f8a9463faf",
    "codex-rs/tui/src/chatwidget/slash_dispatch.rs": "57ed2741b86aae99d5ac6054990703c9eabdf845",
    "codex-rs/tui/src/chatwidget/input_flow.rs": "e463da2af9440b3dc8382464a5d0b98ee6758404",
    "codex-rs/tui/src/chatwidget/status_controls.rs": "c3973a6a72da8533b47951b0d529adf839398b1d",
    "codex-rs/tui/src/app/config_persistence.rs": "aece952c906a2f73f87d1d34b3f9583a36f3c722",
    "codex-rs/tui/src/chatwidget/settings.rs": "bb747507416e7712fe4ff5832446783c0127994e",
    "codex-rs/tui/src/app/test_support.rs": "b95971975610ff94ef823eff73e36a4691e7e29c",
    "codex-rs/tui/src/app/tests.rs": "03febfb8e58c93e975f1f240195576a0f1080e08",
    "codex-rs/tui/src/chatwidget/tests/history_replay.rs": "5dec80fbf862d768bd2210c64c6eb1c80123bf4e",
    "codex-rs/tui/src/chatwidget/protocol.rs": "ad71108b91a0d7f1f079cab0482076237b138994",
    "codex-rs/tui/src/chatwidget/input_submission.rs": "17493a1b689e90ddff7ed4be863b5beb7eb6df0a",
    "codex-rs/tui/src/chatwidget/interaction.rs": "e83bc3d1860a8f2f04a14bc136c4b6468785eb67",
    "codex-rs/tui/src/app/thread_routing.rs": "629ddda83497d1447819bd84ca8c436d4b23fdcf",
    "codex-rs/tui/src/app_server_session.rs": "c9ff4fb4294c894490e224590adfc411a7eff2fa",
    "codex-rs/tui/src/app/tests/session_lifecycle_requests.rs": "994d54b631493b3d0784e836c544c57a275e9985",
    "codex-rs/tui/src/app/tests/startup_defaults_tests.rs": "e60808df0448682d47af1a14b5bd8d5c52206397",
    "codex-rs/tui/src/chatwidget/tests.rs": "2b2404e0db36ace3260ecfca0c423cd146b8c95b",
    "codex-rs/tui/src/chatwidget/tests/misalignment_policy_tests.rs": "2354a8e517c49e1c25fceea4b849cb0b077bc366",
    "codex-rs/tui/src/chatwidget/tests/slash_commands.rs": "bec8d92caae4cdff17b7f06923bd4cd9ee358a52",
    "codex-rs/tui/src/chatwidget/tests/status_and_layout.rs": "23ea7ee8024deb95767f8322a42a1f330be715ff",
    "codex-rs/tui/src/status/tests.rs": "e067b499977a4e0dcb81d04f2e0178bf10e1793d",
    "codex-rs/tui/src/app/tests/navigation_reconnect_tests.rs": "e090162290f82156213340967c76a398887ecf44",
    "codex-rs/tui/src/app/owned_transcript_tests.rs": "c691ba3aaa9c8ff10088b09bdc365d7c06a35273",
    "codex-rs/tui/src/history_cell/tests.rs": "f1075c14620c0217e2433f1501c8f171bc59989d",
    "codex-rs/tui/src/app/tests/disconnect_tests.rs": "662967de32dc64517b250dc180baafd1da4dd36a",
    "codex-rs/tui/src/chatwidget/tests/app_server.rs": "53ca9fdae79b07822e8551289b97f1732b5e3ab0",
    "codex-rs/tui/src/app/tests/startup.rs": "a437116e81e54063a2df7e41752ac3e88a3d245d",
    "codex-rs/tui/src/chatwidget/tests/composer_submission.rs": "dd1a4a0f124a74f7a2fdc5b67be309d414a81c1c",
    "codex-rs/tui/src/app_server_session/workspace_roots_tests.rs": "d6447d22a0a1e943b1352b8c945cf7f6cf09734e",
}



def tree_blobs(ref: str) -> dict[str, str]:
    result = subprocess.run(
        ["git", "ls-tree", "-r", "--full-tree", ref, "--", "codex-rs/tui/src"],
        cwd=ROOT, check=True, capture_output=True, text=True, encoding="utf-8",
    )
    out = {}
    for line in result.stdout.splitlines():
        header, path = line.split("\t", 1)
        out[path] = header.split()[2]
    return out


def git_blob(ref: str, path: str) -> str:
    return subprocess.run(
        ["git", "rev-parse", "--verify", ref + ":" + path],
        cwd=ROOT, check=True, capture_output=True, text=True,
    ).stdout.strip()


def source(path: str) -> str:
    return subprocess.run(
        ["git", "show", "HEAD:" + path],
        cwd=ROOT, check=True, capture_output=True, text=True, encoding="utf-8",
    ).stdout


class TuiUpstreamOwnedSourceTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.head = tree_blobs("HEAD")
        cls.production = tree_blobs(PRODUCTION)

    def assertIn(self, item, values, msg=None):
        if isinstance(values, str) and len(values) > 2048:
            self.assertTrue(item in values, msg or f"missing source marker: {item!r}")
            return
        super().assertIn(item, values, msg)

    def test_two_hundred_official_source_blobs(self):
        self.assertEqual(len(STAGED), 200)
        for path, (_, expected) in STAGED.items():
            with self.subTest(path=path):
                self.assertEqual(self.head.get(path), expected)

    def test_original_fork_blobs_not_overwritten(self):
        for path, (original, _) in STAGED.items():
            with self.subTest(path=path):
                self.assertEqual(self.production.get(path), original or None)

    def test_forty_four_adaptive_ui_fork_conflicts_are_preserved_or_reviewed(self):
        self.assertEqual(len(PRESERVED_FORK), 44)
        for path, original in PRESERVED_FORK.items():
            with self.subTest(path=path):
                self.assertEqual(self.production.get(path), original)
                self.assertEqual(self.head.get(path), REVIEWED_FORK.get(path, original))

    def test_upstream_0163_regression_sources_and_snapshots_reviewed(self):
        # Source-provenance only: these fixtures still need a native Windows run.
        self.assertEqual(len(PRESERVED_FORK), 44)
        self.assertTrue(set(PRESERVED_FORK).issubset(REVIEWED_FORK))
        upstream_tests = {
            "codex-rs/tui/src/app/owned_transcript_tests.rs":
                "footer_selection_defers_warning_clicks_and_preserves_the_draft",
            "codex-rs/tui/src/app/tests/disconnect_tests.rs":
                "disconnected_config_panel_keeps_input",
            "codex-rs/tui/src/app/tests/navigation_reconnect_tests.rs":
                "app.reconnect.presentation",
            "codex-rs/tui/src/app/tests/startup.rs":
                "unattended_modals_survive_unfinished_terminal_input",
            "codex-rs/tui/src/app_server_session/workspace_roots_tests.rs":
                "TEST_STACK_SIZE_BYTES",
            "codex-rs/tui/src/chatwidget/tests/app_server.rs":
                "startup_config_warning_panel",
            "codex-rs/tui/src/chatwidget/tests/composer_submission.rs":
                "plugin_mention_does_not_suppress_same_name_skill",
            "codex-rs/tui/src/history_cell/tests.rs":
                "legacy_zsh_migration_notice_snapshot",
        }
        for path, marker in upstream_tests.items():
            with self.subTest(path=path):
                self.assertIn(marker, source(path))
        for path in (
            "codex-rs/tui/src/history_cell/snapshots/codex_tui__history_cell__tests__legacy_zsh_migration_notice_snapshot.snap",
            "codex-rs/tui/src/app/snapshots/codex_tui__app__owned_transcript__tests__fullscreen_footer_selection.snap",
            "codex-rs/tui/src/chatwidget/tests/snapshots/codex_tui__chatwidget__tests__app_server__startup_config_warning_panel.snap",
        ):
            with self.subTest(snapshot=path):
                self.assertTrue(source(path).startswith("---\n"))

    def test_agents_overview_module_staged(self):
        agents = source("codex-rs/tui/src/app/agents_overview.rs")
        self.assertIn("open_agents_overview", agents)
        self.assertIn("agents_overview_view", agents)

    def test_server_request_routing_staged(self):
        requests = source("codex-rs/tui/src/app/app_server_requests.rs")
        self.assertIn("resolve_notification", requests)
        self.assertIn("note_server_request_owned", requests)

    def test_owned_transcript_render_and_history_retained(self):
        transcript = source("codex-rs/tui/src/app/owned_transcript.rs")
        self.assertIn("render_owned_transcript", transcript)
        self.assertIn("request_owned_history", transcript)

    def test_async_questions_state_and_answers_staged(self):
        async_state = source("codex-rs/tui/src/bottom_pane/async_questions/state.rs")
        self.assertIn("resolve_answers", async_state)
        self.assertIn("recovered_draft", async_state)

    def test_chat_composer_agent_navigation_staged(self):
        composer = source("codex-rs/tui/src/bottom_pane/chat_composer/agents_navigation.rs")
        self.assertIn("handle_agents_navigation", composer)

    def test_native_agent_navigation_tests_staged(self):
        tests = source("codex-rs/tui/src/app/tests/agents_navigation_tests.rs")
        self.assertIn("pending_windows_sandbox_setup_blocks_thread_replacement", tests)

    def test_codexdd_adaptive_cli_and_security_sources_untouched(self):
        self.assertEqual(
            git_blob("HEAD", "codex-rs/cli/src/main.rs"),
            "9379d52b41473f1a57cb49d07048af7c139ebcf9",
        )
        self.assertEqual(git_blob("HEAD", "codex-rs/core/src/tools/registry.rs"),
                         "fc75b3bbf91f1881bd32faaaca93356fbb598b5e")

    def test_product_markers_and_lockfile_unadvanced(self):
        self.assertEqual(source("codex-rs/codexdd-version.txt").strip(), "0.4.4")
        self.assertEqual(source("codex-rs/upstream-codex-release.txt").strip(), "rust-v0.162.0")
        self.assertEqual(git_blob("HEAD", "codex-rs/Cargo.lock"),
                         "a6cd27b0b70fde2f2be9c1777f3666ad3fa318bb")

    def test_protected_tui_merges_still_outstanding(self):
        note = (ROOT / "docs/codexdd-0.4.5-packet-3c9c-tui-upstream.md").read_text(
            encoding="utf-8"
        )
        for marker in ("INCOMPLETE / NOT A RELEASE CANDIDATE",
                       "44 unresolved", "source-only", "native Rust", "Cargo.lock"):
            self.assertIn(marker, note)


if __name__ == "__main__":
    unittest.main()
