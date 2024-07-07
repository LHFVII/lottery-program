use anchor_lang::prelude::*;

pub fn buy_ticket(ctx: Context<BuyTicket>) -> Result<()> {
    
    Ok(())
}

#[derive(Accounts)]
pub struct BuyTicket<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}