# Biyard Smart Contracts

This directory contains the Solidity smart contracts for the Biyard platform, including the BiyardToken (ERC20) and DAOTreasury governance system.

## Overview

### BiyardToken

An ERC20 token with extended functionality:
- **Minting**: Controlled minting with role-based access
- **Burning**: Users can burn their own tokens
- **Pausable**: Owner can pause/unpause transfers
- **Max Supply**: Optional supply cap to prevent unlimited minting

### DAOTreasury

A decentralized autonomous organization treasury contract with proposal-based governance:
- **Proposal System**: Token holders can create proposals for fund transfers
- **Voting Mechanism**: Token-weighted voting on proposals
- **Quorum Requirements**: Configurable quorum percentage
- **Multi-Asset Support**: Supports both native tokens (ETH/MATIC) and ERC20 tokens
- **Time-Based Voting**: Configurable voting periods
- **Proposal Lifecycle**: Complete proposal state management (Pending, Active, Succeeded, Defeated, Executed, Cancelled)

## Project Structure

```
contracts/
├── contracts/
│   ├── BiyardToken.sol       # ERC20 token implementation
│   └── DAOTreasury.sol        # DAO treasury with governance
├── test/
│   ├── BiyardToken.test.ts    # Token contract tests
│   └── DAOTreasury.test.ts    # Treasury contract tests
├── scripts/
│   └── deploy.ts              # Deployment script
├── hardhat.config.ts          # Hardhat configuration
├── package.json               # Dependencies
└── README.md                  # This file
```

## Prerequisites

- Node.js 18+
- pnpm 10+

## Installation

From the root of the monorepo:

```bash
pnpm install
```

Or from the contracts directory:

```bash
cd contracts
pnpm install
```

## Development Commands

### Compile Contracts

```bash
pnpm --filter @biyard/contracts compile
```

### Run Tests

```bash
pnpm --filter @biyard/contracts test
```

All tests should pass:
- BiyardToken: 20 tests
- DAOTreasury: 31 tests
- **Total: 51 tests passing**

### Run Local Node

```bash
pnpm --filter @biyard/contracts node
```

### Deploy to Local Network

In one terminal, start the local node:
```bash
pnpm --filter @biyard/contracts node
```

In another terminal, deploy:
```bash
pnpm --filter @biyard/contracts deploy:local
```

## Contract Details

### BiyardToken

**Constructor Parameters:**
- `name`: Token name (e.g., "Biyard Token")
- `symbol`: Token symbol (e.g., "BIYARD")
- `initialSupply`: Initial token supply to mint to deployer
- `maxSupply`: Maximum supply cap (0 for unlimited)

**Key Functions:**
- `mint(address to, uint256 amount)`: Mint new tokens (minter only)
- `burn(uint256 amount)`: Burn your own tokens
- `addMinter(address account)`: Grant minter role (owner only)
- `removeMinter(address account)`: Revoke minter role (owner only)
- `pause()`: Pause token transfers (owner only)
- `unpause()`: Resume token transfers (owner only)

### DAOTreasury

**Constructor Parameters:**
- `_governanceToken`: Address of the ERC20 token used for voting
- `_proposalThreshold`: Minimum tokens needed to create a proposal
- `_votingPeriod`: Voting period in seconds
- `_quorumPercentage`: Percentage of total supply needed for quorum (1-100)

**Key Functions:**

**Treasury Management:**
- `depositTokens(address token, uint256 amount)`: Deposit ERC20 tokens
- `receive()`: Accept native token deposits (ETH/MATIC)
- `getTreasuryBalance(address tokenAddress)`: Check balance

**Proposal Management:**
- `createProposal(string description, address recipient, uint256 amount, address tokenAddress)`: Create new proposal
- `castVote(uint256 proposalId, bool support)`: Vote on a proposal
- `executeProposal(uint256 proposalId)`: Execute a passed proposal
- `cancelProposal(uint256 proposalId)`: Cancel a proposal (proposer or owner only)

**View Functions:**
- `getProposal(uint256 proposalId)`: Get proposal details
- `getProposalState(uint256 proposalId)`: Get proposal state
- `hasVoted(uint256 proposalId, address voter)`: Check if address has voted

**Governance Configuration (Owner Only):**
- `updateProposalThreshold(uint256)`: Update minimum tokens for proposal creation
- `updateVotingPeriod(uint256)`: Update voting period
- `updateQuorumPercentage(uint256)`: Update quorum requirement

## Deployment Configuration

The default deployment script (`scripts/deploy.ts`) deploys:

1. **BiyardToken**
   - Name: "Biyard Token"
   - Symbol: "BIYARD"
   - Initial Supply: 1,000,000 BIYARD
   - Max Supply: 10,000,000 BIYARD

2. **DAOTreasury**
   - Governance Token: BiyardToken
   - Proposal Threshold: 100 BIYARD
   - Voting Period: 3 days
   - Quorum: 10%

The script also transfers 100,000 BIYARD tokens to the treasury for testing.

## Usage Example

### Creating and Executing a Proposal

```typescript
// 1. User creates a proposal (must have >= 100 BIYARD)
await treasury.createProposal(
  "Fund development team",
  recipientAddress,
  ethers.parseEther("1000"),
  tokenAddress
);

// 2. Token holders vote
await treasury.connect(voter1).castVote(proposalId, true);  // Vote yes
await treasury.connect(voter2).castVote(proposalId, false); // Vote no

// 3. Wait for voting period to end (3 days)

// 4. Execute the proposal
await treasury.executeProposal(proposalId);
```

## Security Features

- **Access Control**: Role-based permissions using OpenZeppelin's Ownable
- **Reentrancy Protection**: SafeERC20 and ReentrancyGuard
- **Pausable Transfers**: Emergency pause mechanism for token
- **Supply Cap**: Optional maximum supply to prevent inflation
- **Voting Integrity**: One vote per address, voting power based on token balance at vote time
- **Proposal Validation**: Checks for sufficient treasury balance before proposal creation

## Testing

The test suite includes comprehensive coverage:

**BiyardToken Tests:**
- Deployment and initialization
- Minting permissions and limits
- Minter role management
- Token burning
- Pausable functionality
- Token transfers

**DAOTreasury Tests:**
- Deployment and configuration
- Native and ERC20 token deposits
- Proposal creation and validation
- Voting mechanism
- Proposal execution
- Proposal cancellation
- Governance parameter updates
- View functions

Run tests with:
```bash
pnpm --filter @biyard/contracts test
```

## License

MIT

## Integration with Biyard Platform

These contracts are designed to integrate with the Biyard platform's backend APIs:
- The token can be minted through platform APIs when users earn points
- The DAO treasury manages community funds and governance
- Proposals can be created through the platform UI and voted on by token holders
- The backend can listen to contract events to update the database
