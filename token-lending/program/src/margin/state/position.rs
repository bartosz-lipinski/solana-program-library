use crate::math::{Decimal, pack_decimal};

use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

const UNINITIALIZED_VERSION: u8 = 0;

/// Margin position state
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MarginPosition {
    /// Version of the obligation
    pub version: u8,
    /// Slot when position was opened or funded or reduced, used to calculate outstading fee
    pub last_update_slot: u64,
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
    /// How much is charged each slot in liquidity tokens while position is opened, =position_fee * amount
    pub fee_per_slot: u64,
}

impl Sealed for MarginPosition {}
impl IsInitialized for MarginPosition {
    fn is_initialized(&self) -> bool {
        self.version != UNINITIALIZED_VERSION
    }
}

const POSITION_LEN: usize = 313;
impl Pack for MarginPosition {
    const LEN: usize = 313;

    fn pack_into_slice(&self, output: &mut [u8]) {
        let output = array_mut_ref![output, 0, POSITION_LEN];
        let (
            version,
            last_update_slot,
            token_swap,
            position_reserve,
            position_account,
            position_mint,
            position_amount_wads,
            collateral_reserve,
            collateral_amount,
            fee_per_slot,
        ) = mut_array_refs![output, 1, 8, 32, 32, 32, 32, 128, 32, 8, 8];

        *version = self.version.to_le_bytes();
        *last_update_slot = self.last_update_slot.to_le_bytes();
        token_swap.copy_from_slice(self.token_swap.as_ref());
        position_reserve.copy_from_slice(self.position_reserve.as_ref());
        position_account.copy_from_slice(self.position_account.as_ref());
        position_mint.copy_from_slice(self.position_mint.as_ref());
        pack_decimal(self.position_amount_wads, position_amount_wads);
        collateral_reserve.copy_from_slice(self.collateral_reserve.as_ref());
        *collateral_amount = self.collateral_amount.to_le_bytes();
        *fee_per_slot = self.fee_per_slot.to_le_bytes();
    }

    /// Unpacks a byte buffer into a [SwapInfo](struct.SwapInfo.html).
    fn unpack_from_slice(input: &[u8]) -> Result<Self, ProgramError> {
        let input = array_ref![input, 0, POSITION_LEN];
        #[allow(clippy::ptr_offset_with_cast)]
        let (
            version,
            last_update_slot,
            token_swap,
            position_reserve,
            position_account,
            position_mint,
            position_amount_wads,
            collateral_reserve,
            collateral_amount,
            fee_per_slot,
        ) = array_refs![input, 1, 8, 32, 32, 32, 32, 128, 32, 8, 8];
        Ok(Self {
            version: 
            token_program_id: Pubkey::new_from_array(*token_program_id),
            token_a: Pubkey::new_from_array(*token_a),
            token_b: Pubkey::new_from_array(*token_b),
            pool_mint: Pubkey::new_from_array(*pool_mint),
            token_a_mint: Pubkey::new_from_array(*token_a_mint),
            token_b_mint: Pubkey::new_from_array(*token_b_mint),
            pool_fee_account: Pubkey::new_from_array(*pool_fee_account),
            fees: Fees::unpack_from_slice(fees)?,
            swap_curve: SwapCurve::unpack_from_slice(swap_curve)?,
        })
    }
}
