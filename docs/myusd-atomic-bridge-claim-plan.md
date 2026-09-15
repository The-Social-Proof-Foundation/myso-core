# Atomic bridge claim into MyUSD

## Goal

When an approved EVM-to-MySo bridge message carries USDC or USDT, the bridge
must atomically turn that exact amount into MyUSD. The recipient receives
MyUSD, while the bridged stable becomes a reserve held by `MyUsdPeg`. A
recipient must not be able to bypass that conversion and claim bridged USDC or
USDT directly.

This is deliberately an on-chain flow. The bridge node only relays a
transaction; it never takes custody of a user's incoming stablecoin.

## Why the current flow cannot do this

`bridge::claim_and_transfer_token<T>` calls `claim_token_internal<T>` and
immediately transfers the minted `Coin<T>` to the bridge message recipient.
`bridge::claim_token<T>` returns the coin, but only when the transaction sender
is that recipient. Therefore a bridge node cannot safely intercept either
function and place the coin in a peg in a later transaction.

The fix is one programmable transaction that approves the message, claims the
stable, deposits it into the peg, mints MyUSD, and transfers MyUSD to the
address embedded in the bridge message.

## Scope and model

- Initial inbound rails: bridged USDC and bridged USDT only.
- Both current bridge tokens and MyUSD use six decimals, so one atomic unit of
  an enabled stable mints one atomic unit of MyUSD. Do not silently generalize
  this rule to a future token with different decimals.
- `MyUsdPeg` is the sole holder of `TreasuryCap<MYUSD>`. No other module or
  operational wallet may mint MyUSD.
- Each rail has an independent reserve balance and enabled/paused state. MyUSD
  is backed by the aggregate of enabled stable reserves, while redemption must
  use a specific rail with sufficient reserve.

## On-chain design

### 1. Put the atomic adapter in the Bridge package

Create `bridge::myusd_peg` in the Bridge package (or move the peg module there)
so it can call a `public(package)` version of
`bridge::claim_token_internal<T>`. Do not expose that internal function as
`public`: a public claim-to-coin API callable by the relay would let the relay
or any caller take a recipient's claim.

`MyUsdPeg` contains the MyUSD treasury cap, `Balance<USDC>`, `Balance<USDT>`,
rail configuration, accounting totals, and a pause flag. With only two fixed
rails, use typed balance fields rather than a generic `Bag`; that makes the
supported collateral types clear in Move and simpler to audit.

### 2. Add an explicit bridge claim policy per token route

Extend `BridgeInner` with policy for token IDs:

- `DIRECT_CLAIM` for existing assets such as BTC and ETH.
- `CONVERT_TO_MYUSD` for the USDC and USDT routes.
- `DISABLED` for a paused or retired rail.

The bridge's governance/system-admin path alone may change this policy. Record
an effective source-chain/sequence boundary when enabling conversion so bridge
messages created before the migration retain their original direct-claim
behavior. Do not retroactively convert a user's already pending transfer.

For `CONVERT_TO_MYUSD`, make both existing generic entry points,
`claim_token<T>` and `claim_and_transfer_token<T>`, abort before minting the
bridged token. They remain unchanged for `DIRECT_CLAIM` tokens.

### 3. Add the one allowed conversion entry point

Add a function shaped like:

```move
public fun claim_stable_into_myusd<T>(
    bridge: &mut Bridge,
    peg: &mut MyUsdPeg,
    clock: &Clock,
    source_chain: u8,
    bridge_seq_num: u64,
    ctx: &mut TxContext,
)
```

It must, in this order:

1. Verify `T` is USDC or USDT, the bridge policy for this message is
   `CONVERT_TO_MYUSD`, and the corresponding peg rail is enabled.
2. Call the package-private bridge claim helper, which verifies the signed
   message, token type, target chain, limiter, and replay state and obtains
   `(Coin<T>, recipient)`.
3. If the claim helper returns `none` because it was already claimed or is
   temporarily limiter-blocked, perform no conversion and emit no mint event.
   The relay treats this as retry/complete based on the bridge record.
