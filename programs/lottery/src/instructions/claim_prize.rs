use anchor_lang::prelude::*;
use anchor_spl::{metadata::{Metadata, MetadataAccount}, token::{Mint, TokenAccount}, token_interface::TokenInterface};

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
    pub ticket_account: InterfaceAccount<'info,TokenAccount>,

    #[account(
        seeds= [b"metadata", token_metadata_program.key().as_ref(), collection_mint.key().as_ref()],
        bump,
        seeds::program = token_metadata_program.key()
    )]
    pub collection_metadata: Account<'info, MetadataAccount>,



    pub token_metadata_program: Program<'info, Metadata>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
