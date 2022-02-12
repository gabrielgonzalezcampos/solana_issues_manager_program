use solana_program::msg;

use crate::models::Issue;

const ISSUES_LIST_SIZE: u32 = 20;
const STRING_SIZE: u32 = 120;
const STATE_STRING_SIZE: u32 = 16;

pub fn get_initial_status() -> Vec<Issue> {
    let mut issues = Vec::new();
    msg!("initDATA");
    /*let newAccount = AccountState {
        issues: issues
    };*/
    //let issue = get_dummy_issue();
    for _ in 0..ISSUES_LIST_SIZE {
        issues.push(get_dummy_issue());
    };
    msg!("data initialized");
    return issues;
}

pub fn get_dummy_issue() -> Issue {
    let dummy_issue = Issue {
        title: get_dummy_string(STRING_SIZE),
        description: get_dummy_string(STRING_SIZE),
        reward: 0,
        issue_type: get_dummy_string(STATE_STRING_SIZE),
        state: get_dummy_string(STATE_STRING_SIZE),
        attachments: Vec::new()
    };
    return dummy_issue;
}

pub fn get_dummy_string(length: u32) -> String {
    let mut dummy_string: String = "".to_string();
    for _ in 0..length {
        dummy_string.push_str("0");
    }
    return  dummy_string;
}

pub fn get_initial_validator_status() -> Vec<String> {
    let addresses = Vec::new();
    return addresses;
}