use serde::{Deserialize, Serialize};
use crate::bus::BusTransaction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Info,
    Warning,
    Error,
    Fatal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TelemetryEvent {
    /// A bus transaction captured during co-simulation
    BusTx(BusTransaction),
    /// Instruction trace verification event (RTL vs Reference Model)
    InstructionTrace {
        pc: u32,
        instruction: u32,
        mnemonic: String,
    },
    /// DKMS driver lifecycle or kernel compliance event
    KernelCompliance {
        severity: Severity,
        message: String,
    },
    /// Hardware assertion or scoreboard failure
    AssertionFailure {
        description: String,
        cycle: u64,
    },
}
