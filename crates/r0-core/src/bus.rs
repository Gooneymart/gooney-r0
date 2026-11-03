use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TxType {
    Read,
    Write,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusTransaction {
    pub timestamp_cycles: u64,
    pub tx_type: TxType,
    pub address: u32,
    pub data: u32,
    pub byte_enable: u8,
}

impl BusTransaction {
    pub fn new_read(cycles: u64, addr: u32) -> Self {
        Self {
            timestamp_cycles: cycles,
            tx_type: TxType::Read,
            address: addr,
            data: 0,
            byte_enable: 0xF,
        }
    }

    pub fn new_write(cycles: u64, addr: u32, data: u32) -> Self {
        Self {
            timestamp_cycles: cycles,
            tx_type: TxType::Write,
            address: addr,
            data,
            byte_enable: 0xF,
        }
    }
}
