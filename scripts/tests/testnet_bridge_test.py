#!/usr/bin/env python3
"""Offline tests for the transaction boundary and public deployment plan."""
import importlib.util
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

MODULE_PATH = Path(__file__).resolve().parents[1] / "lib/testnet_bridge.py"
spec = importlib.util.spec_from_file_location("testnet_bridge", MODULE_PATH)
bridge = importlib.util.module_from_spec(spec)
spec.loader.exec_module(bridge)


class BridgePlannerTest(unittest.TestCase):
    def test_railway_commands_are_never_executed(self):
        with patch.object(bridge.subprocess, "run") as tool:
            with self.assertRaisesRegex(bridge.BridgeError, "Railway operations are outside"):
                bridge.run(["railway", "up"])
            tool.assert_not_called()

    def test_default_command_is_read_only_plan(self):
        self.assertEqual(bridge.parser().parse_args([]).command, "plan")

    def test_wrong_evm_chain_refused_before_any_call(self):
        with patch.object(bridge, "rpc", return_value="0x1") as rpc:
            with self.assertRaisesRegex(bridge.BridgeError, "Wrong EVM chain"):
                bridge.Evm("https://example.invalid")
            rpc.assert_called_once()

    def test_known_compressed_public_key_address(self):
        # secp256k1 generator point; no private key is read or printed.
        public = bytes.fromhex("0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798")
        self.assertEqual(bridge.public_evm_address(public), "0x7e5f4552091a69125d5dfcb7b8c2659029395bdf")

    def test_invalid_curve_point_refused(self):
        with self.assertRaises(bridge.BridgeError):
            bridge.public_evm_address(bytes.fromhex("02" + "ff" * 32))

    def test_no_signing_material_in_cli_arguments(self):
        names = {option for action in bridge.parser()._actions for option in action.option_strings}
        self.assertNotIn("--private-key", names)
        self.assertIn("--account", names)

    def test_stale_genesis_refuses_simulation_or_broadcast(self):
        with tempfile.TemporaryDirectory() as temp:
            args = bridge.parser().parse_args(["simulate", "--output", temp])
            bridge.write_json(Path(temp) / "plan.json", {"planHash": "old"})
            with patch.object(bridge, "live_plan", return_value=({"planHash": "new"}, None)):
                with self.assertRaisesRegex(bridge.BridgeError, "Genesis, committee"):
                    bridge.load_fresh(args)

    def test_modified_deployment_config_refused(self):
        with tempfile.TemporaryDirectory() as temp:
            args = bridge.parser().parse_args(["simulate", "--output", temp])
            data = {"mysoGenesis": "abc", "committeeMembers": ["original"]}
            saved = {"planHash": bridge.digest(data)}
            bridge.write_json(Path(temp) / "plan.json", saved)
            bridge.write_json(Path(temp) / "deploy-config.json", {**data, "committeeMembers": ["changed"]})
            with patch.object(bridge, "live_plan", return_value=(saved, None)):
                with self.assertRaisesRegex(bridge.BridgeError, "differs from"):
                    bridge.load_fresh(args)

    def test_deploy_requires_review_hash_and_named_wallet(self):
        with tempfile.TemporaryDirectory() as temp:
            args = bridge.parser().parse_args(["deploy", "--sender", bridge.TOKEN])
            with patch.object(bridge, "load_fresh", return_value=(Path(temp), {"planHash": "reviewed"}, None)), \
                 patch.object(bridge, "run") as tool:
                with self.assertRaisesRegex(bridge.BridgeError, "Broadcast requires"):
                    bridge.forge_deployment(args, broadcast=True)
                tool.assert_not_called()

    def test_changed_sender_nonce_blocks_broadcast(self):
        with tempfile.TemporaryDirectory() as temp:
            args = bridge.parser().parse_args(["deploy", "--sender", bridge.TOKEN,
                                               "--account", "reviewed-wallet", "--approve-plan", "hash"])
            bridge.write_json(Path(temp) / "simulation.json", {"planHash": "hash",
                              "sender": bridge.TOKEN.lower(), "senderNonce": 1})
            with patch.object(bridge, "load_fresh", return_value=(Path(temp), {"planHash": "hash"}, None)), \
                 patch.object(bridge, "rpc", return_value="0x2"), \
                 patch.object(bridge, "run") as tool:
                with self.assertRaisesRegex(bridge.BridgeError, "nonce changed"):
                    bridge.forge_deployment(args, broadcast=True)
                tool.assert_not_called()

    def test_existing_deployment_cannot_be_deployed_again(self):
        with tempfile.TemporaryDirectory() as temp:
            (Path(temp) / "deployment.json").write_text("{}")
            with patch.object(bridge, "load_fresh", return_value=(Path(temp), {}, None)), \
                 patch.object(bridge, "run") as tool:
                with self.assertRaisesRegex(bridge.BridgeError, "already recorded"):
                    bridge.forge_deployment(bridge.parser().parse_args(["deploy"]), broadcast=True)
                tool.assert_not_called()

    def test_simulate_never_adds_broadcast_or_wallet_flags(self):
        with tempfile.TemporaryDirectory() as temp:
            args = bridge.parser().parse_args(["simulate", "--sender", bridge.TOKEN])
            names = ["MySoBridge", "BridgeCommittee", "BridgeConfig", "BridgeLimiter", "BridgeVault", "Deployment"]
            output = "\n".join(f"[Deployed] {name}: 0x{i:040x}" for i, name in enumerate(names, 1))
            with patch.object(bridge, "load_fresh", return_value=(Path(temp), {"planHash": "hash"}, None)), \
                 patch.object(bridge, "run", return_value=output) as tool, \
                 patch.object(bridge, "rpc", return_value="0x0"):
                bridge.forge_deployment(args)
                command = tool.call_args.args[0]
                self.assertNotIn("--broadcast", command)
                self.assertNotIn("--account", command)
                self.assertFalse((Path(temp) / "deployment.json").exists())
                self.assertEqual(bridge.load(Path(temp) / "simulation.json")["status"], "simulation-only")

    def test_preview_config_has_placeholders_and_never_raw_secret(self):
        with tempfile.TemporaryDirectory() as temp:
            member = {"evmAddress": bridge.TOKEN.lower(), "mysoAddress": "0x123", "url": "https://signer.invalid", "votingPower": 10000}
            plan = {"planHash": "hash", "members": [member], "observedEvmBlock": 42,
                    "authorityKeyFiles": {member["evmAddress"]: "/keys/bridge_validator_1.key.json"},
                    "adapterOwner": bridge.TOKEN, "deployConfig": {"adapter": bridge.ADAPTER, "mysoGenesis": "testgenesis"}}
            bridge.write_json(Path(temp) / "simulation.json", {"planHash": "hash", "addresses": {"proxy": bridge.OLD_PROXY}})
            args = bridge.parser().parse_args(["configure", "--preview"])
            with patch.object(bridge, "load_fresh", return_value=(Path(temp), plan, None)), \
                 patch.object(bridge, "run", return_value="0x1234"):
                bridge.configure(args)
            node = bridge.load(Path(temp) / "preview-configs/validator-1.yaml")
            self.assertEqual(node["deposits"]["relayer-eth-private-key"], "${BRIDGE_RELAYER_PRIVATE_KEY}")
            self.assertEqual(node["approved-governance-actions"], [])
            self.assertFalse(node["deposits"]["enabled"])
            self.assertFalse((Path(temp) / "configs").exists())

    def test_source_digest_changes_when_sources_change(self):
        self.assertRegex(bridge.source_hash(), r"^[0-9a-f]{64}$")


if __name__ == "__main__":
    unittest.main()
