use anchor_lang::prelude::*;

pub mod processor;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("J3bDGL9oCv2uKoKXvkzn78cqs1WMLUc3jWBAF28E5ztC");

#[program]
pub mod escrow {
  use super::*;

  /**
   * 创建一个 escrow
   * 
   */
  pub fn create_escrow(
    ctx: Context<CreateEscrow>,
    expense: u64,
    expect: u64,
  ) -> Result<()> {
    processor::create_escrow(ctx, expense, expect)
  }

  /**
   * 交换
   */
  pub fn swap_escrow(ctx: Context<SwapEscrow>) -> Result<()> {
    processor::swap_escrow(ctx)
  }

  // TODO
  // pub fn cancel_escrow() -> Result<()> {
  //   Ok(())
  // }

}