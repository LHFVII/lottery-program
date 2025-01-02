use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_master_edition_v3, create_metadata_accounts_v3, sign_metadata,
        CreateMasterEditionV3, CreateMetadataAccountsV3, Metadata, SignMetadata,
    },
    token::mint_to,
    token_interface::{Mint, TokenAccount, TokenInterface},
};
use mpl_token_metadata::types::{Creator, DataV2};

#[constant]
pub const name: &str = "Token Lottery number #";
#[constant]
pub const symbol: &str = "TLT";
#[constant]
pub const uri: &str = "https://www.freepik.es/fotos-vectores-gratis/ticket-shapes/7";

pub fn initialize_lottery(ctx: Context<InitializeLottery>) -> Result<(())> {
    let signer_seeds: &[&[&[u8]]] = &[&[b"collection_mint".as_ref(), &[ctx.bumps.collection_mint]]];

    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::MintTo {
                mint: ctx.accounts.collection_mint.to_account_info(),
                to: ctx.accounts.token_collection_account.to_account_info(),
                authority: ctx.accounts.collection_mint.to_account_info(),
            },
            &signer_seeds,
        ),
        1,
    )?;

    create_metadata_accounts_v3(
        CpiContext::new_with_signer(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: ctx.accounts.metadata.to_account_info(),
                mint: ctx.accounts.collection_mint.to_account_info(),
                mint_authority: ctx.accounts.collection_mint.to_account_info(),
                payer: ctx.accounts.signer.to_account_info(),
                update_authority: ctx.accounts.signer.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            &signer_seeds,
        ),
        DataV2 {
            name: name.to_string(),
            symbol: symbol.to_string(),
            uri: "".to_string(),
            seller_fee_basis_points: 0,
            creators: Some(vec![Creator {
                address: ctx.accounts.collection_mint.key(),
                verified: false,
                share: 100,
            }]),
            collection: None,
            uses: None,
        },
        true,
        true,
        Some(mpl_token_metadata::types::CollectionDetails::V1 { size: 0 }),
    )?;

    create_master_edition_v3(
        CpiContext::new_with_signer(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMasterEditionV3 {
                payer: ctx.accounts.signer.to_account_info(),
                mint: ctx.accounts.collection_mint.to_account_info(),
                edition: ctx.accounts.master_edition.to_account_info(),
                mint_authority: ctx.accounts.collection_mint.to_account_info(),
                update_authority: ctx.accounts.collection_mint.to_account_info(),
                metadata: ctx.accounts.metadata.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            &signer_seeds,
        ),
        Some(0),
    )?;

    sign_metadata(CpiContext::new_with_signer(
        ctx.accounts.token_metadata_program.to_account_info(),
        SignMetadata {
            creator: ctx.accounts.collection_mint.to_account_info(),
            metadata: ctx.accounts.metadata.to_account_info(),
        },
        &signer_seeds,
    ))?;

    return Ok(());
}

#[derive(Accounts)]
pub struct InitializeLottery<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        mint::decimals = 0,
        mint::authority = collection_mint,
        mint::freeze_authority = collection_mint,
        seeds = [b"collection_mint".as_ref()],
        bump
    )]
    pub collection_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = signer,
        token::mint = collection_mint,
        token::authority = token_collection_account,
        seeds = [b"collection_associated_token".as_ref()],
        bump
    )]
    pub token_collection_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"metadata", token_metadata_program.key().as_ref(),collection_mint.key().as_ref()],
        bump,
        seeds::program = token_metadata_program.key()
    )]
    /// CHECK: This account is checked by the metadata program
    pub metadata: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [
            b"metadata", 
            token_metadata_program.key().as_ref(),
            collection_mint.key().as_ref(),
            b"edition"
            ],
        bump,
        seeds::program = token_metadata_program.key()
    )]
    /// CHECK: This account is checked by the metadata program
    pub master_edition: UncheckedAccount<'info>,

    pub token_metadata_program: Program<'info, Metadata>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}
