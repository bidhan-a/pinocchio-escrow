use mollusk_svm::result::{Check, ProgramResult};
use mollusk_svm::{program, Mollusk};
use solana_sdk::account::Account;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
extern crate alloc;
use alloc::vec;

use solana_pinocchio_starter::ID;

pub const PROGRAM: Pubkey = Pubkey::new_from_array(ID);

pub const RENT: Pubkey = pubkey!("SysvarRent111111111111111111111111111111111");

pub const PAYER: Pubkey = pubkey!("41LzznNicELmc5iCR9Jxke62a3v1VhzpBYodQF5AQwHX");

pub fn mollusk() -> Mollusk {
    let mollusk = Mollusk::new(&PROGRAM, "target/deploy/solana_pinocchio_starter");
    mollusk
}

#[test]
fn test_make() {
    let mollusk = mollusk();

    //system program and system account
    let (system_program, system_account) = program::keyed_account_for_system_program();

    // TODO
}

#[test]
fn test_take() {
    let mollusk = mollusk();

    //system program and system account
    let (system_program, _system_account) = program::keyed_account_for_system_program();

    // TODO
}
