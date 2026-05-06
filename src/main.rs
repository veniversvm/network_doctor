use network_doctor::commands::dns_resolution;

fn main() {
    println!("Hello, world!");

    let result = dns_resolution();

    println!("{}", result);
}
