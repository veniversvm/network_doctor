use std::process::Command;
use std::str;

// The idea os this function is to locate
// the running ip of the router to test the connection
// and dicard router malfunctioning
pub fn ifconfig() {
    let result = {
        Command::new("sh")
            .arg("-c")
            .arg("ifconfig")
            .output()
            .expect("faile 'ifconfig'")
    };

    let result_str = str::from_utf8(&result.stdout).expect("invalid utf-8");
    let result_to_string = result_str.to_string();

    let split_result: Vec<_> = result_to_string.split("\n\n").collect();

    let mut ip_results: Vec<String> = Vec::new();
    for element in split_result.iter() {
        if element.contains("RUNNING") && !element.contains("127.0.0.1") {
            let ip: String = return_ip(element);
            if ip.chars().count() > 0 {
                ip_results.push(ip);
            }
        }
    }

    println!("{:?}", ip_results);
}

/// return_ip return the inet value
pub fn return_ip(data: &str) -> String {
    let split_data: Vec<_> = data.split("\n").collect();

    for element in split_data.iter() {
        if element.contains("inet") {
            let line: Vec<_> = element.trim().split(" ").collect();
            return line[1].to_string();
        }
    }
    "no inet value founded :c".to_string()
}
