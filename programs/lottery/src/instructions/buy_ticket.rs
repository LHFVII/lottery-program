use anchor_lang::prelude::*;

use anchor_spl::token::{Transfer};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint,TokenInterface},
};
use crate::TokenLottery;

pub fn buy_ticket(ctx: Context<BuyTicket>, amount: u64) -> Result<()> {
    let lottery = &mut ctx.accounts.token_lottery;
    let pot = &mut ctx.accounts.lottery_pot;

    ctx.accounts.token_program.transfer(
        CpiContext::New(
            ctx.accounts.token_program.clone(),
            Transfer{
                from: ctx.accounts.payer.to_account_info().clone(),
                to: pot.to_account_info().clone(),
                authority: ctx.accounts.payer.to_account_info().clone(),
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

    #[account(mut)]
    pub token_lottery: Account<'info,TokenLottery>,

    #[account(mut)]
    pub lottery_pot: InterfaceAccount<'info,TokenLottery>,

    pub mint: InterfaceAccount<'info,Mint>,
    
    pub token_program: Interface<'info,TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info,System>

}