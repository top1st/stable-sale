use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

declare_id!("4gQyRdZge8YHmQ8jybBJGvYsGdNMYEq6zCS3xosrVFHN");

#[program]
pub mod stablesale {
    use anchor_spl::token;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.app_state.owner = ctx.accounts.payer.key();
        Ok(())
    }

    pub fn purchase(ctx: Context<Purchase>, amount: u64) -> Result<()> {
        let receive_cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                authority: ctx.accounts.payer.to_account_info(),
                from: ctx.accounts.from_usdt_account.to_account_info(),
                to: ctx.accounts.usdt_account.to_account_info(),
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
                to: ctx.accounts.receive_mint_account.to_account_info()
            }
        ).with_signer(&authority_seeds);

        token::transfer(transfer_cpi_context, amount)?;


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
                from: ctx.accounts.usdt_account.to_account_info(),
                to: ctx.accounts.to_usdt_account.to_account_info()
            }
        ).with_signer(&authority_seeds);
        
        token::transfer(transfer_cpi_context, ctx.accounts.usdt_account.amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    payer: Signer<'info>,
    mint: Account<'info, Mint>,
    usdt: Account<'info, Mint>,
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
        payer = payer,
        associated_token::mint = usdt,
        associated_token::authority = vault_authority,
    )]
    usdt_account: Account<'info, TokenAccount>,
    #[account(
        init,
        seeds = [],
        bump,
        space = 8 + 32,
        payer = payer,
    )]
    app_state: Account<'info, AppState>,
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
    mint: Account<'info, Mint>,
    usdt: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = vault_authority,
    )]
    mint_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = usdt,
        associated_token::authority = vault_authority,
    )]
    usdt_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        token::mint = mint,
        token::authority = payer,
    )]
    receive_mint_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        token::mint = usdt,
        token::authority = payer,
    )]
    from_usdt_account: Account<'info, TokenAccount>,
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
    usdt: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = usdt,
        associated_token::authority = vault_authority,
    )]
    usdt_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        token::mint = usdt,
        token::authority = payer,
    )]
    to_usdt_account: Account<'info, TokenAccount>,
    token_program: Program<'info, Token>,
}

#[account]
pub struct AppState {
    owner: Pubkey,
}
