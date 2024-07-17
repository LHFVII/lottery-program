pub mod error;
pub mod instructions;

use anchor_lang::prelude::*;

use instructions::*;

declare_id!("GybA6juy56pVEdv9W8aqEyDGec5wkWf5ZLAed5GfEEtW");

#[program]
pub mod lottery {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, start: u64, end: u64, price:u64) -> Result<()> {
        instructions::initialize(ctx, start, end, price)
    }

    pub fn buy_ticket(ctx: Context<BuyTicket>) -> Result<()> {
        instructions::buy_ticket(ctx)
    }
}