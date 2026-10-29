#!/usr/bin/env python3
"""Base Sepolia / MySo testnet bridge planner. Never reads or generates authority secrets.

Authority files are inspected by the CLI's public-only examine-key command. Only the
explicit deploy subcommand can broadcast; plan, check, configure, and simulate cannot.
"""
import argparse
import base64
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tempfile
import time
import urllib.error
import urllib.request

ROOT = Path(__file__).resolve().parents[2]
EVM_DIR = ROOT / "bridge/evm"
TOKEN = "0x8a9e9Ad05010aD980a1d24b61bC2a099B13D42a2"
ADAPTER = "0x643e8c1Cf3Ef9f76D4EF2a81C461Fe39c2F2C7f8"
OLD_PROXY = "0x538f62e8E903DcaAAB6eB2cD1c5b9A8742e66CB1"
ZERO = "0x" + "00" * 20
SCRIPT = "script/deploy_native_testnet.s.sol:DeployNativeTestnet"


class BridgeError(Exception):
    pass


def redact(value):
    return re.sub(r"https?://[^\s\"']+", "<RPC URL withheld>", str(value))


def run(args, cwd=None, env=None, timeout=120):
    if Path(args[0]).name == "railway":
        raise BridgeError("Railway operations are outside this script; only local config generation is supported")
    try:
        p = subprocess.run([str(x) for x in args], cwd=cwd, env=env,
                           capture_output=True, text=True, timeout=timeout)
    except (OSError, subprocess.TimeoutExpired):
        raise BridgeError(f"{Path(args[0]).name} unavailable or timed out") from None
    if p.returncode:
        # Never forward arbitrary tool output: config parsing can include runtime secrets.
        raise BridgeError(f"{Path(args[0]).name} failed; no transaction was inferred successful")
    return p.stdout.strip()


def rpc(url, method, params=None):
    body = json.dumps({"jsonrpc": "2.0", "id": 1, "method": method,
                       "params": params or []}).encode()
    req = urllib.request.Request(url, body, {"Content-Type": "application/json", "User-Agent": "myso-testnet-bridge/1.0"})
    for attempt in range(3):
        try:
            with urllib.request.urlopen(req, timeout=30) as response:
                d = json.load(response)
        except (OSError, ValueError):
            if attempt < 2:
                time.sleep(0.3 * (attempt + 1))
                continue
            raise BridgeError(f"RPC request failed: {method} (URL and response withheld)") from None
        error = d.get("error")
        if error and error.get("code") in (-32000, -32005, -32603) and attempt < 2:
            time.sleep(0.3 * (attempt + 1))
            continue
        if error or "result" not in d:
            code = error.get("code") if isinstance(error, dict) else "unknown"
            raise BridgeError(f"RPC method failed: {method} (code {code})")
        return d["result"]


def addr(value):
    if not re.fullmatch(r"0x[0-9a-fA-F]{40}", value or ""):
        raise BridgeError("Expected a public EVM address")
    return value.lower()


def number(value):
    return int(str(value).split()[0], 0)


class Evm:
    def __init__(self, url):
        self.url = url
        if int(rpc(url, "eth_chainId"), 16) != 84532:
            raise BridgeError("Wrong EVM chain: expected Base Sepolia 84532")
        self.block = rpc(url, "eth_blockNumber")

    def code(self, address):
        return rpc(self.url, "eth_getCode", [addr(address), self.block])

    def call(self, address, signature, *values):
        data = run(["cast", "calldata", signature, *map(str, values)])
        try:
            return rpc(self.url, "eth_call", [{"to": addr(address), "data": data}, self.block])
        except BridgeError:
            raise BridgeError(f"Read failed: {signature} at {addr(address)}") from None

    def address(self, address, signature, *values):
        return addr("0x" + self.call(address, signature, *values)[-40:])

    def integer(self, address, signature, *values):
        return int(self.call(address, signature, *values), 16)


