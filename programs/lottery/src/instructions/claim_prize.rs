use anchor_lang::prelude::*;

pub fn claim_prize(ctx: Context<ClaimPrize>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct ClaimPrize<'info> {
    pub system_program: Program<'info, System>,
}
