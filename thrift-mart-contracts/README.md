# thrift-mart-contracts

Soroban smart contracts for **Thrift Mart** — a crypto-powered marketplace on
Stellar (XLM/USDC) for selling pre-loved items at affordable prices, with
escrow-protected trades.

## Project structure

```text
.
├── contracts
│   └── thrift_mart_core
│       ├── src
│       │   ├── lib.rs       # crate root, `Contract` struct
│       │   ├── types.rs     # Listing, Escrow, MarketplaceError
│       │   ├── admin.rs     # AdminInterface
│       │   ├── listing.rs   # ListingsInterface
│       │   └── escrow.rs    # EscrowInterface
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

- New contracts go in `contracts/`, each in its own crate. Contract crates
  rely on the top-level `Cargo.toml` workspace for shared dependencies
  (`soroban-sdk`).
- Frontend and backend live in the sibling
  [`thrift-mart-web`](../thrift-mart-web) and
  [`thrift-mart-api`](../thrift-mart-api) directories.

## `thrift_mart_core`

Defines the marketplace's core data types and the trait interfaces
subsequent contributors implement for `Contract` (via `#[contractimpl]`):

- **`AdminInterface`** — one-time `initialize`, and admin-gated fee
  configuration.
- **`ListingsInterface`** — create, fetch, and cancel `Listing`s.
- **`EscrowInterface`** — open, fund, release, and refund an `Escrow`
  through its `Pending -> Funded -> Released | Refunded` lifecycle.

Each trait method's authorization and failure-mode expectations are
documented on the method itself in `src/`. Implement a trait like this:

```rust
use soroban_sdk::contractimpl;

#[contractimpl]
impl ListingsInterface for Contract {
    fn create_listing(
        env: Env,
        seller: Address,
        title: String,
        price: i128,
        asset: Address,
    ) -> Result<u64, MarketplaceError> {
        // ...
    }
    // ...
}
```

## Building and testing

```bash
cd contracts/thrift_mart_core
make build   # stellar contract build
make test    # cargo test
```

Requires the [Stellar CLI](https://developers.stellar.org/docs/tools/stellar-cli)
(`stellar`) and the `wasm32v1-none` Rust target.