def public_evm_address(compressed):
    b = bytes(compressed)
    if len(b) != 33 or b[0] not in (2, 3):
        raise BridgeError("Invalid compressed secp256k1 public key in committee")
    p = 2**256 - 2**32 - 977
    x = int.from_bytes(b[1:], "big")
    if x >= p:
        raise BridgeError("Invalid committee public key coordinate")
    y = pow((x**3 + 7) % p, (p + 1) // 4, p)
    if y * y % p != (x**3 + 7) % p:
        raise BridgeError("Committee public key is not on secp256k1")
    if y % 2 != b[0] % 2:
        y = p - y
    raw = (x.to_bytes(32, "big") + y.to_bytes(32, "big")).hex()
    return "0x" + run(["cast", "keccak", "0x" + raw])[-40:]


def committee_from_summary(summary):
    members = []
    for _, item in summary["committee"]["members"]:
        if item["blocklisted"]:
            raise BridgeError("A live MySo bridge committee member is blocklisted")
        url = bytes(item["httpRestUrl"]).decode()
        if not url.startswith(("http://", "https://")):
            raise BridgeError("Invalid committee HTTP URL")
        members.append({"evmAddress": public_evm_address(item["bridgePubkeyBytes"]),
                        "mysoAddress": item["mysoAddress"], "votingPower": int(item["votingPower"]),
                        "publicKey": bytes(item["bridgePubkeyBytes"]).hex(), "url": url})
    members.sort(key=lambda m: m["evmAddress"])
    if not members or sum(m["votingPower"] for m in members) != 10000:
        raise BridgeError("MySo bridge committee must be seated with 10000 total voting power")
    if any(not 0 < m["votingPower"] <= 10000 for m in members):
        raise BridgeError("Invalid committee voting power")
    if len({m["evmAddress"] for m in members}) != len(members):
        raise BridgeError("Duplicate committee public keys")
    return members


def canonical(data):
    return json.dumps(data, sort_keys=True, separators=(",", ":")).encode()


def digest(data):
    return hashlib.sha256(canonical(data)).hexdigest()


def write_json(path, data):
    path = Path(path)
    path.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.NamedTemporaryFile(mode="w", dir=path.parent, delete=False) as f:
        json.dump(data, f, indent=2)
        f.write("\n")
        temp = f.name
    os.replace(temp, path)


def load(path):
    try:
        return json.loads(Path(path).read_text())
    except (OSError, ValueError):
        raise BridgeError(f"Missing or invalid public JSON file: {Path(path).name}") from None


def inspect_key(path, validator=True):
    args = ["myso-bridge", "examine-key", path]
    if validator:
        args.append("--is-validator-key")
    out = run(args)
    match = re.search(r"Corresponding Ethereum address:\s*(?:0x)?([0-9a-fA-F]{40})", out)
    if not match:
        raise BridgeError("Could not derive public EVM address from key file")
    return addr("0x" + match[1])


def source_hash():
    files = list((EVM_DIR / "contracts").rglob("*.sol"))
    files.extend(EVM_DIR / x for x in ["script/deploy_native_testnet.s.sol", "foundry.toml", "remappings.txt", "soldeer.lock"])
    files.extend([Path(__file__), ROOT / "scripts/testnet-bridge-bootstrap.sh"])
    return digest({str(p.relative_to(ROOT)): hashlib.sha256(p.read_bytes()).hexdigest()
                   for p in sorted(files) if p.is_file()})


def live_plan(args):
    evm = Evm(args.evm_rpc)
    summary = rpc(args.myso_rpc, "mysox_getLatestBridge")
    if int(summary["chainId"]) != 2 or summary["isFrozen"]:
        raise BridgeError("Expected unfrozen MySo bridge chain 2")
    treasury = summary["treasury"]
    if not treasury["nativeMysoBootstrapped"] or int(treasury["nativeMysoLocked"]) <= 0:
        raise BridgeError("Native MYSO reserve has not been bootstrapped")
    native = [(name, item) for name, item in treasury["supportedTokens"] if int(item["id"]) == 0]
    if len(native) != 1 or not native[0][0].endswith("::myso::MYSO"):
        raise BridgeError("Genesis MYSO token id 0 is missing")
    name, meta = native[0]
    if int(name.split("::")[0], 16) != 2 or not meta["nativeToken"] or int(meta["decimalMultiplier"]) != 10**9:
        raise BridgeError("Native MYSO registry metadata mismatch")
    token, adapter = addr(args.token), addr(args.adapter)
    if evm.code(token) == "0x" or evm.code(adapter) == "0x":
        raise BridgeError("Token or adapter has no code on Base Sepolia")
    if evm.address(adapter, "mySocialToken()") != token:
        raise BridgeError("Existing adapter is bound to a different token")
    if evm.integer(adapter, "decimals()") != 18 or evm.integer(token, "decimals()") != 18:
        raise BridgeError("Expected 18-decimal EVM representation")
    if evm.address(token, "presaleContract()") != adapter:
        raise BridgeError("Token mint/burn authorization does not point to this adapter")
    members = committee_from_summary(summary)
    config = {"schemaVersion": 1, "sourceHash": source_hash(), "evmChainId": 84532, "mysoBridgeChainId": 2,
              "evmBridgeChainId": 12, "mysoGenesis": rpc(args.myso_rpc, "myso_getChainIdentifier"),
              "token": token, "adapter": adapter, "tokenId": 0,
              "tokenPrice": int(meta["notionalValue"]), "dailyLimitUsd8": args.daily_limit_usd * 10**8,
              "committeeMembers": [m["evmAddress"] for m in members],
              "committeeMemberStake": [m["votingPower"] for m in members]}
    if not 0 < config["tokenPrice"] < 2**64 or not 0 < config["dailyLimitUsd8"] < 2**64:
        raise BridgeError("Price or limit out of range")
    key_matches = {}
    if args.key_dir:
        for path in sorted(Path(args.key_dir).glob("bridge_validator_*.key.json")):
            address = inspect_key(path)
            if address in config["committeeMembers"]:
                if address in key_matches:
                    raise BridgeError("Duplicate authority key files for a committee member")
                key_matches[address] = str(path.resolve())
        if set(key_matches) != set(config["committeeMembers"]):
            raise BridgeError("Local validator key files do not match the entire live committee")
    plan = {"status": "prepared-not-deployed", "planHash": digest(config), "deployConfig": config,
            "observedEvmBlock": int(evm.block, 16), "members": members, "authorityKeyFiles": key_matches,
            "adapterOwner": evm.address(adapter, "owner()"), "tokenOwner": evm.address(token, "owner()"),
            "nativeMysoLockedBaseUnits": str(treasury["nativeMysoLocked"]),
            "mysoMessageVersion": int(summary["messageVersion"])}
    plan["legacyProxyAuthorization"] = {
        "proxy": addr(OLD_PROXY),
        "authorized": bool(evm.integer(adapter, "authorizedBridges(address)", OLD_PROXY)),
        "retirement": "unsigned owner transaction prepared; not automatically executed"}
    plan["memberBalances"] = []
    for m in members:
        balance = rpc(args.myso_rpc, "mysox_getBalance", [m["mysoAddress"], "0x2::myso::MYSO"])
        plan["memberBalances"].append({"evmAddress": m["evmAddress"],
                                       "mysoAddress": m["mysoAddress"],
                                       "mysoBaseUnits": str(balance["totalBalance"])})
    return plan, evm


def verify_proxy(evm, plan, proxy):
    proxy = addr(proxy)
    if evm.code(proxy) == "0x":
        raise BridgeError("Bridge proxy has no code")
    c = evm.address(proxy, "committee()")
    cfg = evm.address(c, "config()")
    issues = []
    addresses = {"proxy": proxy, "committee": c, "config": cfg}
    expected = plan["deployConfig"]

    def require_read(reason, read):
        try:
            if not read():
                issues.append(reason)
        except BridgeError:
            issues.append(reason + " (read failed or incompatible ABI)")

    for getter in ("limiter", "vault"):
        try:
            addresses[getter] = evm.address(proxy, getter + "()")
        except BridgeError:
            issues.append("Could not read bridge " + getter)
    require_read("wrong EVM bridge chain ID", lambda: evm.integer(cfg, "chainID()") == 12)
    require_read("MySo route disabled", lambda: evm.integer(cfg, "isChainSupported(uint8)", 2) == 1)
    require_read("token id 0 adapter mismatch", lambda: evm.address(cfg, "tokenAddressOf(uint8)", 0) == expected["adapter"])
    require_read("wrong MySo decimals", lambda: evm.integer(cfg, "tokenMySoDecimalOf(uint8)", 0) == 9)
    require_read("price differs from MySo", lambda: evm.integer(cfg, "tokenPriceOf(uint8)", 0) == expected["tokenPrice"])
    require_read("bridge paused", lambda: evm.integer(proxy, "paused()") == 0)
    for name in ("limiter", "vault"):
        if name in addresses:
            require_read(name + " owner mismatch", lambda n=name: evm.address(addresses[n], "owner()") == proxy)
    require_read("adapter has not authorized this proxy", lambda: evm.integer(expected["adapter"], "authorizedBridges(address)", proxy) == 1)
    for m in plan["members"]:
        require_read("committee stake mismatch: " + m["evmAddress"],
                     lambda member=m: evm.integer(c, "committeeStake(address)", member["evmAddress"]) == member["votingPower"])
        require_read("committee member blocklisted: " + m["evmAddress"],
                     lambda member=m: evm.integer(c, "blocklist(address)", member["evmAddress"]) == 0)
    require_read("native adapter mismatch", lambda: evm.address(proxy, "nativeAdapter()") == expected["adapter"])
    require_read("native token mismatch", lambda: evm.address(proxy, "nativeToken()") == expected["token"])
    return {**addresses, "issues": issues, "observedEvmBlock": int(evm.block, 16)}


def prepare(args):
    plan, evm = live_plan(args)
    out = Path(args.output).resolve()
    if (out / "deployment.json").exists():
        saved = load(out / "deployment.json")
        if saved["planHash"] != plan["planHash"]:
            raise BridgeError("Existing deployment belongs to a different plan; choose another output directory")
    write_json(out / "plan.json", plan)
    write_json(out / "deploy-config.json", plan["deployConfig"])
    if args.proxy:
        result = verify_proxy(evm, plan, args.proxy)
        write_json(out / "existing-bridge-check.json", result)
        print(f"Existing proxy: {len(result['issues'])} issues; details in existing-bridge-check.json")
    print(f"Plan {plan['planHash']}\nPrepared {len(plan['members'])} validators; token and adapter retained.")
    print(f"Native reserve already bootstrapped. Daily mint limit: ${args.daily_limit_usd:,}.")
    print("No transactions sent. Public plan directory:", out)
    return plan


def load_fresh(args):
    out = Path(args.output).resolve()
    saved = load(out / "plan.json")
    live, evm = live_plan(args)
    if live["planHash"] != saved["planHash"]:
        raise BridgeError("Genesis, committee, token, or limit changed; prepare and review a new plan")
    if digest(load(out / "deploy-config.json")) != saved["planHash"]:
        raise BridgeError("Deployment config differs from the reviewed plan")
    return out, saved, evm


def forge_deployment(args, broadcast=False):
    out, plan, evm = load_fresh(args)
    if (out / "deployment.json").exists():
        raise BridgeError("A deployment is already recorded here; run check instead of redeploying")
    if not args.sender:
        raise BridgeError("Provide a public --sender address for simulation/deployment")
    sender = addr(args.sender)
    if args.preview:
        raise BridgeError("--preview is only valid with configure")
    if broadcast and (bool(args.account) == bool(args.keystore) or args.approve_plan != plan["planHash"]):
        raise BridgeError("Broadcast requires one encrypted --account or --keystore and --approve-plan matching planHash")
    sender_nonce = int(rpc(args.evm_rpc, "eth_getTransactionCount", [sender, "latest"]), 16)
    if broadcast:
        simulation = load(out / "simulation.json")
        if (simulation["planHash"] != plan["planHash"] or simulation["sender"] != sender
                or simulation["senderNonce"] != sender_nonce):
            raise BridgeError("Deployer or nonce changed since simulation; re-simulate and review predicted addresses")
    env = os.environ.copy()
    env["BRIDGE_PLAN_CONFIG"] = str(out / "deploy-config.json")
    env["BRIDGE_DEPLOYER"] = sender
    cmd = ["forge", "script", SCRIPT, "--rpc-url", args.evm_rpc,
           "--chain", "84532", "--sender", sender, "--optimize"]
    if broadcast:
        wallet = ["--keystore", args.keystore] if args.keystore else ["--account", args.account]
        if addr(run(["cast", "wallet", "address", *wallet], env=env)) != sender:
            raise BridgeError("Signing wallet does not match reviewed deployer")
        cmd += ["--broadcast", *wallet]
    # Foundry preserves partial broadcast receipts. Never blindly retry a partially sent suite.
    receipt_path = EVM_DIR / "broadcast/deploy_native_testnet.s.sol/84532/run-latest.json"
    if broadcast and receipt_path.exists():
        raise BridgeError("Prior broadcast receipts exist; reconcile them before any new broadcast")
    try:
        raw = run(cmd, cwd=EVM_DIR, env=env, timeout=600)
    except BridgeError:
        raise BridgeError("Forge did not complete. If broadcasting, inspect Foundry receipts before retrying.") from None
    clean = redact(raw)
    (out / ("deployment.log" if broadcast else "simulation.log")).write_text(clean + "\n")
    names = {"MySoBridge": "proxy", "BridgeCommittee": "committee", "BridgeConfig": "config",
             "BridgeLimiter": "limiter", "BridgeVault": "vault", "Deployment": "deployment"}
    addresses = {names[k]: addr(v) for k, v in re.findall(
        r"\[Deployed\]\s+(\w+):\s*(0x[0-9a-fA-F]{40})", raw) if k in names}
    if set(addresses) != set(names.values()):
        raise BridgeError("Forge output did not include the full deployment; reconcile receipts")
    estimated = re.search(r"Estimated amount required:\s*([0-9.]+) ETH", raw)
    record = {"estimatedEth": estimated[1] if estimated else None,
              "status": "broadcast" if broadcast else "simulation-only", "planHash": plan["planHash"],
              "sender": sender, "senderNonce": sender_nonce, "addresses": addresses}
    if broadcast:
        receipts = load(receipt_path)
        entries = receipts.get("receipts", [])
        if len(entries) != 5 or any(number(r["status"]) != 1 for r in entries):
            raise BridgeError("Missing or failed deployment receipts; reconcile before restarting")
        record["receipts"] = entries
        record["startBlock"] = min(number(r["blockNumber"]) for r in entries)
        write_json(out / "deployment.json", record)
        print("Deployment receipts recorded. Adapter authorization is a separate owner transaction.")
    else:
        write_json(out / "simulation.json", record)
        print("Deployment simulation succeeded; no transactions sent. Predicted addresses are not deployed.")
    print(json.dumps(addresses, indent=2))


def yaml_document(config):
    # JSON is a YAML 1.2 subset and is accepted by serde_yaml; avoids a Python YAML dependency.
    return json.dumps(config, indent=2) + "\n"


def configure(args):
    out, plan, evm = load_fresh(args)
    record = load(out / ("simulation.json" if args.preview else "deployment.json"))
    if record["planHash"] != plan["planHash"]:
        raise BridgeError("Deployment record belongs to a different plan")
    if not args.preview:
        check = verify_proxy(evm, plan, record["addresses"]["proxy"])
        if any(issue != "adapter has not authorized this proxy" for issue in check["issues"]):
            raise BridgeError("Bridge verification failed; run check before configuring nodes")
    if not plan["authorityKeyFiles"]:
        raise BridgeError("Prepare the plan with --key-dir to associate validators with node configs")
    target = out / ("preview-configs" if args.preview else "configs")
    target.mkdir(exist_ok=True)
    proxy = record["addresses"]["proxy"]
    start = record.get("startBlock", plan["observedEvmBlock"])
    ordered_members = sorted(plan["members"], key=lambda m: plan["authorityKeyFiles"][m["evmAddress"]])
    for i, member in enumerate(ordered_members, 1):
        config = {"server-listen-port": 8080, "metrics-port": 9090,
                  "bridge-authority-key-path": "${BRIDGE_AUTHORITY_KEY_PATH}",
                  "run-client": i == 1, "approved-governance-actions": [],
                  "myso": {"myso-rpc-url": "${MYSO_RPC_URL}", "myso-bridge-chain-id": 2},
                  "eth": {"eth-rpc-urls": ["${ETH_RPC_URL}"], "eth-rpc-quorum": 1,
                          "eth-bridge-proxy-address": proxy, "eth-bridge-chain-id": 12,
                          "eth-contracts-start-block-fallback": start}}
        if i == 1:
            config["db-path"] = "${BRIDGE_DB_PATH}"
            config["deposits"] = {"enabled": False, "auto-fund-gas": False,
                                  "relayer-eth-private-key": "${BRIDGE_RELAYER_PRIVATE_KEY}"}
        (target / f"validator-{i}.yaml").write_text(yaml_document(config))
    write_json(target / "validator-map.json", [{"config": f"validator-{i}.yaml", **m,
               "authorityKeyFile": plan["authorityKeyFiles"][m["evmAddress"]],
               "runClient": i == 1} for i, m in enumerate(ordered_members, 1)])
    # Existing Railway images already provide envsubst and the node binary.
    # Render every placeholder, including the EVM relayer, without persisting secrets.
    starts = []
    for i, member in enumerate(ordered_members, 1):
        parts = ["set -eu", "umask 077", ': "${BRIDGE_AUTHORITY_KEY:?Set the matching authority secret}"',
                 f'export BRIDGE_AUTHORITY_KEY_PATH=/tmp/bridge_key_validator_{i}',
                 'export MYSO_RPC_URL="${MYSO_RPC_URL:-https://fullnode.testnet.mysocial.network:443}"',
                 'export ETH_RPC_URL="${ETH_RPC_URL:-https://sepolia.base.org}"',
                 f'export BRIDGE_DB_PATH="${{BRIDGE_DB_PATH:-/data/bridge-{plan["deployConfig"]["mysoGenesis"]}-{proxy[2:10]}/client-db}}"',
                 "printf '%s' \"$BRIDGE_AUTHORITY_KEY\" > \"$BRIDGE_AUTHORITY_KEY_PATH\"",
                 'chmod 600 "$BRIDGE_AUTHORITY_KEY_PATH"']
        if i == 1:
            parts += [': "${BRIDGE_RELAYER_PRIVATE_KEY:?Set the funded EVM relayer secret}"',
                      'mkdir -p "$BRIDGE_DB_PATH"']
        parts += [f"envsubst '$ETH_RPC_URL $MYSO_RPC_URL $BRIDGE_RELAYER_PRIVATE_KEY $BRIDGE_AUTHORITY_KEY_PATH $BRIDGE_DB_PATH' < /app/bridge-config-validator-{i}.yaml > /tmp/testnet-bridge-node.yaml",
                  'chmod 600 /tmp/testnet-bridge-node.yaml',
                  'exec /work/target/release/myso-bridge-node --config-path /tmp/testnet-bridge-node.yaml']
        starts.append({"validator": i, "evmAddress": member["evmAddress"],
                       "templateToCopy": f"validator-{i}.yaml",
                       "imageConfigPath": f"/app/bridge-config-validator-{i}.yaml",
                       "shellCommand": "; ".join(parts)})
    write_json(target / "railway-start-commands.json", starts)
    owner_data = run(["cast", "calldata", "setAuthorizedBridge(address,bool)", proxy, "true"])
    write_json(target / "adapter-authorization.json", {"chainId": 84532,
               "from": plan["adapterOwner"], "to": plan["deployConfig"]["adapter"],
               "data": owner_data, "value": "0x0", "status": "unsigned-not-sent"})
    legacy = plan.get("legacyProxyAuthorization", {})
    if legacy.get("authorized"):
        revoke_data = run(["cast", "calldata", "setAuthorizedBridge(address,bool)", legacy["proxy"], "false"])
        write_json(target / "retire-legacy-proxy.json", {"chainId": 84532,
                   "from": plan["adapterOwner"], "to": plan["deployConfig"]["adapter"],
                   "data": revoke_data, "value": "0x0", "legacyProxy": legacy["proxy"],
                   "status": "unsigned-not-sent"})
    print("Preview only; not deployable until receipts exist." if args.preview else "Verified node configs generated.")
    print("Configs, public validator map, and unsigned adapter authorization:", target)


def check(args):
    _, plan, evm = load_fresh(args)
    proxy = args.proxy
    if not proxy:
        deployment = load(Path(args.output) / "deployment.json")
        if deployment["planHash"] != plan["planHash"]:
            raise BridgeError("Deployment belongs to a different plan")
        proxy = deployment["addresses"]["proxy"]
    result = verify_proxy(evm, plan, proxy)
    endpoint_checks = []
    for member in plan["members"]:
        try:
            with urllib.request.urlopen(member["url"].rstrip("/") + "/ping", timeout=8) as r:
                ok = r.status == 200
        except OSError:
            ok = False
        endpoint_checks.append({"evmAddress": member["evmAddress"], "reachable": ok})
    result["signerEndpoints"] = endpoint_checks
    if not all(x["reachable"] for x in endpoint_checks):
        result["issues"].append("one or more registered signing endpoints are unreachable")
    write_json(Path(args.output) / "health.json", result)
    print(json.dumps(result, indent=2))
    return 1 if result["issues"] else 0


def node(args):
    out, plan, evm = load_fresh(args)
    record = load(out / "deployment.json")
    result = verify_proxy(evm, plan, record["addresses"]["proxy"])
    if result["issues"]:
        raise BridgeError("Refusing node startup: bridge authorization/configuration is incomplete")
    if record["planHash"] != plan["planHash"]:
        raise BridgeError("Deployment belongs to a different plan")
    template = load(out / "configs" / f"validator-{args.validator}.yaml")
    mapping = load(out / "configs/validator-map.json")
    entry = mapping[args.validator - 1]
    authority = os.environ.get("BRIDGE_AUTHORITY_KEY_PATH", entry["authorityKeyFile"])
    if inspect_key(authority) != entry["evmAddress"]:
        raise BridgeError("Authority key does not match the selected validator")
    replacements = {"BRIDGE_AUTHORITY_KEY_PATH": authority, "MYSO_RPC_URL": args.myso_rpc,
                    "ETH_RPC_URL": args.evm_rpc,
                    "BRIDGE_DB_PATH": os.environ.get("BRIDGE_DB_PATH", str(out / "client-db"))}
    if template["run-client"]:
        client = os.environ.get("BRIDGE_CLIENT_KEY_PATH")
        if client:
            # A separate client key is optional; otherwise the funded authority pays MySo gas.
            template["myso"]["bridge-client-key-path"] = client
            info = run(["myso-bridge", "examine-key", client])
            match = re.search(r"Corresponding MySo address:\s*(0x[0-9a-fA-F]+)", info)
            if not match:
                raise BridgeError("Could not derive MySo client address")
            payer = match[1]
        else:
            payer = entry["mysoAddress"]
        balance = rpc(args.myso_rpc, "mysox_getBalance", [payer, "0x2::myso::MYSO"])
        if int(balance["totalBalance"]) < 10_000_000_000:
            raise BridgeError("MySo gas payer needs at least 10 MYSO")
        secret = os.environ.get("BRIDGE_RELAYER_PRIVATE_KEY")
        key_file = os.environ.get("BRIDGE_RELAYER_KEY_FILE")
        if not secret and key_file:
            try:
                decoded = base64.b64decode(Path(key_file).read_text().strip(), validate=True)
            except (OSError, ValueError):
                raise BridgeError("Cannot read relayer key file; contents withheld") from None
            if len(decoded) != 33 or decoded[0] != 1:
                raise BridgeError("Relayer key file must be a flagged secp256k1 client key")
            secret = "0x" + decoded[1:].hex()
        if not secret or not re.fullmatch(r"(?:0x)?[0-9a-fA-F]{64}", secret):
            raise BridgeError("Client node requires BRIDGE_RELAYER_KEY_FILE or BRIDGE_RELAYER_PRIVATE_KEY")
        private = bytes.fromhex(secret.removeprefix("0x"))
        der = bytes.fromhex("302e0201010420") + private + bytes.fromhex("a00706052b8104000a")
        try:
            public = subprocess.run(["openssl", "ec", "-inform", "DER", "-pubout", "-outform", "DER"],
                                    input=der, capture_output=True, check=True).stdout[-65:]
        except (OSError, subprocess.CalledProcessError):
            raise BridgeError("Relayer key derivation failed; details withheld") from None
        if len(public) != 65 or public[0] != 4:
            raise BridgeError("Invalid relayer public key")
        relayer = "0x" + run(["cast", "keccak", "0x" + public[1:].hex()])[-40:]
        if int(rpc(args.evm_rpc, "eth_getBalance", [relayer, "latest"]), 16) < 100_000_000_000_000:
            raise BridgeError("EVM relayer needs at least 0.0001 Base Sepolia ETH")
        replacements.update(BRIDGE_RELAYER_PRIVATE_KEY=secret)
    text = json.dumps(template)
    for key, value in replacements.items():
        text = text.replace("${" + key + "}", json.dumps(value)[1:-1])
    if "${" in text:
        raise BridgeError("Unresolved node config placeholders")
    config = json.loads(text)
    config["server-listen-port"] = args.port
    config["metrics-port"] = args.metrics_port
    # Runtime secret is restricted and removed when the foreground process stops.
    with tempfile.TemporaryDirectory(prefix="myso-testnet-node-") as temp:
        path = Path(temp) / "node.yaml"
        path.write_text(yaml_document(config)); path.chmod(0o600)
        try:
            binary = os.environ.get("BRIDGE_NODE_BIN") or shutil.which("myso-bridge-node")
            if not binary:
                binary = next((str(p) for p in [ROOT / "target/release/myso-bridge-node",
                                               ROOT / "target/debug/myso-bridge-node"] if p.is_file()), None)
            if not binary:
                raise BridgeError("Build myso-bridge-node or set BRIDGE_NODE_BIN")
            p = subprocess.run([binary, "--config-path", str(path)])
        except OSError:
            raise BridgeError("myso-bridge-node is unavailable") from None
        return p.returncode


def parser():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("command", nargs="?", default="plan",
                   choices=["plan", "check", "simulate", "deploy", "configure", "node"])
    p.add_argument("--myso-rpc", default=os.environ.get("MYSO_RPC_URL", "https://fullnode.testnet.mysocial.network:443"))
    p.add_argument("--evm-rpc", default=os.environ.get("ETH_RPC_URL", "https://sepolia.base.org"))
    p.add_argument("--token", default=TOKEN)
    p.add_argument("--adapter", default=ADAPTER)
    p.add_argument("--proxy", help="Existing public proxy to inspect (never automatically upgraded)")
    p.add_argument("--key-dir", help="Directory containing existing bridge_validator_*.key.json files")
    p.add_argument("--output", default=str(ROOT / "network.config/testnet-bridge"))
    p.add_argument("--daily-limit-usd", type=int, default=10000)
    p.add_argument("--sender", help="Public EVM deployer address")
    p.add_argument("--account", help="Named encrypted Foundry wallet (deploy only)")
    p.add_argument("--keystore", help="Encrypted Foundry keystore path (deploy only)")
    p.add_argument("--approve-plan", help="Exact reviewed plan hash (deploy only)")
    p.add_argument("--preview", action="store_true", help="Generate configs from simulation, never for startup")
    p.add_argument("--validator", type=int, choices=[1, 2, 3], default=1)
    p.add_argument("--port", type=int, default=8080)
    p.add_argument("--metrics-port", type=int, default=9090)
    return p


def main():
    args = parser().parse_args()
    actions = {"plan": prepare, "check": check, "simulate": forge_deployment,
               "deploy": lambda a: forge_deployment(a, broadcast=True),
               "configure": configure, "node": node}
    try:
        result = actions[args.command](args)
        return result if isinstance(result, int) else 0
    except (BridgeError, KeyError, IndexError, TypeError, ValueError) as error:
        message = str(error) if isinstance(error, BridgeError) else "Unexpected response or config format; stopped"
        print("Stopped:", message, file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main())
