use anchor_lang::prelude::*;

pub fn choose_winner(ctx: Context<ChooseWinner>) -> Result<()> {
    let clock = Clock::get()?;
    let token_lottery = &mut ctx.accounts.token_lottery;

    let randomness_data = RandomnessAccountData::parse(ctx.accounts.randomness_account_data.data.borrow()).unwrap();
    let revealed_random_value = randomness_data.get(value(&clock))
        .map_err(|_| LotteryProgramError::RandomnessNotResolved)?;
    
    let randomness_result = revealed_random_value[0] % token_lottery.ticket_num;

    Ok(())
}

#[derive(Accounts)]
pub struct ChooseWinner<'info>{
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub token_lottery<'info, TokenLottery>,
}