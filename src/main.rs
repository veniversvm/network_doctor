mod commands;
use commands::ping_command;

fn main() {
    println!("Hello, world!");

    ping_command("linux".to_string());
}
