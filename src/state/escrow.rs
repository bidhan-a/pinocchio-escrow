use super::utils::{load_acc_mut_unchecked, DataLen, Initialized};
use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::{self, Pubkey},
    ProgramResult,
};

use crate::error::MyProgramError;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Escrow {
    pub maker: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub receive_amount: u64,
    pub bump: u8,
}

impl DataLen for Escrow {
    const LEN: usize = core::mem::size_of::<Escrow>();
}

impl Escrow {
    pub fn initialize(
        escrow_account: &AccountInfo,
        maker: Pubkey,
        mint_a: Pubkey,
        mint_b: Pubkey,
        receive_amount: u64,
        bump: u8,
    ) {
        let escrow =
            unsafe { &mut *(escrow_account.borrow_mut_data_unchecked().as_ptr() as *mut Self) };

        escrow.maker = maker;
        escrow.mint_a = mint_a;
        escrow.mint_b = mint_b;
        escrow.receive_amount = receive_amount;
        escrow.bump = bump;
    }
}
