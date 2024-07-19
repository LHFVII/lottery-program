pub fn choose_winner(ctx: Context<ChooseWinner>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct ChooseWinner<'info>{
    pub signer: Signer<'info>,
}