use anchor_lang::prelude::*;

#[account]
pub struct EscrowAccount {
  pub authority: Pubkey,
  pub expense_token_account: Pubkey,
  pub expect_token_account: Pubkey,
  pub expense_amount: u64,
  pub expect_amount: u64,
  pub pda: Pubkey, 
}

impl EscrowAccount {
  pub const LENGTH: usize = 8 
    + 32
    + 32
    + 32
    + 8
    + 8
    + 32
    ;
}