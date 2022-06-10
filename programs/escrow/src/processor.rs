use anchor_lang::prelude::*;
use anchor_spl::token;
use spl_token::instruction::AuthorityType;
use anchor_spl::token::{
  Transfer,
  SetAuthority,
};

use crate::instructions::{
  CreateEscrow,
  SwapEscrow,
};

const PDA_SEED: &[u8] = b"escrow_pda_seed";

pub fn create_escrow(
  ctx: Context<CreateEscrow>,
  expense: u64,
  expect: u64,
) -> Result<()> {
  let escrow_account = &mut ctx.accounts.escrow_account;
  let signer: &Signer = &ctx.accounts.signer;

  // 存储 state EscrowAccount
  escrow_account.authority = *signer.key;
  escrow_account.expense_token_account = *ctx.accounts.expense_token_account.to_account_info().key;
  escrow_account.expect_token_account = *ctx.accounts.expect_token_account.to_account_info().key;
  escrow_account.expense_amount = expense;
  escrow_account.expect_amount = expect;

  // 将 expense token 的 owner 改为 pda
  let (pda_pubkey, _bump) = Pubkey::find_program_address(&[PDA_SEED], ctx.program_id);
  escrow_account.pda = pda_pubkey;
  token::set_authority(ctx.accounts.into(), AuthorityType::AccountOwner, Some(pda_pubkey))
}

pub fn swap_escrow(ctx: Context<SwapEscrow>) -> Result<()> {
  // 0 TODO check accounts and amounts

  let (_pda, bump) = Pubkey::find_program_address(&[PDA_SEED], ctx.program_id);
  let seeds = &[&PDA_SEED[..], &[bump]];

  // 1 将已经属于 pad 的代币转给 taker 的 x 币账户
  let transfer_taker_accounts = Transfer {
    from: ctx.accounts.creator_token_account_x.clone(),
    to:  ctx.accounts.taker_token_account_x.clone(),
    authority: ctx.accounts.pda_account.clone(),
  };
  let transfer_taker_context = CpiContext::new(ctx.accounts.token_program.to_account_info(), transfer_taker_accounts);
  token::transfer(transfer_taker_context.with_signer(&[&seeds[..]]), ctx.accounts.escrow_account.expense_amount)?;

  // 2 将 taker 的 y 代币转给 creator 的 y 币账户
  let transfer_creator_accounts = Transfer {
    from: ctx.accounts.taker_token_account_y.clone(),
    to:  ctx.accounts.creator_token_account_y.clone(),
    authority: ctx.accounts.taker.clone(),
  };
  let transfer_creator_context = CpiContext::new(ctx.accounts.token_program.to_account_info(), transfer_creator_accounts);
  token::transfer(transfer_creator_context, ctx.accounts.escrow_account.expect_amount)?;

  // 3 将 creator 的 x 账户所有权复原
  let set_authority_accounts = SetAuthority {
    current_authority: ctx.accounts.pda_account.clone(),
    account_or_mint: ctx.accounts.creator_token_account_x.clone(),
  };
  let set_authority_context = CpiContext::new(ctx.accounts.token_program.to_account_info(), set_authority_accounts);
  token::set_authority(set_authority_context.with_signer(&[&seeds[..]]), AuthorityType::AccountOwner, Some(ctx.accounts.escrow_account.authority))
  
  // 4 todo close escrow_account
}