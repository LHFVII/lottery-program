use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{Metadata,create_master_edition_v3, create_metadata_accounts_v3,CreateMetadataAccountsV3,CreateMasterEditionV3},
    token_interface::{mint_to,Mint,MintTo,TokenInterface,TokenAccount},
};
use anchor_spl::metadata::mpl_token_metadata::{
    types::{
        DataV2,
    }
};


use crate::TokenLottery;

#[constant]
pub const name: &str = "Token Lottery Ticket";

#[constant]
pub const uri: &str = "Token Lottery";

#[constant]
pub const symbol: &str = "TICKET";

pub fn buy_ticket(ctx: Context<BuyTicket>) -> Result<()> {
    let metadata = &ctx.accounts.metadata.to_account_info();
    let master_edition = &ctx.accounts.master_edition.to_account_info();
    let mint = &ctx.accounts.mint.to_account_info();
    let system_program = &ctx.accounts.system_program.to_account_info();
    let spl_token_program = &ctx.accounts.token_program.to_account_info();
    let spl_metadata_program = &ctx.accounts.token_metadata_program.to_account_info();

    // if you don't cast it, you fail 
    let signer_seeds: &[&[&[u8]]] = &[&[
        &[ctx.bumps.collection_mint],
    ]];
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer{
                from: ctx.accounts.payer.to_account_info(),
                to: ctx.accounts.token_lottery.to_account_info(),
            },
        ),
        ctx.accounts.token_lottery.price
    )?;
    
    Ok(())
}

#[derive(Accounts)]
pub struct BuyTicket<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub token_lottery: Account<'info,TokenLottery>,

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
    
    #[account(
        mut,
        seeds = [token_lottery.key().as_ref()],
        bump,
    )]
    pub collection_mint: InterfaceAccount<'info, Mint>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info,TokenInterface>,
    pub system_program: Program<'info,System>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub rent: Sysvar<'info, Rent>,
}