use soroban_sdk::{Address, Env, String};

use crate::types::{Listing, MarketplaceError};

/// Creating and managing marketplace listings.
///
/// Implementers own persistence of `Listing` records (instance or
/// persistent storage), id assignment, and authorization — `create_listing`
/// and `cancel_listing` must require the `seller`'s authorization.
pub trait ListingsInterface {
    /// Creates a new `Active` listing for `seller` and returns its id.
    /// Requires `seller` authorization.
    fn create_listing(
        env: Env,
        seller: Address,
        title: String,
        price: i128,
        asset: Address,
    ) -> Result<u64, MarketplaceError>;

    /// Fetches a listing by id. Fails with `ListingNotFound` if it doesn't
    /// exist.
    fn get_listing(env: Env, listing_id: u64) -> Result<Listing, MarketplaceError>;

    /// Cancels an `Active` listing. Requires the seller's authorization.
    /// Fails with `ListingNotActive` if the listing is `Sold` or already
    /// `Cancelled`.
    fn cancel_listing(env: Env, listing_id: u64) -> Result<(), MarketplaceError>;
}
