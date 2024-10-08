use primitive_types::U256;

use crate::opcodes::OPCODE;

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
  program: Vec<U256>, // TODO create a custom bytecode type
  gas: u64,
  value: U256,
  calldata: Vec<u8>,
  stop_flag: bool,
  revert_flag: bool,
  return_data: Vec<U256>,
  logs: Vec<u8>
}

impl EvmState {
  pub fn new(sender: EthereumAddress, program: Vec<U256>, gas: u64, value: U256, calldata: Vec<u8>) -> Self {
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

  fn should_execute_next_opcode(&self) -> bool {
    if self.pc >= self.program.len() as u32 || self.revert_flag || self.stop_flag {
      return false
    } else {
      return true
    }
  }

  pub fn run(&mut self) {
    
    while self.should_execute_next_opcode() {
      
      let op: OPCODE = self.peek().try_into().unwrap();
      println!("{:?}", &op);
      match op {
          OPCODE::STOP => {
            self.stop()
          },
          OPCODE::PUSH1 => {
            self.push()
          },
          OPCODE::ADD => {
            self.add()
          },
          OPCODE::SUB => {
            self.sub()
          },
          OPCODE::MUL => {
            self.mul()
          },
          OPCODE::DIV => {
            self.div()
          },
          OPCODE::SDIV => {
            self.sdiv()
          },
          OPCODE::ADDMOD => {
            self.addmod()
          },
          OPCODE::MULMOD => {
            self.mulmod()
          },
          OPCODE::EXP => {
            self.exp()
          },
          OPCODE::SIGNEXTEND => {
            self.sigextend()
          },
          OPCODE::LT => {
            self.lt()
          },
          OPCODE::GT => {
            self.gt()
          },
          OPCODE::SLT => {
            self.slt()
          },
          OPCODE::SGT => {
            self.sgt()
          },
          OPCODE::EQ => {
            self.eq()
          },
          OPCODE::ISZERO => {
            self.iszero()
          },
          OPCODE::AND => {
            self.and()
          },
          OPCODE::OR => {
            self.or()
          },
          OPCODE::XOR => {
            self.xor()
          },
          OPCODE::NOT => {
            self.not()
          },
          OPCODE::BYTE => {
            self.byte()
          },
          OPCODE::SHL => {
            self.shl()
          },
          OPCODE::SHR => {
            self.shr()
          },
          OPCODE::SAR => {
            self.sar()
          },
          
          _ => todo!("Implement other opcodes")
  
      }
    }

    // unimplemented!()
  }


  fn gas_dec(&mut self, gas_amount: u64) {
    self.gas -= gas_amount;
  }

  pub fn push(&mut self) {
    self.pc += 1;
    let value = self.peek();
    let _ = self.stack.push(value);
    self.pc += 1;
    self.gas_dec(3);
  }

  pub fn peek(&self) -> U256 {
    self.program[self.pc as usize]
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

  // Logic ops
  pub fn and(&mut self) {
    let (a, b) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
    let _ = self.stack.push(a & b);
    self.pc += 1;
    self.gas_dec(3);
  }

  pub fn or(&mut self) {
    let (a, b) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
    let _ = self.stack.push(a | b);
    self.pc += 1;
    self.gas_dec(3);
  }

  pub fn xor(&mut self) {
    let (a, b) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
    let _ = self.stack.push(a ^ b);
    self.pc += 1;
    self.gas_dec(3);
  }

  pub fn not(&mut self) {
    let a = self.stack.pop().unwrap();
    let _ = self.stack.push(!a);
    self.pc += 1;
    self.gas_dec(3);
  }

  // Bit ops

  pub fn byte(&mut self) {
    let (i, x) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
    let result = if i > U256::from(32) {
      U256::zero()
    } else {
      (x / U256::from(256).pow(U256::from(31) - i)) % U256::from(256)
    };
    let _ = self.stack.push(result);
    self.pc += 1;
    self.gas_dec(3);
  }

  pub fn shl(&mut self) { // shift left
    let (shift, value) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
    let _ = self.stack.push(value << shift);
    self.pc += 1;
    self.gas_dec(3);
  }

  pub fn shr(&mut self) { // shift right
    let (shift, value) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
    let _ = self.stack.push(value >> shift);
    self.pc += 1;
    self.gas_dec(3);
  }

  pub fn sar(&mut self) { // signed shift right
    let (shift, value) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
    let result = if shift >= U256::from(256) {
      if value.bit(255) { U256::MAX >> 1 } else { U256::zero() }
    } else {
      if value.bit(255) {
        (value >> shift.as_u32()) | (U256::max_value() << (256 - shift.as_u32()))
      } else {
        value >> shift.as_u32()
      }
    };
    let _ = self.stack.push(result);
    self.pc += 1;
    self.gas_dec(3);
  }



}