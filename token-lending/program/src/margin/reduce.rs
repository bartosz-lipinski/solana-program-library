use crate::{
    error::LendingError,
    state::{ReserveConfig, ReserveFees},
    instruction::{BorrowAmountType}
};

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    decode_error::DecodeError,
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::{PrintProgramError, ProgramError},
    program_option::COption,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
    sysvar::{clock::Clock, rent::Rent, Sysvar},
};

use num_traits::FromPrimitive;

/// Reduces margin position
#[inline(never)] // avoid stack frame limit
pub fn process_reduce_position(
    program_id: &Pubkey,
    amount: u64,
    amount_type: BorrowAmountType,
    leverage: u64, 
    min_position_to_amount: u64,
    accounts: &[AccountInfo],
) -> ProgramResult {
    if amount == 0 {
        return Err(LendingError::InvalidAmount.into());
    }

    Ok(())
}