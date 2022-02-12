use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{pubkey::Pubkey, account_info::{AccountInfo, next_account_info}, entrypoint::ProgramResult, msg, program_error::ProgramError};

use std::{io::ErrorKind::InvalidData};

use crate::{models::Issue, builder::{get_initial_status, get_initial_validator_status, DUMMY_STRING}};



pub fn process_save_issue(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;
    if account.owner != _program_id {
        msg!("This account {} is not owned by this program {} and cannot be updated!", account.key, _program_id);
    }
    let validator_account = next_account_info(accounts_iter)?;

    msg!("Save issue transaction received");

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

    let mut existing_validator_assigned_accounts = match <Vec<String>>::try_from_slice(&validator_account.data.borrow_mut()) {
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

    //msg!("Body: {:?}", issue);

    let index = existing_data_messages.iter().position(|p| p.title == String::from(DUMMY_STRING)).unwrap();
    msg!("Index: {:?}", index);
    existing_data_messages[index] = issue;
    msg!("Index2: {:?}", existing_data_messages.iter().position(|p| p.title == String::from(DUMMY_STRING)).unwrap());
    /* msg!("saving: {:?}", existing_data_messages[index]);
    msg!("{:?}", existing_data_messages[index+1]); */
    
    /* let updated_data = existing_data_messages.try_to_vec().expect("Failed to encode data.");
    //let data = &mut &mut account.data.borrow_mut();
    (&mut &mut account.data.borrow_mut())[..(updated_data.len())].copy_from_slice(&updated_data);
 */
    existing_data_messages.serialize(&mut &mut account.data.borrow_mut()[..])?;

    existing_validator_assigned_accounts.push(account.key.to_string());

    existing_validator_assigned_accounts.serialize(&mut &mut validator_account.data.borrow_mut()[..])?;
    
    Ok(())
}

pub fn process_accept_issue(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;
    if account.owner != _program_id {
        msg!("This account {} is not owned by this program {} and cannot be updated!", account.key, _program_id);
    }
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

    let reward = issue.reward.to_owned();

    let index = existing_data_messages.iter().position(|p| p.title == issue.title).unwrap();
    existing_data_messages[index] = issue;
    let updated_data = existing_data_messages.try_to_vec().expect("Failed to encode data.");
    (&mut &mut account.data.borrow_mut())[..(updated_data.len())].copy_from_slice(&updated_data);
    
    let validator_lamports = validator_account.lamports.borrow_mut().to_owned();
    **validator_account.lamports.borrow_mut() = validator_lamports - reward;
    let account_lamports = account.lamports.borrow_mut().to_owned();
    **account.lamports.borrow_mut() = account_lamports + reward;
    Ok(())
}