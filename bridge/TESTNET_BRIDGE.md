# Native MYSO testnet bridge

`scripts/testnet-bridge-bootstrap.sh` prepares a fresh Base Sepolia bridge for the
live MySo testnet genesis committee. It keeps the existing ERC20 and adapter. It
never resets genesis, creates keys, bootstraps the native reserve again, swaps
coins, upgrades an old proxy, or rewrites the checked-in Railway configs.

## Where to run it

Run commands in Terminal on your Mac from
`/Users/brandonshaw/Offline-Projects/ProjectYZ/myso-core`.
The script never pushes code, uploads Railway variables, restarts services, or
invokes the Railway CLI/API. Railway startup templates are local code/config
outputs only. Service deployment is outside this workflow. The longer Railway
deployment command block previously supplied in chat is not part of the script
and should not be run for local preparation.

The old localnet deployer is deliberately untouched. It uses chain 31337, a
single validator, and mock tokens; it is not the Base Sepolia deployment path.

## What is deployed

`MySoNativeBridge` supports only native MYSO, token id 0, between protocol chain
IDs 2 (MySo) and 12 (Base Sepolia). The EVM execution chain ID must be 84532.
The registry points at the existing adapter; all balances and approvals use the
underlying ERC20. MySo -> EVM claims mint through the adapter. EVM -> MySo deposits
transfer approved tokens into the proxy, burn them through `burnFrom`, and emit
standard bridge events. Amounts must convert exactly from 18 to 9 decimals.

Both V1 and V2 transfer messages are supported. Committee signatures, pause,
nonces/replay protection, and the daily mint limiter are enforced. The limit
also applies to mature V2 messages. This conservative policy differs from the
vault bridge's mature-message exemption. New token routes are rejected.

Four implementation contracts are deployed, followed by one atomic deployment
transaction creating and initializing the committee/config/limiter/bridge proxies
and compatibility vault. This avoids a front-running window for the existing
public `initializeConfig` method. The vault remains empty for native MYSO;
its address is retained for the bridge node's discovery interface.

This is a **fresh deployment**, not an upgrade implementation for historical
`mys`, `myso`, or `sui` proxies. Those may have different ABI names and storage
layouts. A rename does not migrate their committee membership.

## Prepare and simulate (no transactions)

Run from the repository root, with `python3`, `myso-bridge`, `cast`, `forge`, and
the existing Foundry dependencies available:

```bash
./scripts/testnet-bridge-bootstrap.sh plan \
  --key-dir "$PWD/genesis" \
  --proxy 0x538f62e8E903DcaAAB6eB2cD1c5b9A8742e66CB1

./scripts/testnet-bridge-bootstrap.sh simulate \
  --sender 0x9d4ef83ec7425d9b264794abf476c32dd57418f4

./scripts/testnet-bridge-bootstrap.sh configure --preview
```

Defaults are public RPC endpoints; override them with `MYSO_RPC_URL` and
`ETH_RPC_URL`. Endpoints and API keys are not persisted in the public plan or
config templates. The default mint limit is **$10,000/day**, using the native
MYSO price in the live MySo bridge treasury. Override `--daily-limit-usd` on every
command when selecting another limit. There is no token swap or new ERC20.

The output directory defaults to `network.config/testnet-bridge`. `plan.json`
records the genesis fingerprint, current compressed public keys, derived EVM
addresses, voting powers, signer URLs, existing key *paths*, gas balances, token,
adapter, and adapter owner. No private key contents are included. Keys are
matched by their derived public addresses, not their filenames. Config ordinal
numbers follow existing key filenames; consult `validator-map.json` before
assigning a service to a validator.

`planHash` binds the deploy configuration, genesis, committee, source files,
and limit. Simulation writes `simulation.json`; its predicted addresses are
**not deployed contracts**. Preview templates are under `preview-configs` and
cannot be used by the node launcher. An unsigned adapter-authorization request
and, if needed, an unsigned legacy-proxy revocation request are also prepared.
Revoke the legacy proxy only after reviewing the effect on the old deployment.

## Explicit deployment, after reviewing the plan

Only `deploy` broadcasts. It requires a named encrypted Foundry account, a public
sender address, and the exact reviewed plan hash. The signing wallet must match
the sender. No private key is accepted on the script command line.

