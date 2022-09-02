use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::*;
use crate::errors::ErrorCode;

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
    proof: Vec<[u8; 32]>
) -> Result<()> {
    let user = &mut ctx.accounts.user;
    let pool = &mut ctx.accounts.pool;
    let asset = &mut ctx.accounts.asset;
    
    // Validation
    let leaf = anchor_lang::solana_program::keccak::hash(&ctx.accounts.asset_user.mint.to_bytes()).0;
    let is_whitelisted = verify(proof, pool.root, leaf);

    if is_whitelisted == false {
        return err!(ErrorCode::MintNotAllowedInThePool);
    }
    
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

pub fn verify(proof: Vec<[u8; 32]>, root: [u8; 32], leaf: [u8; 32]) -> bool {
    let mut computed_hash = leaf;
    for proof_element in proof.into_iter() {
        if computed_hash <= proof_element {
            // Hash(current computed hash + current element of the proof)
            computed_hash =
                anchor_lang::solana_program::keccak::hashv(&[&computed_hash, &proof_element]).0;
        } else {
            // Hash(current element of the proof + current computed hash)
            computed_hash =
                anchor_lang::solana_program::keccak::hashv(&[&proof_element, &computed_hash]).0;
        }
    }
    // Check if the computed hash (root) is equal to the provided root
    computed_hash == root
}