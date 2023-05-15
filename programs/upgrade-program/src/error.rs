//! Error types

use num_derive::FromPrimitive;
use solana_program::{decode_error::DecodeError, msg, program_error::{PrintProgramError, ProgramError}};
use thiserror::Error;

/// Errors that may be returned by the Token program.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum UpgradeError {
    /// 0 The account cannot be initialized because it is already being used.
    #[error("Already in use")]
    AlreadyInUse,
    /// 1 The account hasn't been initialized
    #[error("Not initialized")]
    NotInitialized,
    /// 2 Wrong admin account
    #[error("Wrong admin")]
    WrongAdmin,
    /// 3 Wrong seeds for admin account
    #[error("Wrong seeds")]
    WrongSeeds,
    /// 4 Wrong signature key
    #[error("Wrong signature public key")]
    WrongSignature,
    /// 5 Invalid signature
    #[error("Invalid signature")]
    InvalidSignature,
}


impl From<UpgradeError> for ProgramError {
    fn from(e: UpgradeError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl PrintProgramError for UpgradeError {
    fn print<E>(&self) {
        msg!(&self.to_string());
    }
}

impl<T> DecodeError<T> for UpgradeError {
    fn type_of() -> &'static str {
        "UpgradeError"
    }
}
