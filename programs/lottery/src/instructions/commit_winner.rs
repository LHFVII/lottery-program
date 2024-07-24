use anchor_lang::prelude::*;

pub fn commit_winner(ctx: Context<ChooseWinner>) -> Result<()>{
    
}

#[derive(Accounts)]
pub struct CommitWinner<'info>{
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub token_lottery<'info, TokenLottery>,
}