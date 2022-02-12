use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{pubkey::Pubkey, account_info::{AccountInfo, next_account_info}, entrypoint::ProgramResult, msg, program_error::ProgramError};

use crate::{models::{AccountState, Issue}, entrypoint::{get_initial_status, get_initial_validator_status}};
use std::{io::ErrorKind::InvalidData, cell::{RefMut, RefCell}, ops::Add};



pub fn process_save_issue(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;
    let validatorAccount = next_account_info(accounts_iter)?;

    msg!("Save issue transaction received");

    let mut existing_data_messages = match <Vec<Issue>>::try_from_slice(&account.data.borrow_mut()) {
        Ok(data) => data,
        Err(err) => {
            if err.kind() == InvalidData {
                msg!("InvalidData so initializing account data {:?}", err);
                get_initial_status()
            } else {
                panic!("Unknown error decoding account data {:?}", err)
            }
        }
    };

    let mut existing_validator_assigned_accounts = match <Vec<String>>::try_from_slice(&validatorAccount.data.borrow_mut()) {
        Ok(data) => data,
        Err(err) => {
            if err.kind() == InvalidData {
                msg!("InvalidData so initializing account data");
                get_initial_validator_status()
            } else {
                panic!("Unknown error decoding account data {:?}", err)
            }
        }
    };
    let issue = Issue::try_from_slice(data).map_err(|err| {
        msg!("Attempt to deserialize instruction data has failed. {:?}", err);
        ProgramError::InvalidInstructionData
    })?;

    msg!("Body: {:?}", issue);

    msg!("Issues: {:?}", existing_data_messages.len());

    existing_data_messages.push(issue);

    existing_data_messages.serialize(&mut &mut account.data.borrow_mut()[..])?;

    msg!("Issues Saved: {:?}", existing_data_messages.len());

    existing_validator_assigned_accounts.push(account.key.to_string());

    existing_validator_assigned_accounts.serialize(&mut &mut validatorAccount.data.borrow_mut()[..])?;
    
    Ok(())
}

pub fn process_accept_issue(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;
    let validator_account = next_account_info(accounts_iter)?;

    msg!("Accept issue transaction received");

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

    let issue = Issue::try_from_slice(data).map_err(|err| {
        msg!("Attempt to deserialize instruction data has failed. {:?}", err);
        ProgramError::InvalidInstructionData
    })?;

    if validator_account.lamports() < issue.reward {
        panic!("Validator has not enough lamports")
    }
    
    let validator_lamports = validator_account.lamports.borrow_mut().to_owned();
    **validator_account.lamports.borrow_mut() = validator_lamports - issue.reward;
    let account_lamports = account.lamports.borrow_mut().to_owned();
    **account.lamports.borrow_mut() = account_lamports + issue.reward;
    Ok(())
}