use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{pubkey::Pubkey, account_info::{AccountInfo, next_account_info}, entrypoint::ProgramResult, msg};

use crate::{models::{AccountState, Issue}, entrypoint::get_initial_status};
use std::io::ErrorKind::InvalidData;



pub fn process_save_issue(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    issue: Issue,
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

    existing_data_messages.issues.push(issue);

    existing_data_messages.serialize(&mut &mut account.data.borrow_mut()[..])?;
    
    Ok(())
}

pub fn process_accept_issue(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    Ok(())
}