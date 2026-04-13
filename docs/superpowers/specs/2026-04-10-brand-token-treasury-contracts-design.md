# Brand Token & Treasury Contract System Design

**Date:** 2026-04-10
**Status:** Approved

## Overview

Replace all existing smart contracts with a unified system for brand token issuance, treasury management, and token exchange. Includes a test-only BUSDT token for development.

## Architecture

Four contracts:

```
BrandToken (ERC-20)  <-->  Treasury  <-->  Multisig
                                |
                             BUSDT/USDT (ERC-20)
```

## Contract 1: BrandToken (ERC-20)

### Purpose

ERC-20 brand token with scheduled monthly emission and claim-based distribution.

### Deployment Parameters

- `name`, `symbol` (immutable after deploy)
- `maxSupply` — total lifetime cap, calculated as sum of all monthly emissions
- `monthlyEmission` — initial tokens per month
- `decayRateBps` — monthly emission decrease in basis points (e.g., 500 = 5%)
- `treasury` — address of the Treasury contract
- `claimSigner` — server address that signs claim authorizations

### Derived State

- `currentMonth` — months elapsed since deploy (epoch-based, 30-day periods)
- `monthlyMinted[month]` — tracks minted amount per month
- `monthlyCeiling(month)` — computed: `monthlyEmission * (1 - decayRate)^month`
- `claimedNonces[nonce]` — prevents double-claim

### Distribution Slots (DAO-mutable)

Array of `(address wallet, uint16 bps)` pairs. Example:

- Wallet A: 1000 bps (10%)
- Wallet B: 1000 bps (10%)
- Remaining (80%): stays in contract as claim pool

Sum of all slot bps must be < 10000 (100%). Remainder = claim pool.

### Functions

#### `triggerMonthlyMint()`

- Callable by owner (Multisig) only, once per month (checks `monthlyMinted[currentMonth] == 0`)
- Calculates `ceiling = monthlyCeiling(currentMonth)`
- For each distribution slot: mint `ceiling * slot.bps / 10000` to `slot.wallet`
- Mint remainder to `address(this)` (claim pool)
- Total minted for month must not exceed ceiling
- Total cumulative minted must not exceed `maxSupply`

#### `claim(uint256 amount, uint256 nonce, uint256 deadline, bytes signature)`

- User calls this to convert Points to Tokens
- Verifies EIP-712 signature from `claimSigner`
- Checks nonce not used, deadline not passed
- Transfers `amount` from contract balance (claim pool) to `msg.sender`
- Reverts if claim pool (contract balance) insufficient
- Does NOT mint new tokens; transfers from pre-minted pool

#### DAO-mutable (via Multisig):

- `setMonthlyEmission(uint256)` — changes future month emissions
- `setDecayRateBps(uint16)` — changes future decay rate
- `setDistributionSlots((address, uint16)[])` — replaces slot array
- `setClaimSigner(address)` — rotates server signing key

### Invariants

- `totalSupply() <= maxSupply` always
- `monthlyMinted[m] <= monthlyCeiling(m)` for each month
- Distribution slot bps sum < 10000

---

## Contract 2: Treasury

### Purpose

Holds USDT (or BUSDT) deposits. Provides token price calculation and buyback mechanism.

### Deployment Parameters

- `stableToken` — address of the single accepted ERC-20 (USDT or BUSDT)
- `brandToken` — address of BrandToken
- `multisig` — address of Multisig contract

### Price Formula

```
circulatingSupply = brandToken.totalSupply()
                  - brandToken.balanceOf(address(brandToken))   // claim pool
                  - brandToken.balanceOf(address(treasury))     // buyback 보유분
price = stableToken.balanceOf(address(treasury)) / circulatingSupply
```

Treasury holds both USDT and potentially bought-back BrandTokens.
Bought-back tokens reduce circulating supply, increasing price.

### Functions

#### `deposit(uint256 amount)`

- Anyone can deposit `stableToken` into Treasury
- `transferFrom(msg.sender, address(this), amount)`

#### `buyback(uint256 tokenAmount)`

- User sells BrandTokens at current floor price
- Calculates `usdtOut = tokenAmount * price()`
- Transfers `tokenAmount` BrandToken from user to Treasury (NOT burned, held)
- Transfers `usdtOut` USDT from Treasury to user
- Reverts if Treasury has insufficient USDT

#### `getPrice() view returns (uint256)`

- Returns current price with 18 decimal precision
- Returns 0 if circulating supply is 0

#### Multisig-only:

- `withdrawStable(address to, uint256 amount)` — withdraw USDT
- `withdrawToken(address token, address to, uint256 amount)` — withdraw any ERC-20 (including held-back BrandTokens)

### Invariants

- Only `stableToken` accepted via `deposit()`
- Buyback price is always based on current reserves / circulating supply
- No token burns; bought-back tokens stay in Treasury

---

## Contract 3: Multisig

### Purpose

N-of-M signature-based governance. Replaces full DAO voting for initial phase. No fixed signer limit.

### Deployment Parameters

- `signers[]` — initial array of authorized addresses
- `threshold` — minimum signatures required to execute

### Proposal Flow

1. Any signer calls `propose(target, data, value)` — creates a proposal
2. Other signers call `approve(proposalId)` — adds their signature
3. Once `approvals >= threshold`, anyone calls `execute(proposalId)`
4. Proposal is marked executed, cannot re-execute

### Functions

- `propose(address target, bytes calldata data, uint256 value) returns (uint256 proposalId)`
- `approve(uint256 proposalId)`
- `execute(uint256 proposalId)` — calls `target.call{value}(data)`
- `addSigner(address)` — only via self-call (proposed + executed)
- `removeSigner(address)` — only via self-call
- `setThreshold(uint256)` — only via self-call

### Constraints

- `threshold <= signers.length`
- `threshold >= 1`
- Cannot remove signer if it would make `signers.length < threshold`

---

## Contract 4: BUSDT (Test Token)

### Purpose

Mock USDT for testnet/local development. Unlimited minting faucet.

### Based on existing `MockUSDT.sol`:

- 6 decimals (matches real USDT)
- `mint(address to, uint256 amount)` — callable by anyone (faucet)
- `name: "Biyard USD Tether"`, `symbol: "BUSDT"`

---

## Deployment Sequence

1. Deploy **BUSDT** (testnet only)
2. Deploy **Multisig** with initial signers + threshold
3. Deploy **BrandToken** with emission params, treasury=placeholder
4. Deploy **Treasury** with stableToken=BUSDT, brandToken, multisig
5. Update BrandToken's treasury address (one-time setter or constructor param)
6. Transfer BrandToken ownership to Multisig
7. Mint initial BUSDT for testing via faucet

## Test Scenario

1. Mint 1M BUSDT via faucet
2. Deposit 100K BUSDT into Treasury
3. `triggerMonthlyMint()` — month 1 tokens issued
4. Check `getPrice()` — should be `100K / circulatingSupply`
5. User claims tokens via signed message
6. User does buyback — sells tokens at floor price
7. Multisig proposes + executes BUSDT withdrawal

## File Structure

```
contracts/
  src/
    BrandToken.sol
    Treasury.sol
    Multisig.sol
    BUSDT.sol
  test/
    BrandToken.test.ts
    Treasury.test.ts
    Multisig.test.ts
    Integration.test.ts
  scripts/
    deploy.ts
    deployTestnet.ts
```

## Open Decisions

None — all design questions resolved in conversation.
