use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializePool<'info> {
    pub collection: Account<'info, Collection>,

    #[account(mut)]
    pub initializer: Signer<'info>,

    #[account(
        init, 
        space=10000, 
        payer=initializer
    )]
    pub pool: Box<Account<'info, Pool>>,

    /// CHECK: TODO
    #[account(
        seeds = [
            pool.to_account_info().key.as_ref(),
        ],
        bump,
    )]
    pub pool_signer: UncheckedAccount<'info>,

    // misc
    pub system_program: Program<'info, System>,
}

pub fn handler<'info>(
    ctx: Context<'_, '_, '_, 'info, InitializePool<'info>>,
) -> Result<()> {
    let pool  = &mut ctx.accounts.pool;
    pool.collection = ctx.accounts.collection.key();
    pool.aup = Vec::new();
    pool.balance = 0;
    pool.count = 0;
    pool.nonce = *ctx.bumps.get("pool_signer").unwrap();

    Ok(())
}