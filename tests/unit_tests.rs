use mollusk_svm::result::Check;
use mollusk_svm::{program, Mollusk};
use pinocchio_escrow::instruction::MakeIxData;
use pinocchio_escrow::state::{to_bytes, DataLen, Escrow};
use solana_sdk::account::Account;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
extern crate alloc;
use alloc::vec;

use solana_sdk::{account::WritableAccount, program_option::COption, program_pack::Pack};
use spl_token::state::AccountState;

use pinocchio_escrow::ID;

pub const PROGRAM: Pubkey = Pubkey::new_from_array(ID);

pub const RENT: Pubkey = pubkey!("SysvarRent111111111111111111111111111111111");

pub const PAYER: Pubkey = pubkey!("41LzznNicELmc5iCR9Jxke62a3v1VhzpBYodQF5AQwHX");

pub const DEPOSIT_AMOUNT: u64 = 10;
pub const RECEIVE_AMOUNT: u64 = 9;

pub fn mollusk() -> Mollusk {
    let mut mollusk = Mollusk::new(&PROGRAM, "target/deploy/pinocchio_escrow");
    mollusk.add_program(
        &spl_token::ID,
        "tests/elfs/spl_token",
        &mollusk_svm::program::loader_keys::LOADER_V3,
    );
    mollusk
}

#[test]
fn test_make() {
    let mollusk = mollusk();

    let (system_program, system_account) = mollusk_svm::program::keyed_account_for_system_program();
    let (token_program, token_account) = (
        spl_token::ID,
        program::create_program_account_loader_v3(&spl_token::ID),
    );

    let maker = Pubkey::new_from_array([0x01; 32]);
    let maker_account = Account::new(1 * LAMPORTS_PER_SOL, 0, &system_program);

    let (escrow, escrow_bump) = solana_sdk::pubkey::Pubkey::find_program_address(
        &[(b"escrow"), &maker.to_bytes()],
        &PROGRAM,
    );
    let escrow_account = Account::new(0, 0, &system_program);

    let mint_x = Pubkey::new_from_array([0x02; 32]);
    let mut mint_x_account = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Mint::LEN),
        spl_token::state::Mint::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Mint {
            mint_authority: COption::None,
            supply: 100_000_000,
            decimals: 6,
            is_initialized: true,
            freeze_authority: COption::None,
        },
        mint_x_account.data_as_mut_slice(),
    )
    .unwrap();

    let mint_y = Pubkey::new_from_array([0x03; 32]);
    let mut mint_y_account = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Mint::LEN),
        spl_token::state::Mint::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Mint {
            mint_authority: COption::None,
            supply: 100_000_000,
            decimals: 6,
            is_initialized: true,
            freeze_authority: COption::None,
        },
        mint_y_account.data_as_mut_slice(),
    )
    .unwrap();

    let maker_ata = Pubkey::new_from_array([0x04; 32]);
    let mut maker_ata_account = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN),
        spl_token::state::Account::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Account {
            mint: mint_x,
            owner: maker,
            amount: 100_000_000,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        },
        maker_ata_account.data_as_mut_slice(),
    )
    .unwrap();

    let vault = Pubkey::new_from_array([0x05; 32]);
    let mut vault_account = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN),
        spl_token::state::Account::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Account {
            mint: mint_x,
            owner: escrow,
            amount: 0,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        },
        vault_account.data_as_mut_slice(),
    )
    .unwrap();

    // Create the instruction data
    let ix_data = MakeIxData {
        deposit_amount: DEPOSIT_AMOUNT,
        receive_amount: RECEIVE_AMOUNT,
        bump: escrow_bump,
    };

    // Ix discriminator = 0
    let mut ser_ix_data = vec![0];

    // Serialize the instruction data
    ser_ix_data.extend_from_slice(unsafe { to_bytes(&ix_data) });

    let instruction = Instruction::new_with_bytes(
        PROGRAM,
        &ser_ix_data,
        vec![
            AccountMeta::new(maker, true),
            AccountMeta::new_readonly(mint_x, false),
            AccountMeta::new_readonly(mint_y, false),
            AccountMeta::new(maker_ata, false),
            AccountMeta::new(vault, false),
            AccountMeta::new(escrow, true),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(token_program, false),
        ],
    );

    mollusk.process_and_validate_instruction(
        &instruction,
        &vec![
            (maker, maker_account),
            (mint_x, mint_x_account),
            (mint_y, mint_y_account),
            (maker_ata, maker_ata_account),
            (vault, vault_account),
            (escrow, escrow_account),
            (system_program, system_account),
            (token_program, token_account),
        ],
        &[Check::success()],
    );
}

