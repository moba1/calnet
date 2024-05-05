pub mod cidr;
pub mod error;

use std::error::Error;
use std::process::exit;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about = "calculate network addresses from CIDR", long_about = None)]
struct Args {
  #[command(subcommand)]
  command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
  #[command(about = "calculate IPv4 CIDR", name = "ipv4")]
  IPv4 {
    #[arg(help = "IPv4 CIDR (ex: 192.168.1.0/24)")]
    cidr: String
  }
}

fn main() {
  let arg: Args = Args::parse();
  let exit_status = match arg.command {
    Command::IPv4 { cidr } => calculate_ipv4_cidr(cidr),
  };
  if exit_status.is_err() {
    exit(1);
  }
}

fn calculate_ipv4_cidr(cidr: String) -> Result<(), Box<dyn Error>> {
  println!("{:?}", cidr.parse::<cidr::ipv4::CIDR>());
  Ok(())
}
