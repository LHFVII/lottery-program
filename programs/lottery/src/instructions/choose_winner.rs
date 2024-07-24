use anchor_lang::prelude::*;

pub fn choose_winner(ctx: Context<ChooseWinner>) -> Result<()> {
    let clock = Clock::get()?;
    Ok(())
}

#[derive(Accounts)]
pub struct ChooseWinner<'info>{
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub token_lottery<'info, TokenLottery>,
}