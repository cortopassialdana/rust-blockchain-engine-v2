use super::smart_vm::{SmartVM, VMOp};

pub struct ContractLoader;

impl ContractLoader {
    pub fn load_from_bytes(code: &[u8]) -> Vec<VMOp> {
        let mut ops = Vec::new();
        for &byte in code {
            match byte {
                0x00 => ops.push(VMOp::Nop),
                0x01 => ops.push(VMOp::Push(0)),
                0x02 => ops.push(VMOp::Add),
                0x03 => ops.push(VMOp::Sub),
                _ => {}
            }
        }
        ops
    }

    pub fn run_contract(vm: &mut SmartVM, code: &[u8]) -> Result<(), String> {
        let ops = Self::load_from_bytes(code);
        vm.execute(&ops)
    }
}
