use evm_from_scratch_rust;

#[cfg(test)]
mod test {
  use evm_from_scratch_rust::{Stack, Memory, Storage, EvmState};
  use evm_from_scratch_rust::types::EthereumAddress;
  use primitive_types::U256;

  #[test]
  fn test_evm_state_creation() {
    let sender: EthereumAddress = [0_u8; 20].into();
    let program = vec![U256::from(0x60), U256::from(10), U256::from(0x60), U256::from(20), U256::from(0x1)];
    let gas = 100000000;
    let value = U256::zero();
    let calldata = vec![0];

    let mut state = EvmState::new(sender, program, gas, value, calldata);
    state.run();


  }
}