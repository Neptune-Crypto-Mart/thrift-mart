use soroban_sdk::{contracterror, contracttype, Address, String};

/// A pre-loved item listed for sale.
///
/// `price` is denominated in the smallest unit of `asset` (a Stellar Asset
/// Contract address, e.g. native XLM or a USDC SAC).
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Listing {
    pub id: u64,
    pub seller: Address,
    pub title: String,
    pub price: i128,
    pub asset: Address,
    pub status: ListingStatus,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ListingStatus {
    Active,
    Sold,
    Cancelled,
}

/// An escrow-protected trade between a buyer and a seller for a `Listing`.
///
/// Expected lifecycle: `Pending` -> `Funded` -> `Released` | `Refunded`,
/// with `Disputed` as a branch out of `Funded` pending arbitration.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Escrow {
    pub id: u64,
    pub listing_id: u64,
    pub buyer: Address,
    pub seller: Address,
    pub amount: i128,
    pub asset: Address,
    pub status: EscrowStatus,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EscrowStatus {
    Pending,
    Funded,
    Released,
    Refunded,
    Disputed,
}

/// Shared error type returned by the core marketplace interfaces.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum MarketplaceError {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    Unauthorized = 3,
    InvalidAmount = 4,
    InvalidFee = 5,
    ListingNotFound = 6,
    ListingNotActive = 7,
    EscrowNotFound = 8,
    EscrowNotPending = 9,
    EscrowNotFunded = 10,
}
