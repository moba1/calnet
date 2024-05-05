use regex::Regex;
use once_cell::sync::Lazy;
use std::error::Error;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct SyntaxError(String);

impl Display for SyntaxError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "invalid address notation: {}", self.0)
  }
}

impl Error for SyntaxError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    None
  }
}

#[derive(Debug, Clone)]
pub struct CIDR {
  address: u32,
  subnetmask: u8,
}

macro_rules! matcher {
    ($name:ident, $needle:expr) => {
      fn $name(haystack: &str) -> bool {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new($needle).unwrap());
        RE.is_match(haystack)
      }
    };
}

matcher!(match4, r"\d{1,3}.\d{1,3}.\d{1,3}.\d{1,3}");

impl CIDR {
  pub fn parse(cidr: &str) -> Result<Self, SyntaxError> {
    let cidr_parts: Vec<&str> = cidr.split('/').collect();
    let (address, subnetmask) = match cidr_parts.len() {
      0 | 1 => return Err(SyntaxError(format!("cannot find address / subnetmask: {}", cidr))),
      2 => (cidr_parts[0], cidr_parts[1]),
      _ => return Err(SyntaxError(format!("included unknown attribute: {}", cidr))),
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
    let address: u32 = if match4(address) {
      parse_address4(address)?
    } else {
      return Err(SyntaxError(format!("unsupported format: {}", cidr)))
    };

    Ok(CIDR{ address, subnetmask })
  }
}

fn parse_address4(address: &str) -> Result<u32, SyntaxError> {
    let address_parts: Vec<&str> = address.split('.').collect();
    let address_parts: Vec<u8> = address_parts
      .iter()
      .map(|part| part.parse::<u8>().unwrap())
      .collect();
    assert_eq!(address_parts.len(), 4);
    let address_parts: [u8; 4] = [address_parts[0], address_parts[1], address_parts[2], address_parts[3]];
    let address = if cfg!(target_endian = "big") {
      u32::from_ne_bytes(address_parts)
    } else {
      u32::from_le_bytes(address_parts)
    };
    Ok(address)
}

impl Display for CIDR {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}/{}", self.address, self.subnetmask)
  }
}
