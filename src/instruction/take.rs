use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey, ProgramResult,
};
use pinocchio_token::state::TokenAccount;

use crate::{
    constants::ESCROW_SEED,
    state::{load_acc_mut_unchecked, Escrow},
};

pub fn process_take(accounts: &[AccountInfo]) -> ProgramResult {
    let [taker, maker, mint_a, mint_b, taker_ata_a, taker_ata_b, maker_ata_b, vault, escrow, _system_program, _token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !taker.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Load accounts.
    let escrow_account =
        unsafe { load_acc_mut_unchecked::<Escrow>(escrow.borrow_mut_data_unchecked())? };
    let vault_account = pinocchio_token::state::TokenAccount::from_account_info(vault)?;

    // Validate escrow account.
    let escrow_pda = pubkey::create_program_address(
        &[
            ESCROW_SEED.as_bytes(),
            maker.key().as_ref(),
            &[escrow_account.bump],
        ],
        &crate::ID,
    )?;
    if escrow.key() != &escrow_pda {
        return Err(ProgramError::InvalidAccountData);
    }

    // Validate vault owner.
    assert!(unsafe {
        TokenAccount::from_account_info_unchecked(vault)
            .unwrap()
            .owner()
            == escrow.key()
    });

    // Transfer token from taker to maker.
    pinocchio_token::instructions::Transfer {
        from: taker_ata_b,
        to: maker_ata_b,
        authority: taker,
        amount: escrow_account.receive_amount,
    }
    .invoke()?;

    // Transfer token from vault to taker.
    let bump = [escrow_account.bump];
    let seed = [
        Seed::from(b"escrow"),
        Seed::from(maker.key()),
        Seed::from(&bump),
    ];
    let seeds = Signer::from(&seed);

    pinocchio_token::instructions::Transfer {
        from: vault,
        to: taker_ata_a,
        authority: escrow,
        amount: vault_account.amount(),
    }
    .invoke_signed(&[seeds.clone()])?;

    // Close vault account.
    pinocchio_token::instructions::CloseAccount {
        account: vault,
        destination: maker,
        authority: escrow,
    }
    .invoke_signed(&[seeds])?;

    // Close escrow account.
    unsafe {
        *maker.borrow_mut_lamports_unchecked() += *escrow.borrow_lamports_unchecked();
        *escrow.borrow_mut_lamports_unchecked() = 0
    };

    Ok(())
}
