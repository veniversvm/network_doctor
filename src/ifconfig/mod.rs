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

    let mut ip_results: Vec<(String, String)> = Vec::new();
    for element in split_result.iter() {
        if element.contains("RUNNING") && !element.contains("127.0.0.1") {
            let ip: (String, String) = return_ip_and_net_mask(element);
            if ip.1.chars().count() > 0 {
                ip_results.push(ip);
            }
        }
    }

    println!("{:?}", ip_results);
    println!("{:?}", address_to_numbers("192.168.80.52".to_string()))
}

/// return_ip return the inet value and his subnet mask
pub fn return_ip_and_net_mask(data: &str) -> (String, String) {
    let split_data: Vec<_> = data.split("\n").collect();

    for element in split_data.iter() {
        if element.contains("inet") {
            let line: Vec<_> = element.trim().split(" ").collect();
            dbg!(&line);
            return (line[1].to_string(), line[4].to_string());
        }
    }
    ("".to_string(), "no inet founded :c".to_string())
}

/// calculate_gateway_ip to println
/// must use the subnetmsk to determine to proper ping address the
/// router
fn calculate_gateway_ip() {
    // must take the values and tansform into 8bits value.

    // I'm sure that bit operation can give us the proper value.

    // return the gateway
}

fn address_to_numbers(address: String) -> (u8, u8, u8, u8) {
    let address_split: Vec<_> = address.split(".").collect();

    // TODO: create a generic function that parse values and manages limits
    (
        address_split[0]
            .parse::<u8>()
            .expect("error parsing address"),
        address_split[1]
            .parse::<u8>()
            .expect("error parsing address"),
        address_split[2]
            .parse::<u8>()
            .expect("error parsing address"),
        address_split[3]
            .parse::<u8>()
            .expect("error parsing address"),
    )
}
