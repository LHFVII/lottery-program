pub mod error;
pub mod instructions;

use anchor_lang::prelude::*;

use instructions::*;

declare_id!("GybA6juy56pVEdv9W8aqEyDGec5wkWf5ZLAed5GfEEtW");

#[program]
pub mod lottery {
    use super::*;

    pub fn buy_ticket(ctx: Context<BuyTicket>) -> Result<()> {
        instructions::buy_ticket(ctx)
    }

    pub fn choose_winner(ctx: Context<ChooseWinner>) -> Result<()> {
        instructions::choose_winner(ctx)
    }

    pub fn claim_prize(ctx: Context<ClaimPrize>) -> Result<()> {
        instructions::claim_prize(ctx)
    }

    pub fn commit_winner(ctx: Context<CommitWinner>) -> Result<()> {
        instructions::commit_winner(ctx)
    }

    pub fn initialize(ctx: Context<Initialize>, start: u64, end: u64, price: u64) -> Result<()> {
        instructions::initialize(ctx, start, end, price)
    }

    pub fn initialize_lottery(ctx: Context<InitializeLottery>) -> Result<()> {
        return instructions::initialize_lottery(ctx);
    }
}