4. Deposit the entire `Coin<T>` into the matching peg reserve.
5. Mint exactly the same integer amount of `Coin<MYUSD>` from the peg's
   treasury cap and transfer it to `recipient` from the signed bridge message.
6. Emit a `StableClaimConvertedToMyUsd` event containing bridge message key,
   source chain and sequence, rail/token ID, input amount, MyUSD amount,
   recipient, peg ID, and post-operation reserve/issued totals.

The whole transaction must abort if any step fails. That leaves the bridge
message unclaimed, the stable unminted, and MyUSD unissued, allowing a safe
retry. The caller never supplies the final recipient.

### 4. Preserve redeemability and accounting invariants

Implement the complementary peg operations before enabling inbound conversion:

- `redeem_myusd_to_rail<USDC|USDT>` burns the supplied MyUSD and withdraws the
  same amount from the chosen rail only when that reserve has enough balance.
- Rail pause blocks new conversion and redemption for that rail; global pause
  blocks both.
- Maintain and expose `reserve`, `issued`, and `redeemed` per rail. Assert
  `myusd_total_supply == usdc_reserve + usdt_reserve` in test-only invariant
  checks, accounting for coins in the transaction before the final transfer.
- Reject zero amounts, unsupported types, a decimal mismatch, and conversion
  when MyUSD or the rail is frozen/deny-listed. These failures must be atomic.

Do not add an admin withdrawal from a collateral reserve. If a reserve is ever
moved, it needs a separately governed, fully collateralized migration.

## Bridge-node and transaction-builder changes

1. Add MyUSD package ID, peg object ID, rail token IDs, and policy state to the
   bridge node configuration. Validate all configured object/type IDs at
   startup.
2. In `myso_transaction_builder.rs`, retain approval construction, but replace
   the final `bridge::claim_and_transfer_token<T>` call with
   `bridge::myusd_peg::claim_stable_into_myusd<T>` only for USDC and USDT
   messages whose policy is active. BTC/ETH/MYSO keep their present calls.
3. Pass the shared bridge, shared peg, clock, source chain, and sequence into
   that call. The PTB has no user-controlled recipient argument.
4. Index the conversion event and bridge claim event together, keyed by
   `(source_chain, bridge_seq_num)`, and make relay retries idempotent. A failed
   transaction is retried; a successful conversion is never re-submitted.
5. The UI should show only MyUSD after a completed USDC/USDT inbound bridge.
   It may show the selected outbound rail and current reserve availability, but
   the contract remains the final authority on availability.

## Migration and rollout

1. Publish the peg and conversion adapter, create and share one `MyUsdPeg`, and
   place the MyUSD treasury cap inside it.
2. Register USDC and USDT rails disabled. Test direct claims are still accepted
   while conversion policy is off.
3. Add the builder/node behavior and event/indexer/GraphQL decoding. Run a
   localnet and testnet rehearsal with separate stable claims, no localnet ↔
   testnet fallback.
4. Enable one rail at a defined message sequence boundary, then the other;
   direct claim must abort for new messages on each enabled rail.
5. Monitor reserve, MyUSD supply, claim status, conversion failures, and
   limiter stalls. Keep an emergency rail pause, never a bypass that sends
   incoming stable to the relay or an admin wallet.

## Required tests

- USDC and USDT conversion each issue exactly one MyUSD unit per input unit and
  retain the stable in the correct reserve.
- A direct USDC/USDT claim fails once its conversion boundary is active; BTC,
  ETH, and MYSO direct claims still work.
- A mismatched token type, disabled rail, bad recipient, limiter rejection,
  duplicate claim, and denied/frozen transfer leave all balances and claim
  status unchanged.
- A conversion event's recipient is the message recipient, never the relay
  signer.
- Redemption burns MyUSD and releases only the selected funded rail; an
  underfunded rail aborts without burning.
- PTB integration tests prove approve + claim + reserve deposit + MyUSD mint is
  one transaction and a failure rolls back the bridge record's `claimed` flag.
