use anchor_lang::prelude::*;

#[account]
pub struct Pool {
    /// Collection
    pub collection: Pubkey,

    /// Number of assets under protocol
    pub balance: u16,

    /// Assets Under Protocol
    pub aup: Vec<u16>,

    /// Count of assets deposited till now
    pub count: u16,

    /// Nonce to derive the program-dervied address owning the accounts
    pub nonce: u8
}

#[account]
pub struct Asset {
    /// Collection
    pub pool: Pubkey,

    /// Asset
    pub asset_mint: Pubkey,

    /// Asset Vault holding the asset
    pub asset_vault: Pubkey,

    /// Nonce
    pub nonce: u8
}

#[account]
pub struct User {
    /// Pool
    pub pool: Pubkey,

    /// The User 
    pub owner: Pubkey,
    
    /// Number of assets the user can redeem from the mixer
    pub balance: u64,

    /// Nonce
    pub nonce: u8
}

// Burnt Finance Collection Standard Accounts
#[account]
pub struct Collection {
    pub creator: Pubkey,
    pub authority: Pubkey,
    pub mutable: bool,
    pub name: String,
    pub bump: u8,
    pub meta: String,
}

#[account]
pub struct AssetMapping {
    pub collection: Pubkey,
    pub asset: Pubkey,
    pub bump: u8,
    pub meta: String,
}