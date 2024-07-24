use anchor_lang::prelude::*;

use switchboard_on_demand::accounts::RandomnessAccountData;

use crate::instructions::initialize::TokenLottery;
use crate::error::{LotteryProgramError};

pub fn choose_winner(ctx: Context<ChooseWinner>) -> Result<()> {
    let clock = Clock::get()?;
    let token_lottery = &mut ctx.accounts.token_lottery;

    require!(
        ctx.accounts.payer.key() == token_lottery.authority,
        LotteryProgramError::NotAuthorized);
    
    require!(
        ctx.accounts.randomness_account_data.key() == token_lottery.randomness_account,
        LotteryProgramError::IncorrectRandomnessAccount
    );

    require!(clock.slot >= token_lottery.end_time,
        LotteryProgramError::LotteryNotEndedYet);

    let randomness_data = RandomnessAccountData::parse(ctx.accounts.randomness_account_data.data.borrow()).unwrap();
    let revealed_random_value = randomness_data.get_value(&clock)
        .map_err(|_| LotteryProgramError::RandomnessNotResolved)?;
    
    let randomness_result = revealed_random_value[0] as u32 % token_lottery.ticket_num;

    token_lottery.winner = randomness_result;

    Ok(())
}

#[derive(Accounts)]
pub struct ChooseWinner<'info>{
    pub payer: Signer<'info>,
    
    #[account(mut)]
    pub token_lottery: Account<'info, TokenLottery>,

    /// CHECK: The account's data is validated manually by the handler.
    pub randomness_account_data: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>
}