use crate::models::Issue;

pub const ISSUES_LIST_SIZE: u32 = 20;
pub const DUMMY_STRING: &str  = "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
pub const DUMMY__STATE_STRING: &str  = "0000000000000000";

pub fn get_initial_status() -> Vec<Issue> {
    let mut issues = Vec::new();
    /*let newAccount = AccountState {
        issues: issues
    };*/
    //let issue = get_dummy_issue();
    for _ in 0..ISSUES_LIST_SIZE {
        issues.push(get_dummy_issue());
    };
    return issues;
}

pub fn get_dummy_issue() -> Issue {
    let dummy_issue = Issue {
        title: DUMMY_STRING.to_string(),
        description: DUMMY_STRING.to_string(),
        reward: 0,
        issue_type: DUMMY__STATE_STRING.to_string(),
        state: DUMMY__STATE_STRING.to_string(),
        attachments: Vec::new()
    };
    return dummy_issue;
}

/* pub fn get_dummy_string(length: u32) -> String {
    let mut dummy_string: String = "".to_string();
    for _ in 0..length {
        dummy_string.push_str("0");
    }
    return  dummy_string;
} */

pub fn get_initial_validator_status() -> Vec<String> {
    let addresses = Vec::new();
    return addresses;
}