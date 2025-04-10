use mollusk_svm::{program, Mollusk};
use mollusk_svm_bencher::MolluskComputeUnitBencher;
use solana_pinocchio_starter::ID;
use solana_sdk::pubkey;
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
};

pub const PROGRAM: Pubkey = Pubkey::new_from_array(ID);

pub const RENT: Pubkey = pubkey!("SysvarRent111111111111111111111111111111111");

pub const PAYER: Pubkey = pubkey!("41LzznNicELmc5iCR9Jxke62a3v1VhzpBYodQF5AQwHX");

fn main() {
    let mollusk = Mollusk::new(&PROGRAM, "target/deploy/solana_pinocchio_starter");

    let (system_program, system_account) = program::keyed_account_for_system_program();

    // TODO
}
