use soroban_sdk::{Address, Env};

use crate::types::MarketplaceError;

/// Contract initialization and administrative configuration.
///
/// Implementers are responsible for persisting the admin address and fee,
/// and for guarding `set_fee` behind `admin.require_auth()`.
pub trait AdminInterface {
    /// One-time setup. Sets the contract `admin` and the marketplace fee
    /// (in basis points, e.g. `250` = 2.5%) charged on released escrows.
    /// Must fail with `AlreadyInitialized` if called more than once.
    fn initialize(env: Env, admin: Address, fee_bps: u32) -> Result<(), MarketplaceError>;

    /// Returns the current admin address. Fails with `NotInitialized` if
    /// `initialize` has not been called yet.
    fn admin(env: Env) -> Result<Address, MarketplaceError>;

    /// Updates the marketplace fee. Requires admin authorization.
    fn set_fee(env: Env, fee_bps: u32) -> Result<(), MarketplaceError>;
}
