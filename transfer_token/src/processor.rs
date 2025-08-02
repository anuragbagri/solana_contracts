use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

use borsh::{BorshDeserialize, BorshSerialize};

use crate::instructions::{create_mint, mint_nft, mint_token, transfer_token};

enum Instructions {
    Create(CreateTokenArgs),
    MintNft,
    MintSpl(SplTokenArgs),
    Transfer(TransferTokenArgs),
}

pub fn process_instruction(
    pubkey: &Pubkey,
    accounts: &[AccountInfo],
    instructions_data: &[u8],
) -> ProgramResult {
    let instructions = Instructions::try_from_slice(instructions_data);

    match instructions {
        Instructions::Create(args) => create_mint(accounts, args),
        Instructions::MintNft => mint_nft(accounts),
        Instructions::MintSpl(args) => mint_token(accounts, args),
        Instructions::transfer(args) => transfer_token(accounts, args),
    }
}