```bash
./scripts/testnet-bridge-bootstrap.sh deploy \
  --sender 0x9d4ef83ec7425d9b264794abf476c32dd57418f4 \
  --account myso-testnet-deployer \
  --approve-plan REPLACE_WITH_REVIEWED_PLAN_HASH

./scripts/testnet-bridge-bootstrap.sh configure
```

The account name above is illustrative; use your existing encrypted wallet.
`configure` checks the deployed suite and tolerates only the pending adapter
owner authorization. It writes actual node configs and unsigned owner requests.
The script never sends these owner requests itself. The adapter owner must call
`setAuthorizedBridge(newProxy, true)` on the existing adapter. After reviewing
legacy traffic, retire the old proxy with `setAuthorizedBridge(oldProxy, false)`.
The existing token's presale authorization already points at this adapter and
must not be changed again.

The deployer address and transaction nonce must still match the reviewed simulation;
otherwise deployment stops for a fresh simulation. Successful receipts and start
block are saved in `deployment.json`. A repeated
deploy is refused when a deployment or prior broadcast receipts exist. On partial
failure, preserve `broadcast/deploy_native_testnet.s.sol/84532/run-latest.json`
and reconcile transactions before continuing; never clear receipts and blindly
retry. A changed genesis, committee, source, or limit invalidates the reviewed
plan. Preserve this deployment directory across restarts. A future genesis reset
must not reuse old transfer authorizations: use new bridge authority keys and
review the new committee before deploying another suite.

## Verify and run the three nodes

```bash
./scripts/testnet-bridge-bootstrap.sh check
```

This checks registry metadata, token/adapter binding and authorization, committee
weights/blocklist, ownership, pause state, and each registered signer `/ping` URL.
A reachable ping endpoint alone does not prove that the service uses the right
key or proxy. Finish with real small transfers in both directions after rollout.

The generated configs approve **no governance proposals** by default. No token
registration transaction is needed: genesis MYSO is already present on MySo,
and token id 0 is initialized on EVM in the deployment transaction. Old event
cursors and client databases are not reused. Node 1 runs the client and pays MySo
gas using its funded authority key unless `BRIDGE_CLIENT_KEY_PATH` selects a
separate funded client key. Nodes 2 and 3 serve signatures.

For a local foreground launch of node 1, point to the existing flagged secp256k1
client key file used for Base Sepolia relayer gas (never paste its contents):

```bash
export BRIDGE_RELAYER_KEY_FILE="$PWD/genesis/eth-key.json"
./scripts/testnet-bridge-bootstrap.sh node --validator 1
```

The launcher writes a mode-0600 runtime config in a temporary directory and
removes it on normal exit. It verifies the selected authority, MySo gas balance,
and EVM relayer ETH balance before startup. Raw relayer keys may alternatively
come from `BRIDGE_RELAYER_PRIVATE_KEY` in the process environment. Avoid shell
tracing. Generated templates contain only secret placeholders.

For local service configuration changes, the generated
`railway-start-commands.json` describes the startup command corresponding to each
`validator-N.yaml`. The checked-in Dockerfile expects these templates at
`/app/bridge-config-validator-N.yaml`. These files can be reviewed and copied into
the corresponding repository config files once actual deployment receipts exist.
The script does not upload these files or secrets, change Railway settings, or
restart services. Keep the existing correct authority keys. The client config
requires a funded relayer, a fresh database path, and the receipt-confirmed EVM
start block. Preview addresses must not be installed as live config.

The custodial deposit-address API is disabled here; native MYSO uses ordinary
wallet approval and `bridgeERC20`/`bridgeERC20V2`. Automatic EVM claim submission
still uses the relayer key configured under the node's existing `deposits`
section, even with that API disabled. This follows `node.rs`'s signer loading.

Before calling the bridge fully operational, verify a small MySo -> EVM claim
and a return EVM -> MySo deposit, including finalized receipts, supply/balance
changes, and the receiving MySo coin. No real smoke-test transaction is included
in plan or simulation.

## Tests

```bash
python3 scripts/tests/testnet_bridge_test.py
bash scripts/tests/bridge-bootstrap-common-test.sh
cd bridge/evm
forge test --match-path test/MySoNativeBridge.t.sol
forge test --match-path test/MySoNativeBridgeFork.t.sol \
  --fork-url https://sepolia.base.org --fork-block-number 51816246
```

The fork suite impersonates the adapter owner only inside the local EVM fork.
It exercises the actual deployed token and adapter without broadcasting.
