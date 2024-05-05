use std::fmt::Display;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct SyntaxError(pub String);

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