
use anchor_lang::prelude::*;
pub mod errors;
pub mod instructions;
pub mod state;
pub use errors::ErrorCode;

pub use instructions::*;
pub use state::*;

declare_id!("7MH6HxrLenAA28squJhHjc5j93zcGt3tKxsSNmVUAPVs");

#[program]
pub mod nft_mixer {
    use super::*;

    pub fn initialize_pool<'info>(ctx: Context<'_, '_, '_, 'info, InitializePool<'info>>) -> Result<()> {
        initialize_pool::handler(ctx)
    }

    pub fn initialize_user<'info>(ctx: Context<'_, '_, '_, 'info, InitializeUser<'info>>) -> Result<()> {
        initialize_user::handler(ctx)
    }

    pub fn deposit<'info>(ctx: Context<'_, '_, '_, 'info, Deposit<'info>>) -> Result<()> {
        deposit::handler(ctx)
    }

    pub fn mix_n_withdraw<'info>(ctx: Context<'_, '_, '_, 'info, MixNWithdraw<'info>>) -> Result<()> {
        mix_n_withdraw::handler(ctx)
    }
}
