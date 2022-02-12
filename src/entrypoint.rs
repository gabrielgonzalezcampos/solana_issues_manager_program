#![cfg(not(feature = "no-entrypoint"))]

use std::str::FromStr;

use borsh::{ BorshDeserialize};
use solana_program::{
    account_info::{ AccountInfo },
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{request::{Request, Endpoint}, processors::process_accept_issue};

use crate::{processors::process_save_issue};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    /* let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;
    if account.owner != program_id {
        msg!("This account {} is not owned by this program {} and cannot be updated!", account.key, program_id);
    } */

    let instruction = Request::try_from_slice(instruction_data).map_err(|err| {
        msg!("Attempt to deserialize instruction data has failed. {:?}", err);
        ProgramError::InvalidInstructionData
    })?;

    let endpoint = match Endpoint::from_str(&instruction.endpoint){
        Ok(data) => data,
        Err(_) => {
            panic!("Unknown endpoint")
        }
    };

    match endpoint {
        Endpoint::Save => {
            return process_save_issue(program_id, accounts, &instruction.body)
        }

        Endpoint::Accept => {
            return process_accept_issue(program_id, accounts, &instruction.body)
        }
    }
}