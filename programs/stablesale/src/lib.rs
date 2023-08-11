use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

declare_id!("CZCbBLPGeMx4GzGHjDhfthv1KwbujGeZYWfhtAtKkHdi");

pub mod states;
use crate::states::{AppState, SalePair};

#[program]
pub mod stablesale {
    use anchor_spl::token;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.app_state.owner = ctx.accounts.payer.key();
        ctx.accounts.app_state.mint = ctx.accounts.mint.key();
        ctx.accounts.app_state.mint_account = ctx.accounts.mint_account.key();
        Ok(())
    }

    pub fn init_pair(ctx: Context<InitPair>, price: u64) -> Result<()> {
        ctx.accounts.sale_pair.price = price;
        ctx.accounts.sale_pair.token = ctx.accounts.token.key();
        ctx.accounts.sale_pair.token_account = ctx.accounts.token_account.key();
        Ok(())
    }

    pub fn update_price(ctx: Context<UpdatePair>, price: u64) -> Result<()> {
        ctx.accounts.sale_pair.price = price;
        Ok(())
    }

    pub fn purchase(ctx: Context<Purchase>, amount: u64) -> Result<()> {
        let receive_cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                authority: ctx.accounts.payer.to_account_info(),
                from: ctx.accounts.from_token_account.to_account_info(),
                to: ctx.accounts.token_account.to_account_info(),
            },
        );

        token::transfer(receive_cpi_context, amount)?;

        let authority_bump = *ctx.bumps.get("vault_authority").unwrap();
        let authority_seed = [&b"vault_authority"[..], &[authority_bump]];
        let authority_seeds = [&authority_seed[..]];

        let transfer_cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                authority: ctx.accounts.vault_authority.clone(),
                from: ctx.accounts.mint_account.to_account_info(),
                to: ctx.accounts.receive_mint_account.to_account_info(),
            },
        )
        .with_signer(&authority_seeds);
        let one_mint: u64 = 10u64.pow(ctx.accounts.mint.decimals.into());
        token::transfer(
            transfer_cpi_context,
            amount * one_mint / ctx.accounts.sale_pair.price,
        )?;

        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        let authority_bump = *ctx.bumps.get("vault_authority").unwrap();
        let authority_seed = [&b"vault_authority"[..], &[authority_bump]];
        let authority_seeds = [&authority_seed[..]];

        let transfer_cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                authority: ctx.accounts.vault_authority.clone(),
                from: ctx.accounts.token_account.to_account_info(),
                to: ctx.accounts.to_token_account.to_account_info(),
            },
        )
        .with_signer(&authority_seeds);

        token::transfer(transfer_cpi_context, ctx.accounts.token_account.amount)?;
        Ok(())
    }

    pub fn transfer_owner(ctx: Context<TransferOwner>) -> Result<()> {
        ctx.accounts.app_state.owner = ctx.accounts.new_owner.key();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    payer: Signer<'info>,
    mint: Account<'info, Mint>,
    /// CHECK: No Check Required
    #[account(
        seeds = [b"vault_authority"],
        bump
    )]
    vault_authority: AccountInfo<'info>,
    #[account(
        init,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = vault_authority,
    )]
    mint_account: Account<'info, TokenAccount>,
    #[account(
        init,
        seeds = [],
        bump,
        space = 8 + 32 + 32 + 32,
        payer = payer,
    )]
    app_state: Account<'info, AppState>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferOwner<'info> {
    #[account(
        seeds = [],
        bump,
    )]
    app_state: Account<'info, AppState>,
    #[account(
        mut,
        constraint = app_state.owner.eq(payer.key)
    )]
    payer: Signer<'info>,
    /// CHECK: No Check Required
    new_owner: AccountInfo<'info>
}

#[derive(Accounts)]
pub struct UpdatePair<'info> {
    #[account(
        seeds = [],
        bump,
    )]
    app_state: Account<'info, AppState>,
    #[account(
        mut,
        constraint = app_state.owner.eq(payer.key)
    )]
    payer: Signer<'info>,
    token: Account<'info, Mint>,
    #[account(
        mut,
        seeds = [&b"sale_pair"[..], &token.key().to_bytes()],
        bump,
    )]
    sale_pair: Account<'info, SalePair>,
}

#[derive(Accounts)]
pub struct InitPair<'info> {
    #[account(
        seeds = [],
        bump,
    )]
    app_state: Account<'info, AppState>,
    #[account(
        mut,
        constraint = app_state.owner.eq(payer.key)
    )]
    payer: Signer<'info>,
    #[account(
        init,
        seeds = [&b"sale_pair"[..], &token.key().to_bytes()],
        bump,
        space = 8 + 8 + 32 + 32,
        payer = payer,
    )]
    sale_pair: Account<'info, SalePair>,
    /// CHECK: No Check Required
    #[account(
        seeds = [b"vault_authority"],
        bump
    )]
    vault_authority: AccountInfo<'info>,
    token: Account<'info, Mint>,
    #[account(
        init,
        payer = payer,
        associated_token::mint = token,
        associated_token::authority = vault_authority,
    )]
    token_account: Account<'info, TokenAccount>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Purchase<'info> {
    #[account(mut)]
    payer: Signer<'info>,
    /// CHECK: No Check Required
    #[account(
        seeds = [b"vault_authority"],
        bump
    )]
    vault_authority: AccountInfo<'info>,
    #[account(address = app_state.mint)]
    mint: Account<'info, Mint>,
    #[account(address = sale_pair.token)]
    token: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = vault_authority,
    )]
    mint_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = token,
        associated_token::authority = vault_authority,
    )]
    token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        token::mint = mint,
        token::authority = payer,
    )]
    receive_mint_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        token::mint = token,
        token::authority = payer,
    )]
    from_token_account: Account<'info, TokenAccount>,
    #[account(
        seeds = [],
        bump,
    )]
    app_state: Account<'info, AppState>,
    #[account(
        seeds = [&b"sale_pair"[..], &token.key().to_bytes()],
        bump,
    )]
    sale_pair: Account<'info, SalePair>,
    token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        seeds = [],
        bump,
    )]
    app_state: Account<'info, AppState>,
    #[account(
        constraint = app_state.owner.eq(payer.key)
    )]
    payer: Signer<'info>,
    /// CHECK: No Check Required
    #[account(
        seeds = [b"vault_authority"],
        bump
    )]
    vault_authority: AccountInfo<'info>,
    token: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = token,
        associated_token::authority = vault_authority,
    )]
    token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        token::mint = token,
        token::authority = payer,
    )]
    to_token_account: Account<'info, TokenAccount>,
    token_program: Program<'info, Token>,
}


