use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};
use pinocchio_token::state::TokenAccount;

use crate::{
    constants::ESCROW_SEED,
    state::{load_ix_data, DataLen, Escrow},
};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MakeIxData {
    pub receive_amount: u64,
    pub bump: u8,
}

impl DataLen for MakeIxData {
    const LEN: usize = core::mem::size_of::<MakeIxData>();
}

pub fn process_make(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [maker, mint_a, mint_b, maker_ata_a, vault, escrow, _system_program, _token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !maker.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let ix_data = load_ix_data::<MakeIxData>(data)?;

    // Validate escrow account.
    let escrow_pda = pubkey::create_program_address(
        &[
            ESCROW_SEED.as_bytes(),
            maker.key().as_ref(),
            &[ix_data.bump],
        ],
        &crate::ID,
    )?;
    if escrow.key() != &escrow_pda {
        return Err(ProgramError::InvalidAccountData);
    }

    // Validate vault owner.
    assert!(TokenAccount::from_account_info(vault).unwrap().owner() == escrow.key());

    // Create escrow account.
    pinocchio_system::instructions::CreateAccount {
        from: maker,
        to: escrow,
        space: Escrow::LEN as u64,
        lamports: Rent::get()?.minimum_balance(Escrow::LEN),
        owner: &crate::ID,
    }
    .invoke()?;

    // Initialize escrow account.
    Escrow::initialize(
        escrow,
        *maker.key(),
        *mint_a.key(),
        *mint_b.key(),
        ix_data.receive_amount,
        ix_data.bump,
    );

    // Transfer tokens to vault.
    pinocchio_token::instructions::Transfer {
        from: maker_ata_a,
        to: vault,
        authority: maker,
        amount: ix_data.receive_amount,
    }
    .invoke()?;

    Ok(())
}
