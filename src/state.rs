use primitive_types::U256;

use super::types::EthereumAddress;
use super::stack::Stack;
use super::memory::Memory;
use super::storage::Storage;

pub struct EvmState {
  pc: u32,
  stack: Stack,
  memory: Memory,
  storage: Storage,
  sender: EthereumAddress,
  program: Vec<u8>, // TODO create a custom bytecode type
  gas: u64,
  value: U256,
  calldata: Vec<u8>,
  stop_flag: bool,
  revert_flag: bool,
  return_data: Vec<u8>,
  logs: Vec<u8>
}

impl EvmState {
  pub fn new(sender: EthereumAddress, program: Vec<u8>, gas: u64, value: U256, calldata: Vec<u8>) -> Self {
    let mut pc = 0;
    let stack = Stack::new();
    let memory = Memory::new();
    let storage = Storage::new();

    let mut stop_flag = false;
    let mut revert_flag = false;

    let mut return_data = Vec::new();
    let mut logs = Vec::new();

    Self {
      pc,
      stack,
      memory,
      storage,
      sender,
      program,
      gas,
      value,
      calldata,
      stop_flag,
      revert_flag,
      return_data,
      logs
    }
  }

  fn gas_dec(&mut self, gas_amount: u64) {
    self.gas -= gas_amount;
  }

  pub fn stop(&mut self) {
    self.stop_flag = true;
  }

  pub fn add(&mut self) {
    let (a, b) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
    let _ = self.stack.push(a + b);
    self.pc += 1;
    self.gas_dec(3);
  }

  pub fn sub(&mut self) {
    let (a, b) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
    let _ = self.stack.push(a - b);
    self.pc += 1;
    self.gas_dec(3);
  }

  pub fn mul(&mut self) {
    let (a, b) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
    let _ = self.stack.push(a * b);
    self.pc += 1;
    self.gas_dec(5);
  }

  pub fn div(&mut self) {
    let (a, b) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
    let _ = self.stack.push(if b == U256::zero() { U256::zero() } else { a / b });
    self.pc += 1;
    self.gas_dec(5);
  }

  fn pos_or_neg(value: U256) -> i32 {
    if value.bit(255) { -1 } else { 1 }
  }

  pub fn sdiv(&mut self) {
    let (a, b) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
    let sign = Self::pos_or_neg(a) * Self::pos_or_neg(b);
    
    let result = if b == U256::zero() { U256::zero() } else {
      let abs_a = if a.bit(255) { !a + U256::one() } else { a };
      let abs_b = if b.bit(255) { !b + U256::one() } else { b };
      let abs_result = abs_a / abs_b;
      if sign < 0 {
        !abs_result + U256::one()
      } else {
        abs_result
      }
    };
    let _ = self.stack.push(result);
    self.pc += 1;
    self.gas_dec(5);
  }

  pub fn modulo(&mut self) {
    let (a, b) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
    let _ = self.stack.push(if b == U256::zero() { U256::zero() } else { a % b });
    self.pc += 1;
    self.gas_dec(5);
  }

  pub fn smod(&mut self) {
    let (a, b) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
    let sign = Self::pos_or_neg(a) * Self::pos_or_neg(b);
    let result = if b == U256::zero() { U256::zero() } else {
      let abs_a = if a.bit(255) { !a + U256::one() } else { a };
      let abs_b = if b.bit(255) { !b + U256::one() } else { b };
      let abs_result = abs_a % abs_b;
      if sign < 0 {
        !abs_result + U256::one()
      } else {
        abs_result
      }
    };
    let _ = self.stack.push(result);
    self.pc += 1;
    self.gas_dec(5);
  }

  pub fn addmod(&mut self) {
    let (a, b) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
    let n = self.stack.pop().unwrap();
    let _ = self.stack.push((a + b) % n);
    self.pc += 1;
    self.gas_dec(8);
  }

  pub fn mulmod(&mut self) {
    let (a, b) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
    let n = self.stack.pop().unwrap();
    let _ = self.stack.push((a * b) % n);
    self.pc += 1;
    self.gas_dec(8);
  }

  // fn size_in_bytes()



}