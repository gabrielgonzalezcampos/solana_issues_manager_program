use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{pubkey::Pubkey, account_info::{AccountInfo, next_account_info}, entrypoint::ProgramResult, msg, program_error::ProgramError};

use crate::{models::{AccountState, Issue}, entrypoint::get_initial_status};
use std::{io::ErrorKind::InvalidData, cell::{RefMut, RefCell}, ops::Add};



pub fn process_save_issue(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;
    let mut existing_data_messages = match AccountState::try_from_slice(&account.data.borrow_mut()) {
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

    let issue = Issue::try_from_slice(data).map_err(|err| {
        msg!("Attempt to deserialize instruction data has failed. {:?}", err);
        ProgramError::InvalidInstructionData
    })?;

    existing_data_messages.issues.push(issue);

    existing_data_messages.serialize(&mut &mut account.data.borrow_mut()[..])?;
    
    Ok(())
}

pub fn process_accept_issue(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;
    let validatorAccount = next_account_info(accounts_iter)?;

    let mut existing_data_messages = match AccountState::try_from_slice(&account.data.borrow_mut()) {
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

    let issue = Issue::try_from_slice(data).map_err(|err| {
        msg!("Attempt to deserialize instruction data has failed. {:?}", err);
        ProgramError::InvalidInstructionData
    })?;

    let validator_lamports = &validatorAccount.lamports.borrow_mut();

    if validatorAccount.lamports() < issue.reward {
        panic!("Validator has not enough lamports")
    }

    validatorAccount.lamports.borrow_mut().checked_sub(issue.reward);
    
    account.lamports.borrow_mut().checked_add(issue.reward);

    Ok(())
}