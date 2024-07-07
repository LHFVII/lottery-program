use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint,TokenInterface,TokenAccount},
};
use crate::TokenLottery;

pub fn buy_ticket(ctx: Context<BuyTicket>, amount: u64) -> Result<()> {
    /*let lottery = &mut ctx.accounts.token_lottery;
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

    pot.amount += amount;*/
    
    Ok(())
}

#[derive(Accounts)]
pub struct BuyTicket<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub token_lottery: Account<'info,TokenLottery>,

    #[account(mut)]
    pub lottery_pot: InterfaceAccount<'info,TokenAccount>,

    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = mint,
        mint::freeze_authority = mint,
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = payer
    )]
    pub destination: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    /// CHECK: This account will be initialized by the metaplex program
    pub metadata: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: This account will be initialized by the metaplex program
    pub master_edition: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub collection_mint: InterfaceAccount<'info, Mint>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info,TokenInterface>,
    pub system_program: Program<'info,System>

}