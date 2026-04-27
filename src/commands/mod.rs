
use std::process::Command;
use std::str;

pub fn ping_command(target_os: String) {
    let mut ping = if target_os == "linux" {
        println!("linux");
        Command::new("sh")
            .arg("-c")
            .arg("ping 8.8.8.8 -c 4")
            .output()
            .expect("failed process")
    } else {
        println!("not linux");
        Command::new("sh")
            .arg("-c")
            .arg("echo Invalid Command")
            .output()
            .expect("Failed echo command")
    };

    let res = str::from_utf8(&ping.stdout).expect("invalid utf-8");

    println!("{}", res);
}
