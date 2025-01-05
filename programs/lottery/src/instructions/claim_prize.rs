use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{Metadata, MetadataAccount},
    token::TokenAccount,
    token_interface::{Mint, TokenInterface},
};

use crate::{error::LotteryProgramError, NAME};

use super::TokenLottery;

pub fn claim_prize(ctx: Context<ClaimPrize>) -> Result<()> {
    require!(
        ctx.accounts.token_lottery.winner_claimed,
        LotteryProgramError::WinnerNotChosen
    );
    require!(
        ctx.accounts
            .ticket_metadata
            .collection
            .as_ref()
            .unwrap()
            .verified,
        LotteryProgramError::NotVerified
    );
    require!(
        ctx.accounts
            .ticket_metadata
            .collection
            .as_ref()
            .unwrap()
            .key
            == ctx.accounts.collection_mint.key(),
        LotteryProgramError::NotVerified
    );
    let ticket_name = NAME.to_owned() + &ctx.accounts.token_lottery.winner.to_string();
    let metadata_name = ctx.accounts.ticket_metadata.name.replace("\u{0}", "");
    require!(
        ticket_name == metadata_name,
        LotteryProgramError::IncorrectTicket
    );
    require!(
        ctx.accounts.ticket_account.amount > 0,
        LotteryProgramError::EmptyTicketAccount
    );

    **ctx
        .accounts
        .token_lottery
        .to_account_info()
        .lamports
        .borrow_mut() -= ctx.accounts.token_lottery.lottery_pot_amount;
    **ctx.accounts.signer.to_account_info().lamports.borrow_mut() +=
        ctx.accounts.token_lottery.lottery_pot_amount;
    ctx.accounts.token_lottery.lottery_pot_amount = 0;
    Ok(())
}

#[derive(Accounts)]
pub struct ClaimPrize<'info> {
    #[account(mut)]
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

    #[account(
        seeds = [b"collection_mint".as_ref()],
        bump
    )]
    pub collection_mint: InterfaceAccount<'info, Mint>,

    #[account(
        seeds= [b"metadata", token_metadata_program.key().as_ref(), ticket_mint.key().as_ref()],
        bump,
        seeds::program = token_metadata_program.key()
    )]
    pub ticket_metadata: Account<'info, MetadataAccount>,

    #[account(
        associated_token::mint = ticket_mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program,
    )]
    pub ticket_account: Account<'info, TokenAccount>,

    #[account(
        seeds= [b"metadata", token_metadata_program.key().as_ref(), collection_mint.key().as_ref()],
        bump,
        seeds::program = token_metadata_program.key()
    )]
    pub collection_metadata: Account<'info, MetadataAccount>,

    pub token_metadata_program: Program<'info, Metadata>,
    pub token_program: Interface<'info, TokenInterface>,
}
