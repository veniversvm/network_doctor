use network_doctor::commands::{extrac_ping_statistics, ping_command, ping_extraction};

fn main() {
    println!("Hello, world!");

    let result = ping_command("linux".to_string());
    let result = ping_extraction(result);

    let statictis = extrac_ping_statistics(&result[1]);

    println!("{}", result[0]);
    println!("{}", statictis);
}
