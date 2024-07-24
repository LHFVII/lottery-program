use anchor_lang::prelude::*;
use switchboard_on_demand::accounts::RandomnessAccountData;

use crate::error::{LotteryProgramError};

pub fn commit_winner(ctx: Context<ChooseWinner>) -> Result<()>{
    let clock = Clock::get()?;
    require!(clock.slot >= ctx.accounts.token_lottery.end_time,
        LotteryProgramError::LotteryNotEndedYet)
    let randomness_data = RandomnessAccountData::parse(ctx.accounts.randomness_account_data.data.borrow().unwrap());
    require!(randomness_data.seed_slot < clock.slot -1,
        LotteryProgramError::RandomnessAlreadyRevealed
    )
    
    
}

#[derive(Accounts)]
pub struct CommitWinner<'info>{
    pub user: Signer<'info>,
    
    #[account(mut)]
    pub token_lottery: Account<'info, TokenLottery>,

    pub randomness_account_data: UncheckedAccount<'info>

    pub system_program: Program<'info, System>
}