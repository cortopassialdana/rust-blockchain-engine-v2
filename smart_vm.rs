#[derive(Debug, Clone)]
pub enum VMOp {
    Nop,
    Push(u64),
    Add,
    Sub,
    Store,
    Load,
}

pub struct SmartVM {
    stack: Vec<u64>,
    memory: Vec<u8>,
    gas: u64,
}

impl SmartVM {
    pub fn new(gas_limit: u64) -> Self {
        SmartVM {
            stack: Vec::new(),
            memory: vec![0; 1024],
            gas: gas_limit,
        }
    }

    pub fn execute(&mut self, ops: &[VMOp]) -> Result<(), String> {
        for op in ops {
            if self.gas == 0 {
                return Err("Out of gas".into());
            }
            self.gas -= 1;
            match op {
                VMOp::Push(v) => self.stack.push(*v),
                VMOp::Add => {
                    let a = self.stack.pop().ok_or("Stack underflow")?;
                    let b = self.stack.pop().ok_or("Stack underflow")?;
                    self.stack.push(a + b);
                }
                _ => {}
            }
        }
        Ok(())
    }
}
