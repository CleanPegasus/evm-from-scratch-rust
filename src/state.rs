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
  gas: u32,
  value: U256,
  calldata: Vec<u8>,
  stop_flag: bool,
  revert_flag: bool,
  return_data: Vec<u8>,
  logs: Vec<u8>
}

impl EvmState {
  pub fn new(sender: EthereumAddress, program: Vec<u8>, gas: u32, value: U256, calldata: Vec<u8>) -> Self {
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
}