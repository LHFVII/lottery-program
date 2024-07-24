use anchor_lang::prelude::*;
use switchboard_on_demand::accounts::RandomnessAccountData;

use crate::instructions::initialize::TokenLottery;

use crate::error::{LotteryProgramError};

pub fn commit_winner(ctx: Context<CommitWinner>, randomness_account: Pubkey) -> Result<()>{
    let clock = Clock::get()?;
    let token_lottery = &mut ctx.accounts.token_lottery;
    require!(clock.slot >= token_lottery.end_time,
        LotteryProgramError::LotteryNotEndedYet);
    let randomness_data = RandomnessAccountData::parse(ctx.accounts.randomness_account_data.data.borrow()).unwrap();
    require!(randomness_data.seed_slot < clock.slot -1,
        LotteryProgramError::RandomnessAlreadyRevealed
    );
    
    token_lottery.randomness_account = randomness_account;

    Ok(())
    
}

#[derive(Accounts)]
pub struct CommitWinner<'info>{
    pub user: Signer<'info>,
    
    #[account(mut)]
    pub token_lottery: Account<'info, TokenLottery>,

    /// CHECK: The account's data is validated manually by the handler.
    pub randomness_account_data: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>
}