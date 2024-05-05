pub mod cidr;

fn main() {
    println!("{:?}", "255.255.255.255/24".parse::<cidr::ipv4::CIDR>());
    println!("Hello, world!");
}
