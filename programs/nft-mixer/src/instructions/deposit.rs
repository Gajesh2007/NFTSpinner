use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::*;

const PREFIX: &str = "collections";

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub pool: Account<'info, Pool>,

    #[account(
        mut, 
        constraint = pool_vault.owner == pool_signer.key(),
    )]
    pub pool_vault: Account<'info, TokenAccount>,

    /// CHECK: TODO
    #[account(
        seeds = [
            pool.to_account_info().key().as_ref()
        ],
        bump = pool.nonce
    )]
    pub pool_signer: UncheckedAccount<'info>,

    #[account(
        mut,
        has_one = owner,
        has_one = pool,
        seeds = [
                user.key().as_ref(),
                pool.to_account_info().key().as_ref(),
        ],
        bump = user.nonce,
    )]
    pub user: Box<Account<'info, User>>,

    #[account(
        constraint = asset_user.owner == owner.key()
    )]
    pub asset_user: Box<Account<'info, TokenAccount>>,

    #[account(
        constraint = pool.collection == asset_mapping.collection,
        constraint = asset_user.mint == asset_mapping.asset,
        seeds = [PREFIX.as_bytes(), pool.collection.as_ref(), asset_user.mint.as_ref()],
        bump = asset_mapping.bump
    )]
    pub asset_mapping: Account<'info, AssetMapping>,

    #[account(
        init, 
        space = 150,
        payer = owner, 
        seeds = [pool.count.to_string().as_ref(), pool.key().as_ref()],
        bump,
    )]
    pub asset: Box<Account<'info, Asset>>,

    #[account(mut)]
    pub owner: Signer<'info>,

    // misc
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}

pub fn handler<'info>(
    ctx: Context<'_, '_, '_, 'info, Deposit<'info>>,
) -> Result<()> {
    let user = &mut ctx.accounts.user;
    let pool = &mut ctx.accounts.pool;
    let asset = &mut ctx.accounts.asset;
    
    // Transfer NFT into the Mixer vault.
    {
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.asset_user.to_account_info(),
                to: ctx.accounts.pool_vault.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(), 
            },
        );
        transfer(cpi_ctx, 1)?;
    }
    
    // Pool Data Update
    pool.balance = pool.balance + 1;
    let count = pool.count;
    pool.aup.push(count);
    pool.count = pool.count + 1;

    // User Data Update
    user.balance = user.balance + 1;
    
    // Asset Data Update
    asset.asset_mint = ctx.accounts.asset_user.mint;
    asset.asset_vault = ctx.accounts.asset_user.key();
    asset.pool = pool.key();
    asset.nonce = *ctx.bumps.get("asset").unwrap();

    Ok(())
}