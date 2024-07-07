use anchor_lang::prelude::*;

#[error_code]
pub enum LotteryProgramError {
    
    #[msg("Math overflow on `u64` value")]
    InvalidArithmetic,
}