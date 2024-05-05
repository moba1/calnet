use std::str::FromStr;
use std::fmt::{Debug, Display};
use std::convert::From;
use regex::Regex;
use once_cell::sync::Lazy;
use crate::error::SyntaxError;

#[derive(Clone, PartialEq, Eq)]
pub struct Address(u32);

macro_rules! matcher {
    ($name:ident, $needle:expr) => {
      fn $name(haystack: &str) -> bool {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new($needle).unwrap());
        RE.is_match(haystack)
      }
    };
}

matcher!(match4, r"\d{1,3}.\d{1,3}.\d{1,3}.\d{1,3}");

impl FromStr for Address {
  type Err = SyntaxError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let address: u32 = if match4(s) {
      parse_address4(s)?
    } else {
      return Err(SyntaxError(format!("unsupported format: {}", s)))
    };

    Ok(Self(address))
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

impl Debug for Address {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let address = if cfg!(target_endian = "big") {
      self.0.to_ne_bytes()
    } else {
      self.0.to_le_bytes()
    };
    let address = address
      .iter()
      .map(|part| part.to_string())
      .collect::<Vec<String>>()
      .join(".");
    write!(f, "{}", address)
  }
}

impl Display for Address {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl From<u32> for Address {
  fn from(value: u32) -> Self {
    Self(value)
  }
}

impl Into<u32> for Address {
  fn into(self) -> u32 {
    self.0
  }
}

impl Address {
  pub fn to_le_bytes(self) -> [u8; 4] {
    self.0.to_le_bytes()
  }

  pub fn to_ne_bytes(self) -> [u8; 4] {
    self.0.to_ne_bytes()
  }

  pub fn to_be_bytes(self) -> [u8; 4] {
    self.0.to_be_bytes()
  }

  pub fn from_ne_bytes(buf: [u8; 4]) -> Address {
    Address(u32::from_ne_bytes(buf))
  }

  pub fn from_be_bytes(buf: [u8; 4]) -> Address {
    Address(u32::from_be_bytes(buf))
  }

  pub fn from_le_bytes(buf: [u8; 4]) -> Address {
    Address(u32::from_le_bytes(buf))
  }
}

impl Address {
  pub fn to_dot_notation(&self, block_number: u8) -> String {
    match block_number {
      1 => format!("{}", self.0.to_be()),
      4 => format!("{}", self),
      _ => panic!("unsupported dot notation"),
    }
  }
}