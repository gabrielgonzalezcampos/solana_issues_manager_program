use borsh::{ BorshDeserialize, BorshSerialize };
use solana_program::{
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Request {
    pub endpoint: String,
    pub request_data: String
}
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum IssueType {
    Thrash,
    Road
}
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum IssueState {
    Processing,
    Uploaded,
    Accepted,
    Solving,
    Solved,
    Rejected,
    Error
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Issue {
    pub title: String,
    pub description: String,
    pub reward: u64,
    pub issue_type: IssueType,
    pub state: IssueState,
    pub attachments: Vec<String>
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct AccountState {
    pub issues: Vec<Issue>
}