pub mod cidr;

fn main() {
    println!("{:?}", cidr::ipv4::CIDR::parse("255.255.255.255/24"));
    println!("Hello, world!");
}
