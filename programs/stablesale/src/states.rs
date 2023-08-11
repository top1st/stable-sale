use anchor_lang::prelude::*;

#[account]
pub struct AppState {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub mint_account: Pubkey
}

#[account]
pub struct SalePair {
    pub price: u64,
    pub token: Pubkey,
    pub token_account: Pubkey,

}