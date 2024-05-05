pub mod cidr;
pub mod error;

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
    println!("{:?}", arg);
    println!("{:?}", "255.128.255.255/24".parse::<cidr::ipv4::CIDR>());
    println!("Hello, world!");
}
