#![cfg(not(feature = "no-entrypoint"))]

use borsh::{ BorshDeserialize, BorshSerialize };
use solana_program::{
    log::sol_log_compute_units,
    account_info::{ next_account_info, AccountInfo },
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use std::io::ErrorKind::InvalidData;

use crate::models::{*};

pub fn get_initial_status() -> Vec<Issue> {
    let issues = Vec::new();
    return issues;
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;
    if account.owner != program_id {
        msg!("This account {} is not owned by this program {} and cannot be updated!", account.key, program_id);
    }

    let instruction_data_message = Issue::try_from_slice(instruction_data).map_err(|err| {
        msg!("Attempt to deserialize instruction data has failed. {:?}", err);
        ProgramError::InvalidInstructionData
    })?;
    msg!("Instruction_data message object {:?}", instruction_data_message);

    let mut existing_data_messages = match <Vec<Issue>>::try_from_slice(&account.data.borrow_mut()) {

        Ok(data) => data,
        Err(err) => {
            if err.kind() == InvalidData {
                msg!("InvalidData so initializing account data");
                get_initial_status()
            } else {
                panic!("Unknown error decoding account data {:?}", err)
            }
        }
    };

    existing_data_messages.push(instruction_data_message);
    
    Ok(())
}