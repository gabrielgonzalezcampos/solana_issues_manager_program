#![cfg(not(feature = "no-entrypoint"))]

use borsh::{ BorshDeserialize };
use solana_program::{
    account_info::{ next_account_info, AccountInfo },
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{request::Request, processors::process_accept_issue};

use crate::{models::{*}, processors::process_save_issue};

pub fn get_initial_status() -> AccountState {
    let issues = Vec::new();
    let newAccount = AccountState {
        issues: issues
    };
    return newAccount;
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

    let instruction = Request::try_from_slice(instruction_data).map_err(|err| {
        msg!("Attempt to deserialize instruction data has failed. {:?}", err);
        ProgramError::InvalidInstructionData
    })?;

    match instruction {
        Request::SaveIssue {issue} => {
            process_save_issue(program_id, accounts, issue)
        }

        Request::AcceptIssue { address, index, amount } => {
            return process_accept_issue(program_id, accounts)
        }
    }
}