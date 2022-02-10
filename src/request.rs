use borsh::{BorshSerialize, BorshDeserialize};

use crate::models::Issue;



#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum Request {
    SaveIssue {
        issue: Issue,
    },
    AcceptIssue {
        address: String,
        index: u64,
        amount: u64,
    }
}