use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    pub pool: Box<Account<'info, Pool>>,

    #[
        account(
            init, 
            space = 150,
            payer = owner,
            seeds = [
                owner.key.as_ref(),
                pool.to_account_info().key().as_ref(),
            ],
            bump
        )
    ]
    pub user: Box<Account<'info, User>>,

    pub system_program: Program<'info, System>
}

pub fn handler<'info>(
    ctx: Context<'_, '_, '_, 'info, InitializeUser<'info>>,
) -> Result<()> {
    let user = &mut ctx.accounts.user;

    user.pool = ctx.accounts.pool.key();
    user.owner = ctx.accounts.owner.key();
    user.balance = 0;
    user.nonce = *ctx.bumps.get("user").unwrap();

    Ok(())
}