pub mod address;

use std::fmt::{Debug, Display};
use std::str::FromStr;
use crate::error::SyntaxError;
use address::Address;

#[derive(Clone, PartialEq, Eq)]
pub struct CIDR {
  address: Address,
  subnetmask: u8,
}

impl FromStr for CIDR {
  type Err = SyntaxError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let cidr_parts: Vec<&str> = s.split('/').collect();
    let (address, subnetmask) = match cidr_parts.len() {
      0 | 1 => return Err(SyntaxError(format!("cannot find address / subnetmask: {}", s))),
      2 => (cidr_parts[0], cidr_parts[1]),
      _ => return Err(SyntaxError(format!("included unknown attribute: {}", s))),
    };

    let subnetmask = if let Ok(subnetmask) = subnetmask.parse::<u8>() {
      if let 8..=32 = subnetmask {
        subnetmask
      } else {
        return Err(SyntaxError(format!("invalid subnetmask range: {}", subnetmask)));
    }
    } else {
      return Err(SyntaxError(format!("invalid subnetmask range: {}", subnetmask)));
    };
    let address = address.parse::<Address>()?;

    Ok(CIDR{ address, subnetmask })        
  }
}

impl Display for CIDR {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}/{}", self.address, self.subnetmask)
  }
}

impl Debug for CIDR {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self)
  }
}

impl CIDR {
  pub fn address(self) -> Address {
    self.address
  }

  pub fn subnetmask(self) -> u8 {
    self.subnetmask
  }
}
