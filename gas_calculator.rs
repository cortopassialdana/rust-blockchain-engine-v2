use super::smart_vm::VMOp;

pub struct GasCalculator;

impl GasCalculator {
    pub const BASE_TX: u64 = 21000;
    pub const PER_BYTE: u64 = 68;
    pub const PER_OP: u64 = 3;

    pub fn calculate_transaction(data: &[u8]) -> u64 {
        Self::BASE_TX + (data.len() as u64) * Self::PER_BYTE
    }

    pub fn calculate_contract(ops: &[VMOp]) -> u64 {
        (ops.len() as u64) * Self::PER_OP
    }

    pub fn calculate_total(tx_data: &[u8], ops: &[VMOp]) -> u64 {
        Self::calculate_transaction(tx_data) + Self::calculate_contract(ops)
    }
}
