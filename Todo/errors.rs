use anchor_lang::prelude::*;

#[error_code]
pub enum TodoError {
    #[msg("You are unauthorized to perform this action")]
    Unauthorized,
    #[msg("Not Allowed")]
    NotAllowed,
    #[msg("Math operation overflow")]
    MathOverflow,
    #[msg("Already marked")]
    AlreadyMarked,
}