#[test]
fn test_take() {
    let mollusk = mollusk();

    let (system_program, system_account) = mollusk_svm::program::keyed_account_for_system_program();
    let (token_program, token_account) = (
        spl_token::ID,
        program::create_program_account_loader_v3(&spl_token::ID),
    );

    let taker = Pubkey::new_from_array([0x01; 32]);
    let taker_account = Account::new(1 * LAMPORTS_PER_SOL, 0, &system_program);

    let maker = Pubkey::new_from_array([0x02; 32]);
    let maker_account = Account::new(1 * LAMPORTS_PER_SOL, 0, &system_program);

    let (escrow, escrow_bump) = solana_sdk::pubkey::Pubkey::find_program_address(
        &[(b"escrow"), &maker.to_bytes()],
        &PROGRAM,
    );

    let mint_x = Pubkey::new_from_array([0x03; 32]);
    let mut mint_x_account = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Mint::LEN),
        spl_token::state::Mint::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Mint {
            mint_authority: COption::None,
            supply: 100_000_000,
            decimals: 6,
            is_initialized: true,
            freeze_authority: COption::None,
        },
        mint_x_account.data_as_mut_slice(),
    )
    .unwrap();

    let mint_y = Pubkey::new_from_array([0x04; 32]);
    let mut mint_y_account = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Mint::LEN),
        spl_token::state::Mint::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Mint {
            mint_authority: COption::None,
            supply: 100_000_000,
            decimals: 6,
            is_initialized: true,
            freeze_authority: COption::None,
        },
        mint_y_account.data_as_mut_slice(),
    )
    .unwrap();

    let taker_ata_x = Pubkey::new_from_array([0x05; 32]);
    let mut taker_ata_x_account = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN),
        spl_token::state::Account::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Account {
            mint: mint_x,
            owner: taker,
            amount: 0,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        },
        taker_ata_x_account.data_as_mut_slice(),
    )
    .unwrap();

    let taker_ata_y = Pubkey::new_from_array([0x06; 32]);
    let mut taker_ata_y_account = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN),
        spl_token::state::Account::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Account {
            mint: mint_y,
            owner: taker,
            amount: 100_000_000,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        },
        taker_ata_y_account.data_as_mut_slice(),
    )
    .unwrap();

    let maker_ata_y = Pubkey::new_from_array([0x07; 32]);
    let mut maker_ata_y_account = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN),
        spl_token::state::Account::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Account {
            mint: mint_y,
            owner: maker,
            amount: 0,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        },
        maker_ata_y_account.data_as_mut_slice(),
    )
    .unwrap();

    let vault = Pubkey::new_from_array([0x08; 32]);
    let mut vault_account = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN),
        spl_token::state::Account::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Account {
            mint: mint_x,
            owner: escrow,
            amount: DEPOSIT_AMOUNT,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        },
        vault_account.data_as_mut_slice(),
    )
    .unwrap();

    let mut escrow_account = Account::new(
        mollusk.sysvars.rent.minimum_balance(Escrow::LEN),
        Escrow::LEN,
        &PROGRAM.into(),
    );
    let escrow_state = Escrow {
        is_initialized: true,
        maker: *maker.as_array(),
        mint_a: *mint_x.as_array(),
        mint_b: *mint_y.as_array(),
        receive_amount: RECEIVE_AMOUNT,
        bump: escrow_bump,
    };
    escrow_account.data = unsafe { to_bytes(&escrow_state).to_vec() };

    // Create the instruction data
    // Ix discriminator = 1
    let ser_ix_data = vec![1];

    let instruction = Instruction::new_with_bytes(
        PROGRAM,
        &ser_ix_data,
        vec![
            AccountMeta::new(taker, true),
            AccountMeta::new(maker, true),
            AccountMeta::new_readonly(mint_x, false),
            AccountMeta::new_readonly(mint_y, false),
            AccountMeta::new(taker_ata_x, false),
            AccountMeta::new(taker_ata_y, false),
            AccountMeta::new(maker_ata_y, false),
            AccountMeta::new(vault, false),
            AccountMeta::new(escrow, true),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(token_program, false),
        ],
    );

    mollusk.process_and_validate_instruction(
        &instruction,
        &vec![
            (taker, taker_account),
            (maker, maker_account),
            (mint_x, mint_x_account),
            (mint_y, mint_y_account),
            (taker_ata_x, taker_ata_x_account),
            (taker_ata_y, taker_ata_y_account),
            (maker_ata_y, maker_ata_y_account),
            (vault, vault_account),
            (escrow, escrow_account),
            (system_program, system_account),
            (token_program, token_account),
        ],
        &[Check::success()],
    );
}

