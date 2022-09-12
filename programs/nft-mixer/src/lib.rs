
use anchor_lang::prelude::*;
pub mod errors;
pub mod instructions;
pub mod state;
pub use errors::ErrorCode;

pub use instructions::*;
pub use state::*;


declare_id!("7MH6HxrLenAA28squJhHjc5j93zcGt3tKxsSNmVUAPVs");

#[program]
pub mod nft_spinner {
    use super::*;

    pub fn initialize_pool<'info>(ctx: Context<'_, '_, '_, 'info, InitializePool<'info>>, root: [u8; 32]) -> Result<()> {
        initialize_pool::handler(ctx, root)
    }

    pub fn initialize_user<'info>(ctx: Context<'_, '_, '_, 'info, InitializeUser<'info>>) -> Result<()> {
        initialize_user::handler(ctx)
    }

    pub fn deposit<'info>(ctx: Context<'_, '_, '_, 'info, Deposit<'info>>, proof: Vec<[u8; 32]>) -> Result<()> {
        deposit::handler(ctx, proof)
    }

    pub fn mix_n_withdraw<'info>(ctx: Context<'_, '_, '_, 'info, MixNWithdraw<'info>>) -> Result<()> {
        mix_n_withdraw::handler(ctx)
    }
}
