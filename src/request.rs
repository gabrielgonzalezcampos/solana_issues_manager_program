use borsh::{BorshSerialize, BorshDeserialize};
use strum_macros::EnumString;



#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Request {
    pub endpoint: String,
    pub body: Vec<u8>
}

extern crate strum;
#[derive(EnumString)]
pub enum Endpoint {
    Save,
    Accept
}