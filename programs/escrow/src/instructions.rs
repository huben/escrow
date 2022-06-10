use anchor_lang::prelude::*;
use anchor_spl::token::{
  Token,
  TokenAccount,
  SetAuthority,
};

use crate::state::{ 
  EscrowAccount,
};

#[derive(Accounts)]
pub struct CreateEscrow<'info> {
  #[account(
    init,
    payer = signer,
    space = EscrowAccount::LENGTH
  )]
  pub escrow_account: Account<'info, EscrowAccount>,
  #[account(mut)]
  pub signer: Signer<'info>,

  #[account(mut)]
  pub expense_token_account: Account<'info, TokenAccount>,
  pub expect_token_account: Account<'info, TokenAccount>,

  pub system_program: Program<'info, System>,
  pub token_program: Program<'info, Token>,
}
impl<'a, 'b, 'c, 'info> From<&mut CreateEscrow<'info>> for CpiContext<'a, 'b, 'c, 'info, SetAuthority<'info>> {
  fn from(accounts: &mut CreateEscrow<'info>) -> CpiContext<'a, 'b, 'c, 'info, SetAuthority<'info>> {
    let cpi_account = SetAuthority {
      current_authority: accounts.signer.to_account_info().clone(),
      account_or_mint: accounts.expense_token_account.to_account_info().clone(),
    };
    let cpi_program = accounts.token_program.to_account_info();
    CpiContext::new(cpi_program, cpi_account)
  }
}

#[derive(Accounts)]
pub struct SwapEscrow<'info> {
  #[account(mut)]
  pub escrow_account: Account<'info, EscrowAccount>,

  #[account(signer)]
  /// CHECK:
  pub taker: AccountInfo<'info>,

  #[account(mut)]
  /// CHECK:
  pub taker_token_account_x: AccountInfo<'info>,
  #[account(mut)]
  /// CHECK:
  pub taker_token_account_y: AccountInfo<'info>,

  #[account(mut)]
  /// CHECK:
  pub creator_token_account_x: AccountInfo<'info>,
  #[account(mut)]
  /// CHECK:
  pub creator_token_account_y: AccountInfo<'info>,

  /// CHECK:
  pub pda_account: AccountInfo<'info>,
  /// CHECK:
  pub token_program: AccountInfo<'info>,

}