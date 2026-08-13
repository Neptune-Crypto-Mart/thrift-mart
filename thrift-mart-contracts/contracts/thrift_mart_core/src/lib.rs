#![no_std]

mod admin;
mod escrow;
mod listing;
mod types;

pub use admin::AdminInterface;
pub use escrow::EscrowInterface;
pub use listing::ListingsInterface;
pub use types::{Escrow, EscrowStatus, Listing, ListingStatus, MarketplaceError};

use soroban_sdk::contract;

/// The core Thrift Mart contract.
///
/// This crate defines the shared types ([`Listing`], [`Escrow`], ...) and
/// the [`AdminInterface`], [`ListingsInterface`], and [`EscrowInterface`]
/// traits that make up the marketplace's public surface. Contributors
/// implement each trait for `Contract` with `#[contractimpl]`, e.g.:
///
/// ```ignore
/// use soroban_sdk::contractimpl;
///
/// #[contractimpl]
/// impl ListingsInterface for Contract {
///     fn create_listing(
///         env: Env,
///         seller: Address,
///         title: String,
///         price: i128,
///         asset: Address,
///     ) -> Result<u64, MarketplaceError> {
///         todo!()
///     }
///     // ...
/// }
/// ```
#[contract]
pub struct Contract;
