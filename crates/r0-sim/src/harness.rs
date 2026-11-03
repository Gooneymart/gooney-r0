// crates/r0-sim/src/harness.rs

use crate::bridge::*;
use std::os::raw::c_void;
use r0_core::bus::{BusTransaction, TxType};

pub struct SimHarness {
    handle: *mut c_void,
    current_cycle: u64,
}

impl SimHarness {
    pub fn new(base_addr: u64) -> Self {
        let handle = unsafe { sim_init(base_addr) };
        Self {
            handle,
            current_cycle: 0,
        }
    }

    pub fn step(&mut self, cycles: u64) {
        unsafe {
            sim_step(self.handle, cycles);
        }
        self.current_cycle += cycles;
    }

    pub fn execute_transaction(&mut self, tx: &BusTransaction) -> u32 {
        match tx.tx_type {
            TxType::Read => {
                unsafe { sim_read_mem(self.handle, tx.address) }
            }
            TxType::Write => {
                unsafe { sim_write_mem(self.handle, tx.address, tx.data); }
                0
            }
        }
    }

    pub fn cycle(&self) -> u64 {
        self.current_cycle
    }
}

impl Drop for SimHarness {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe {
                sim_destroy(self.handle);
            }
        }
    }
}
