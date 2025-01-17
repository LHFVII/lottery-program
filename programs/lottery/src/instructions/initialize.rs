use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenInterface},
};

pub fn initialize(ctx: Context<Initialize>, start: u64, end: u64, price: u64) -> Result<()> {
    ctx.accounts.token_lottery_config.bump = ctx.bumps.token_lottery_config;
    ctx.accounts.token_lottery_config.mint = ctx.accounts.mint.key();
    ctx.accounts.token_lottery_config.start_time = start;
    ctx.accounts.token_lottery_config.end_time = end;
    ctx.accounts.token_lottery_config.ticket_num = 0;
    ctx.accounts.token_lottery_config.price = price;
    ctx.accounts.token_lottery_config.randomness_account = Pubkey::default();
    ctx.accounts.token_lottery_config.authority = ctx.accounts.signer.key();
    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + TokenLottery::INIT_SPACE,
        seeds =[b"token_lottery_config".as_ref()],
        bump
    )]
    pub token_lottery_config: Account<'info, TokenLottery>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = signer,
        mint::decimals = 0,
        mint::authority = collection_mint,
        seeds = [token_lottery_config.key().as_ref()],
        bump,
    )]
    pub collection_mint: InterfaceAccount<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct TokenLottery {
    pub bump: u8,
    pub winner: u32,
    pub winner_claimed: bool,
    pub start_time: u64,
    pub end_time: u64,
    pub lottery_pot_amount: u64,
    pub mint: Pubkey,
    pub ticket_num: u32,
    pub price: u64,
    pub authority: Pubkey,
    pub randomness_account: Pubkey,
}
