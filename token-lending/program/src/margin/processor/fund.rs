use crate::{
    error::LendingError,
    instruction::BorrowAmountType,
    margin::token_swap::spl_token_swap,
    state::{LendingMarket, Reserve},
};

use solana_program::{account_info::{next_account_info, AccountInfo}, clock::Clock, entrypoint::ProgramResult, msg, program_pack::Pack, pubkey::Pubkey, rent::Rent, sysvar::Sysvar};

/// Open/Increase margin position
/// For long position:
///     1. deposit collateral (lending quote)
///     2. borrow leveraged amount (lending quote)
///     3. swap using AMM to base
///     4. store base inside position
/// For short position:
///     1. deposit collateral (lending quote)
///     2. borrow leveraged amount (base)
///     3. swap using AMM to quote
///     4. store quote inside position
#[inline(never)] // avoid stack frame limit
pub fn process_fund_position(
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

    let account_info_iter = &mut accounts.iter();
    let source_collateral_info = next_account_info(account_info_iter)?;

    let token_swap_pool_info = next_account_info(account_info_iter)?;
    let token_swap_authority_info = next_account_info(account_info_iter)?;
    let token_swap_swap_source_info = next_account_info(account_info_iter)?;
    let token_swap_swap_destination_info = next_account_info(account_info_iter)?;
    let token_swap_pool_mint_info = next_account_info(account_info_iter)?;
    let token_swap_pool_fee_account_info = next_account_info(account_info_iter)?;

    let deposit_reserve_info = next_account_info(account_info_iter)?;
    let deposit_reserve_collateral_supply_info = next_account_info(account_info_iter)?;
    let deposit_reserve_collateral_fees_receiver_info = next_account_info(account_info_iter)?;

    let borrow_reserve_info = next_account_info(account_info_iter)?;
    let borrow_reserve_liquidity_supply_info = next_account_info(account_info_iter)?;

    let position_info = next_account_info(account_info_iter)?;
    let position_token_mint_info = next_account_info(account_info_iter)?;
    let position_token_output_info = next_account_info(account_info_iter)?;
    let position_token_owner_info = next_account_info(account_info_iter)?;
    let position_escrow_account_info = next_account_info(account_info_iter)?;

    let lending_market_info = next_account_info(account_info_iter)?;
    let lending_market_authority_info = next_account_info(account_info_iter)?;
    let user_transfer_authority_info = next_account_info(account_info_iter)?;

    let memory = next_account_info(account_info_iter)?;
    let clock = &Clock::from_account_info(next_account_info(account_info_iter)?)?;
    let rent_info = next_account_info(account_info_iter)?;
    let rent = &Rent::from_account_info(rent_info)?;
    let token_program_id = next_account_info(account_info_iter)?;
    let token_swap_program_id = next_account_info(account_info_iter)?;

    // TODO: should this be optional?
    let host_fee_recipient = next_account_info(account_info_iter)?;

    let lending_market = LendingMarket::unpack(&lending_market_info.data.borrow())?;
    if &lending_market.token_program_id != token_program_id.key {
        return Err(LendingError::InvalidTokenProgram.into());
    }

    let mut deposit_reserve = Reserve::unpack(&deposit_reserve_info.data.borrow())?;
    if deposit_reserve_info.owner != program_id {
        return Err(LendingError::InvalidAccountOwner.into());
    }
    if &deposit_reserve.lending_market != lending_market_info.key {
        msg!("Invalid reserve lending market account");
        return Err(LendingError::InvalidAccountInput.into());
    }

    let mut borrow_reserve = Reserve::unpack(&borrow_reserve_info.data.borrow())?;
    if borrow_reserve_info.owner != program_id {
        return Err(LendingError::InvalidAccountOwner.into());
    }
    if borrow_reserve.lending_market != deposit_reserve.lending_market {
        return Err(LendingError::LendingMarketMismatch.into());
    }

    // accrue interest and update rates
    borrow_reserve.accrue_interest(clock.slot);
    deposit_reserve.accrue_interest(clock.slot);

    // TODO: initalize position

    let swap_amount_in = 0;
    let swap_min_amount_out = 0;
    // swap tokens using AMM
    spl_token_swap(
        token_swap_program_id.clone(),
        token_program_id.clone(),
        token_swap_pool_info.clone(),
        token_swap_authority_info.clone(),
        user_transfer_authority_info.clone(),
        deposit_reserve_collateral_supply_info.clone(),
        token_swap_swap_source_info.clone(),
        token_swap_swap_destination_info.clone(),
        position_escrow_account_info.clone(),
        token_swap_pool_mint_info.clone(),
        token_swap_pool_fee_account_info.clone(),
        host_fee_recipient.clone(),
        swap_amount_in,
        swap_min_amount_out,
    )?;

    Ok(())
}
