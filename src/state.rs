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

  // Math ops
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

  fn size_in_bytes(value: U256) -> usize {
    if value.is_zero() {
      return 1
    }
    let leading_zeros = value.leading_zeros();
    let bits_needed = (256 - leading_zeros) as usize;
    (bits_needed + 7) / 8
  }

  pub fn exp(&mut self) {
    let (a, exponent) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
    let _ = self.stack.push(a.pow(exponent));
    self.pc += 1;
    self.gas_dec(10 + (50 * Self::size_in_bytes(exponent) as u64))
  }

  pub fn sigextend(&mut self) {
    unimplemented!()
  }

  // comparison
  pub fn lt(&mut self) {
    let (a, b) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
    let _ = self.stack.push(if a < b {U256::one()} else {U256::zero()});
    self.pc += 1;
    self.gas_dec(3);
  }

  fn unsigned_to_signed(value: U256) -> i128 {
    if value > U256::from(i128::MAX as u128) {
        let twos_complement = (!value).overflowing_add(U256::from(1)).0;
        if twos_complement <= U256::from(i128::MAX as u128) {
            -(twos_complement.as_u128() as i128)
        } else {
            i128::MIN
        }
    } else {
        value.as_u128() as i128
    }
}

  pub fn slt(&mut self) {
    let (a, b) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
    let a_sig = Self::unsigned_to_signed(a);
    let b_sig = Self::unsigned_to_signed(b);
    let _ = self.stack.push(if a_sig < b_sig {U256::one()} else {U256::zero()});
    self.pc += 1;
    self.gas_dec(3);
  }

  pub fn gt(&mut self) {
    let (a, b) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
    let _ = self.stack.push(if a > b {U256::one()} else {U256::zero()});
    self.pc += 1;
    self.gas_dec(3);
  }

  pub fn sgt(&mut self) {
    let (a, b) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
    let a_sig = Self::unsigned_to_signed(a);
    let b_sig = Self::unsigned_to_signed(b);
    let _ = self.stack.push(if a_sig > b_sig {U256::one()} else {U256::zero()});
    self.pc += 1;
    self.gas_dec(3);
  }

  pub fn eq(&mut self) {
    let (a, b) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
    let _ = self.stack.push(if a == b {U256::one()} else {U256::zero()});
    self.pc += 1;
    self.gas_dec(3);
  }

  pub fn iszero(&mut self) {
    let a = self.stack.pop().unwrap();
    let _ = self.stack.push(if a == U256::zero() {U256::one()} else {U256::zero()});
    self.pc += 1;
    self.gas_dec(3);
  }


}