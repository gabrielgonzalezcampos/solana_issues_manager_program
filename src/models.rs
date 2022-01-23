use borsh::{ BorshDeserialize, BorshSerialize };
use solana_program::{
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Request {
    pub endpoint: String,
    pub requestData: String
}

pub enum IssueType {
    thrash,
    road
}

pub enum IssueState {
    processing,
    uploaded,
    accepted,
    solving,
    solved,
    rejected,
    error
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Issue {
    pub title: String,
    pub description: String,
    pub reward: int,
    pub issueType: IssueType,
    pub state: IssueState
    pub attachments: Vec<String>
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct AccountState {
    pub issues: Vec<Issue>
}