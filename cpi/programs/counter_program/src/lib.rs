use borsh::{BorshDeserialize , BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
};

#[derive{BorshDeserialize , BorshSerialize}]; 

entrypoint!(process_instruction); 

struct Counter {
    count : u32,
}

enum Instruction {
    Increase {amount : u32},
    Decrease {amount : u3},
}


fn process_instruction(
    program_id : &Pubkey,
    accounts : &[AccountInfo],
    instruction_data : &[u8]
) -> ProgramResult {
    let mut account_iter = accounts.iter();
    let instruction = Instruction::try_from_slice(instruction_data)?;

    let account = next_account_info(&mut account_iter)?;
    let data = Counter::try_from_slice(account)?:

    match instruction {
        Instruction::Increase { amount } => {
            Counter.count = Counter.count + amount;
        }
        Instruction::Decrease { amount } => {
            Counter.count = Counter.count - amount;
        }    
    };
    Ok(())
}