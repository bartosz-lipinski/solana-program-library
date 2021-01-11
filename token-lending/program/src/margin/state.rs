use crate::{
    math::{Decimal}
};

use solana_program::{
    pubkey::Pubkey,
};


/// Margin position state
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MarginPosition {
    /// Version of the obligation
    pub version: u8,

    /// Token swap liquidity pool used for margin requirements and trade
    pub token_swap: Pubkey,

    /// Margin (borrow) reserve used to borrow money
    pub position_reserve: Pubkey,
    /// (Escrow) Account that holds position
    /// Potentially we could feed that back to 
    /// the lending platform to provide more liquidity and earn intrests on deposits
    pub position_account: Pubkey,
    /// Mint used to create position tokens
    pub position_mint: Pubkey,
    /// Amount of tokens held for this position plus interest
    pub position_amount_wads: Decimal,

    /// Reserve which collateral tokens were deposited into
    pub collateral_reserve: Pubkey,
    /// How much in collateral tokens was spend on buying position tokens
    pub collateral_amount: u64, 

    /// Slot when position was opened or funded or reduced, used to calculate outstading fee
    pub last_update_slot: u64,
    /// How much is charged each slot in liquidity tokens while position is opened, =position_fee * amount
    pub fee_per_slot: u64,
}
