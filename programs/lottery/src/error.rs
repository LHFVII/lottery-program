use anchor_lang::prelude::*;

#[error_code]
pub enum LotteryProgramError {
    
    #[msg("Lottery is already finished")]
    LotteryEnded,

    #[msg("Lottery hasn't ended yet")]
    LotteryNotEndedYet,

    #[msg("Randomness has already been revealed!")]
    RandomnessAlreadyRevealed,
}