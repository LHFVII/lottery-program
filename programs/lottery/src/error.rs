use anchor_lang::prelude::*;

#[error_code]
pub enum LotteryProgramError {
    #[msg("Ticket account is empty")]
    EmptyTicketAccount,

    #[msg("Lottery is already finished")]
    LotteryEnded,

    #[msg("Lottery hasn't ended yet")]
    LotteryNotEndedYet,

    #[msg("Randomness has already been revealed!")]
    RandomnessAlreadyRevealed,

    #[msg("Randomness is not resolved!")]
    RandomnessNotResolved,

    #[msg("Signer is not authorized to execute the instruction")]
    NotAuthorized,

    #[msg("Ticket metadata collection is not verified")]
    NotVerified,

    #[msg("Wrong randomness account")]
    IncorrectRandomnessAccount,

    #[msg("Wrong ticket")]
    IncorrectTicket,

    #[msg("Winner has already been chosen")]
    WinnerAlreadyChosen,

    #[msg("Winner has not been chosen")]
    WinnerNotChosen,
}
