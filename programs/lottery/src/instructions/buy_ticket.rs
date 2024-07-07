use anchor_lang::prelude::*;

use anchor_spl::token::{Transfer};

pub fn buy_ticket(ctx: Context<BuyTicket>, amount: u64) -> Result<()> {
    let lottery = &mut ctx.accounts.token_lottery;
    let pot = &mut ctx.accounts.lottery_pot;

    ctx.accounts.token_program.transfer(
        CpiContext::New(
            ctx.accounts.token_program.clone(),
            Transfer{
                from: ctx.accounts.from.to_account_info().clone(),
                to: pot.to_account_info().clone(),
                authority: ctx.accounts.from.to_account_info().clone(),
            },
        ),
        amount,
    )?;

    pot.amount += amount;
    
    Ok(())
}

#[derive(Accounts)]
pub struct BuyTicket<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}