#[test]
fn test_refund() {
    let mollusk = mollusk();

    let (system_program, system_account) = mollusk_svm::program::keyed_account_for_system_program();
    let (token_program, token_account) = (
        spl_token::ID,
        program::create_program_account_loader_v3(&spl_token::ID),
    );

    let maker = Pubkey::new_from_array([0x01; 32]);
    let maker_account = Account::new(1 * LAMPORTS_PER_SOL, 0, &system_program);

    let (escrow, escrow_bump) = solana_sdk::pubkey::Pubkey::find_program_address(
        &[(b"escrow"), &maker.to_bytes()],
        &PROGRAM,
    );

    let mint_x = Pubkey::new_from_array([0x02; 32]);
    let mut mint_x_account = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Mint::LEN),
        spl_token::state::Mint::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Mint {
            mint_authority: COption::None,
            supply: 100_000_000,
            decimals: 6,
            is_initialized: true,
            freeze_authority: COption::None,
        },
        mint_x_account.data_as_mut_slice(),
    )
    .unwrap();

    let mint_y = Pubkey::new_from_array([0x03; 32]);
    let mut mint_y_account = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Mint::LEN),
        spl_token::state::Mint::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Mint {
            mint_authority: COption::None,
            supply: 100_000_000,
            decimals: 6,
            is_initialized: true,
            freeze_authority: COption::None,
        },
        mint_y_account.data_as_mut_slice(),
    )
    .unwrap();

    let maker_ata = Pubkey::new_from_array([0x04; 32]);
    let mut maker_ata_account = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN),
        spl_token::state::Account::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Account {
            mint: mint_x,
            owner: maker,
            amount: 100_000_000,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        },
        maker_ata_account.data_as_mut_slice(),
    )
    .unwrap();

    let vault = Pubkey::new_from_array([0x05; 32]);
    let mut vault_account = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN),
        spl_token::state::Account::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Account {
            mint: mint_x,
            owner: escrow,
            amount: DEPOSIT_AMOUNT,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        },
        vault_account.data_as_mut_slice(),
    )
    .unwrap();

    let mut escrow_account = Account::new(
        mollusk.sysvars.rent.minimum_balance(Escrow::LEN),
        Escrow::LEN,
        &PROGRAM.into(),
    );
    let escrow_state = Escrow {
        is_initialized: true,
        maker: *maker.as_array(),
        mint_a: *mint_x.as_array(),
        mint_b: *mint_y.as_array(),
        receive_amount: RECEIVE_AMOUNT,
        bump: escrow_bump,
    };
    escrow_account.data = unsafe { to_bytes(&escrow_state).to_vec() };

    // Create the instruction data
    // Ix discriminator = 2
    let ser_ix_data = vec![2];

    let instruction = Instruction::new_with_bytes(
        PROGRAM,
        &ser_ix_data,
        vec![
            AccountMeta::new(maker, true),
            AccountMeta::new_readonly(mint_x, false),
            AccountMeta::new_readonly(mint_y, false),
            AccountMeta::new(maker_ata, false),
            AccountMeta::new(vault, false),
            AccountMeta::new(escrow, true),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(token_program, false),
        ],
    );

    mollusk.process_and_validate_instruction(
        &instruction,
        &vec![
            (maker, maker_account),
            (mint_x, mint_x_account),
            (mint_y, mint_y_account),
            (maker_ata, maker_ata_account),
            (vault, vault_account),
            (escrow, escrow_account),
            (system_program, system_account),
            (token_program, token_account),
        ],
        &[Check::success()],
    );
}
