use crate::errors::ErrorCode;
use crate::state::*;

use anchor_lang::prelude::*;
use anchor_spl::token::*;
use anchor_lang::solana_program::{clock};
use anchor_lang::solana_program::pubkey::Pubkey;
use std::str::FromStr;

#[derive(Accounts)]
pub struct MixNWithdraw<'info> {
    #[account(mut)]
    pub pool: Account<'info, Pool>,

    /// CHECK: TODO
    #[account(
        seeds = [
            pool.to_account_info().key().as_ref()
        ],
        bump = pool.nonce
    )]
    pub pool_signer: UncheckedAccount<'info>,

    pub asset: Box<Account<'info, Asset>>,
    #[account(
        constraint = asset_vault.owner == pool_signer.key()
    )]
    pub asset_vault: Box<Account<'info, TokenAccount>>,
    #[account(
        constraint = asset_user.owner == owner.key()
    )]
    pub asset_user: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        has_one = owner,
        has_one = pool,
        seeds = [
                owner.key().as_ref(),
                pool.to_account_info().key().as_ref(),
        ],
        bump = user.nonce,
    )]
    pub user: Box<Account<'info, User>>,

    #[account(mut)]
    pub owner: Signer<'info>,

    // misc
    pub token_program: Program<'info, Token>,
}

pub fn handler<'info>(
    ctx: Context<'_, '_, '_, 'info, MixNWithdraw<'info>>,
) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let user = &mut ctx.accounts.user;
    let program_id = Pubkey::from_str("7MH6HxrLenAA28squJhHjc5j93zcGt3tKxsSNmVUAPVs").unwrap();

    if pool.balance == 0 {
        return err!(ErrorCode::EmptyPool)
    } if user.balance == 0 {
        return err!(ErrorCode::EmptyBalance)
    }

    let timestamp: u64 = clock::Clock::get().unwrap().unix_timestamp.try_into().unwrap();

    let index = timestamp % pool.balance as u64;
    
    let (_asset, _bump_seed) = Pubkey::find_program_address(&[pool.aup[index as usize].to_string().as_ref(), pool.key().as_ref()], &program_id);

    // Validation 
    if ctx.accounts.asset.key() != _asset {
        return err!(ErrorCode::AccountInvalid)
    } if ctx.accounts.asset.asset_vault != ctx.accounts.asset_vault.key() {
        return err!(ErrorCode::AccountInvalid)
    } if ctx.accounts.asset.asset_mint.key() != ctx.accounts.asset_user.mint && ctx.accounts.asset_vault.mint != ctx.accounts.asset.asset_mint {
        return err!(ErrorCode::InvalidMint);
    }

    {
        let seeds = &[pool.to_account_info().key.as_ref(), &[pool.nonce]];
        let pool_signer = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.asset_vault.to_account_info(),
                to: ctx.accounts.asset_user.to_account_info(),
                authority: ctx.accounts.pool_signer.to_account_info(),
            },
            pool_signer,
        );
        transfer(cpi_ctx, 1)?;
    }

    // Pool Data Update
    pool.balance = pool.balance - 1;
    pool.aup.remove(index as usize);
    
    // User Data Update
    user.balance = user.balance - 1;

    Ok(())
}