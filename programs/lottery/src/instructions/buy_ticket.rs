use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_metadata_accounts_v3,CreateMetadataAccountsV3,
        Metadata,
        SetAndVerifySizedCollectionItem,set_and_verify_sized_collection_item,
    },
    token_interface::{mint_to,Mint,MintTo,TokenInterface,TokenAccount},
};
use mpl_token_metadata::accounts::{ MasterEdition };

use anchor_spl::metadata::mpl_token_metadata::{
    types::{
        DataV2,
    }
};

use crate::error::{LotteryProgramError};

#[constant]
pub const NAME: &str = "Token Lottery Ticket";

#[constant]
pub const URI: &str = "Token Lottery";

#[constant]
pub const SYMBOL: &str = "TICKET";

use crate::TokenLottery;

pub fn buy_ticket(ctx: Context<BuyTicket>) -> Result<()> {
    let clock = Clock::get()?;
    require!(clock.slot > ctx.accounts.token_lottery.start_time && clock.slot < ctx.accounts.token_lottery.end_time, 
        LotteryProgramError::LotteryEnded);
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

    // if you don't cast it, you fail 
    let signer_seeds: &[&[&[u8]]] = &[&[
        &[ctx.bumps.collection_mint],
    ]];

    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo{
                mint: ctx.accounts.ticket_mint.to_account_info(),
                to: ctx.accounts.destination.to_account_info(),
                authority: ctx.accounts.collection_mint.to_account_info(),
            },
            signer_seeds,
        ),
        1,
    )?;

    // Now we'll create the ticket metadata
    create_metadata_accounts_v3(
        CpiContext::new_with_signer(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: ctx.accounts.metadata.to_account_info(),
                mint: ctx.accounts.ticket_mint.to_account_info(),
                mint_authority: ctx.accounts.collection_mint.to_account_info(),
                update_authority: ctx.accounts.collection_mint.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            &signer_seeds,
        ),
        DataV2 {
            name: NAME.to_string(),
            symbol: SYMBOL.to_string(),
            uri: URI.to_string(),
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        },
        true,
        true,
        None,
    )?;

    // Adding to master edition -> Account which prohibits minting

    set_and_verify_sized_collection_item(
        CpiContext::new_with_signer(
            ctx.accounts.token_metadata_program.to_account_info(),
            SetAndVerifySizedCollectionItem {
                metadata: ctx.accounts.metadata.to_account_info(),
                collection_authority: ctx.accounts.collection_mint.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
                update_authority: ctx.accounts.collection_mint.to_account_info(),
                collection_mint: ctx.accounts.collection_mint.to_account_info(),
                collection_metadata: ctx.accounts.collection_metadata_account.to_account_info(),
                collection_master_edition: ctx
                    .accounts
                    .collection_master_edition
                    .to_account_info(),
            },
            &signer_seeds,
        ),
        None,
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
        mint::authority = ticket_mint,
        mint::freeze_authority = ticket_mint,
    )]
    pub ticket_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = payer,
        associated_token::mint = ticket_mint,
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

    

     /// CHECK:
     #[account(
        mut,
        address=MasterEdition::find_pda(&collection_mint.key()).0
    )]
    pub collection_metadata_account: UncheckedAccount<'info>, 

     /// CHECK:
     #[account(
        mut,
        address=MasterEdition::find_pda(&collection_mint.key()).0
    )]
    pub collection_master_edition: UncheckedAccount<'info>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info,TokenInterface>,
    pub system_program: Program<'info,System>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub rent: Sysvar<'info, Rent>,
}