use soroban_sdk::{Address, Env};

use crate::types::{Escrow, MarketplaceError};

/// Escrow-protected trades between a buyer and a seller.
///
/// Implementers move funds through the `Escrow` lifecycle
/// (`Pending` -> `Funded` -> `Released` | `Refunded`, with `Disputed` as a
/// branch out of `Funded`) using a token client (e.g. `token::Client`) for
/// the escrow's `asset`, and must enforce the authorization noted per
/// method.
pub trait EscrowInterface {
    /// Opens a `Pending` escrow for `buyer` against an `Active` listing and
    /// returns its id. Requires `buyer` authorization. Fails with
    /// `ListingNotActive` if the listing is not `Active`.
    fn open_escrow(env: Env, buyer: Address, listing_id: u64) -> Result<u64, MarketplaceError>;

    /// Transfers the escrow amount of `asset` from the buyer into the
    /// contract, moving the escrow to `Funded`. Requires `buyer`
    /// authorization. Fails with `EscrowNotPending` otherwise.
    fn fund_escrow(env: Env, escrow_id: u64) -> Result<(), MarketplaceError>;

    /// Releases a `Funded` escrow's funds to the seller (minus the
    /// marketplace fee) and marks the listing `Sold`. Requires `buyer`
    /// authorization, confirming receipt of the item. Fails with
    /// `EscrowNotFunded` otherwise.
    fn release_escrow(env: Env, escrow_id: u64) -> Result<(), MarketplaceError>;

    /// Refunds a `Funded` escrow's funds back to the buyer. Requires
    /// `seller` authorization. Fails with `EscrowNotFunded` otherwise.
    fn refund_escrow(env: Env, escrow_id: u64) -> Result<(), MarketplaceError>;

    /// Fetches an escrow by id. Fails with `EscrowNotFound` if it doesn't
    /// exist.
    fn get_escrow(env: Env, escrow_id: u64) -> Result<Escrow, MarketplaceError>;
}
