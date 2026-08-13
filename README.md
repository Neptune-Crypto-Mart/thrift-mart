# thrift-mart

A crypto-powered marketplace on Stellar (XLM/USDC) for selling pre-loved
items at affordable prices. Fast, low-fee transactions, escrow protection,
and global reach. Declutter smart, buy sustainably, pay with digital assets.
Sustainable thrifting meets blockchain efficiency.

## Monorepo structure

This repo is split into three independent packages:

| Package                                       | What it is                              | Stack                              |
| ---------------------------------------------- | ---------------------------------------- | ----------------------------------- |
| [`thrift-mart-web`](./thrift-mart-web)         | Web frontend                             | Next.js, React, TypeScript, Tailwind |
| [`thrift-mart-api`](./thrift-mart-api)         | Backend API                              | Express 5, TypeScript (NodeNext/ESM) |
| [`thrift-mart-contracts`](./thrift-mart-contracts) | Soroban smart contracts (escrow, listings) | Rust, soroban-sdk                   |

Each package has its own `README.md` with setup instructions, scripts, and
directory layout — start there when working within a package.

## Getting started

Clone the repo, then set up whichever package(s) you're working on:

```bash
# Web
cd thrift-mart-web && npm install && npm run dev

# API
cd thrift-mart-api && npm install && cp .env.example .env && npm run dev

# Contracts
cd thrift-mart-contracts/contracts/thrift_mart_core && make build
```

## How the pieces fit together

- **`thrift-mart-contracts`** holds the on-chain source of truth: listings
  and escrow-protected trades, settled in XLM/USDC on Stellar. The core
  crate (`thrift_mart_core`) defines the shared types and trait interfaces
  (`AdminInterface`, `ListingsInterface`, `EscrowInterface`) that contract
  implementations build on.
- **`thrift-mart-api`** is the backend service that fronts the contracts and
  any off-chain concerns (search, notifications, sessions, etc.) for
  clients.
- **`thrift-mart-web`** is the marketplace UI end users browse, list items,
  and trade through.

## Contributing

- Keep changes scoped to one package per PR where possible.
- Follow the conventions already established in each package (see its
  README) rather than introducing new tooling.
- Contract work should implement or extend the interfaces defined in
  `thrift-mart-contracts/contracts/thrift_mart_core`.
