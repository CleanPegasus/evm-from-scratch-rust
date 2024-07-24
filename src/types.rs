use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EthereumAddress([u8; 20]);

impl EthereumAddress {
    pub fn new(address: [u8; 20]) -> Self {
        EthereumAddress(address)
    }

    pub fn as_bytes(&self) -> &[u8; 20] {
        &self.0
    }
}

impl fmt::Display for EthereumAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x")?;
        for byte in &self.0 {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

impl From<[u8; 20]> for EthereumAddress {
    fn from(bytes: [u8; 20]) -> Self {
        EthereumAddress(bytes)
    }
}

