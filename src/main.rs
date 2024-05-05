pub mod cidr;
pub mod error;

fn main() {
    println!("{:?}", "255.128.255.255/24".parse::<cidr::ipv4::CIDR>());
    println!("Hello, world!");
}
