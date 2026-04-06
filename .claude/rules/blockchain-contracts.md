---
globs: ["contracts/**"]
---

# Blockchain & Smart Contracts

## Contract Development

- Solidity smart contracts in `contracts/` (Hardhat project)
- Primary contract: `BiyardToken.sol`

## PaaS API Integration

- Biyard provides PaaS APIs for token and point management over blockchain
- APIs handle blockchain operations on behalf of user projects
- Design endpoints to handle async blockchain operations

## Error Handling

- Blockchain transactions can fail — implement proper retry and error handling
- Consider async processing patterns: webhooks, callbacks, polling
- Log transaction hashes and block numbers for debugging

## Token Deployment

- Support testnet and mainnet deployments
- Track deployment status per chain (pending, deployed, failed)
- Store contract addresses after successful deployment
