// use std::result;

// use network_doctor::commands::dns_resolution;
use network_doctor::ifconfig::ifconfig;

fn main() {
    println!("Hello, world!");

    // let result = dns_resolution();
    ifconfig();

    //println!("{:?}", result);
}
