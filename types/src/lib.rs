use serde::{Serialize, Deserialize};

pub type Address = [u8; 32];

#[derive(Debug, Serialize, Deserialize)]
pub enum TxKind {
    Deploy,
    Call,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub from: Address,
    pub nonce: u64,
    pub kind: TxKind,
    pub data: Vec<u8>,
}
