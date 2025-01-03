use anchor_lang::prelude::*;
use anchor_spl::{token::Mint, token_interface::TokenInterface};

use super::TokenLottery;

pub fn claim_prize(ctx: Context<ClaimPrize>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct ClaimPrize<'info> {
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds=[b"token_lottery".as_ref()],
        bump = token_lottery.bump
    )]
    pub token_lottery: Account<'info, TokenLottery>,
    #[account(
        seeds=[token_lottery.winner.to_le_bytes().as_ref()],
        bump,
    )]
    pub ticket_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
