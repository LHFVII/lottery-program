use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint,TokenAccount, TokenInterface}
};

pub fn initialize(ctx: Context<Initialize>, start: u64, end: u64) -> Result<()> {
    ctx.accounts.token_lottery.bump = ctx.bumps.token_lottery;
    ctx.accounts.token_lottery.mint = ctx.accounts.mint.key();
    ctx.accounts.token_lottery.pot = ctx.accounts.lottery_pot_account.key();
    ctx.accounts.token_lottery.start_time = start;
    ctx.accounts.token_lottery.end_time = end;
    ctx.accounts.token_lottery.ticket_num = 0;
    ctx.accounts.token_lottery.amount = 0;
    
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
    pub lottery_pot_account: InterfaceAccount<'info, TokenAccount>,
    
    pub mint: InterfaceAccount<'info,Mint>,

    #[account(
        init,
        payer = signer,
        mint::decimals = 0,
        mint::authority = collection_mint,
        seeds = [token_lottery.key().as_ref()],
        bump,
    )]
    pub collection_mint: InterfaceAccount<'info,Mint>,

    /// CHECK: This account will be initialized by the metaplex program
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,

    /// CHECK: This account will be initialized by the metaplex program
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = collection_mint,
        associated_token::authority = collection_token_account
    )]
    pub collection_token_account: InterfaceAccount<'info, TokenAccount>,
    
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info,TokenInterface>,
    pub system_program: Program<'info,System>

}

#[account]
#[derive(InitSpace)]
pub struct TokenLottery{
    pub bump:u8,
    pub winner:u32,
    pub start_time: u64,
    pub end_time: u64,
    pub pot: Pubkey,
    pub mint: Pubkey,
    pub ticket_num: u64,
    pub amount: u64,
}
