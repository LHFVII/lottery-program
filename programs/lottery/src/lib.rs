pub mod error;
pub mod instructions;

use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

use instructions::*;

declare_id!("GybA6juy56pVEdv9W8aqEyDGec5wkWf5ZLAed5GfEEtW");

#[program]
pub mod lottery {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, start: u64, end: u64) -> Result<()> {
        ctx.accounts.token_lottery.bump = ctx.bumps.token_lottery;
        ctx.accounts.token_lottery.mint = ctx.accounts.mint.key();
        ctx.accounts.token_lottery.pot = ctx.accounts.lottery_pot.key();
        ctx.accounts.token_lottery.start_time = start;
        ctx.accounts.token_lottery.end_time = end;
        
        Ok(())
    }

    pub fn buy_ticket(ctx: Context<BuyTicket>) -> Result<()> {
        instructions::buy_ticket(ctx)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + TokenLottery::INIT_SPACE,
        seeds =[b"token_lottery".as_ref()],
        bump
    )]
    pub token_lottery: Account<'info, TokenLottery>,
    
    #[account(
        init,
        payer = signer,
        seeds =[b"token_lottery".as_ref()],
        bump,
        token::mint = mint,
        token::authority = lottery_pot_account
    )]
    pub lottery_pot_account: Account<'info, TokenAccount>,
    pub mint: Account<'info,Mint>,
    pub token_program: Program<'info,Token>,
    pub system_program: Program<'info,System>

}

#[account]
#[derive(InitSpace)]
pub struct TokenLottery{
    pub bump:u8,
    pub winner:u32,
    pub start_time: u64,
    pub end_time: u64,
    // USDC
    pub pot: Pubkey,
}

// What's next?
// Adding consts
// Adding more instructions
//
