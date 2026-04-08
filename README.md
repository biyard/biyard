# Biyard

Biyard is a Launchpad-style SaaS/PaaS for issuing and managing blockchain
points and tokens. UI label: **Brand**. Internal/code term: **Project**.

## Layout

- `console/` — Dioxus 0.7 fullstack app (UI + server handlers)
- `landing/` — Dioxus 0.7 fullstack marketing site
- `contracts/` — Solidity smart contracts (Hardhat)
- `cdk/` — AWS CDK infrastructure (TypeScript)
- `playwright/` — E2E test suite

## Quick start

```bash
# Local infra: LocalStack (DynamoDB/S3/SQS) + DynamoDB Admin UI
docker compose up -d

# Console dev server
cd console && DYNAMO_TABLE_PREFIX=biyard-dev dx serve --port 8000 --web

# Landing dev server
cd landing && dx serve --port 8001 --web
```

## For contributors / AI agents

- [`CLAUDE.md`](CLAUDE.md) — repo conventions, tech stack, constraints
- [`AGENT.md`](AGENT.md) — terminology and tenancy rules
- [`.claude/rules/`](.claude/rules/) — scoped conventions (auto-loaded by glob)
