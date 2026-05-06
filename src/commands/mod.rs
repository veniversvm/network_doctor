use std::process::Command;
use std::str;

type IPV4<'a> = &'a str;
type IPV6<'a> = &'a str;
type DNS<'a> = &'a str;
type DomainName<'a> = &'a str;
type IPDir<'a> = (DomainName<'a>, IPV4<'a>, IPV6<'a>, DNS<'a>);

//////
//////
//////

const PING_ROUTES: [IPDir; 4] = [
    ("Google", "8.8.8.8", "2001:4860:4860::8888", "google.com"),
    (
        "Cloudflare",
        "1.1.1.1",
        "2001:4860:4860::8888",
        "cloudflare.com",
    ),
    ("Quad9", "9.9.9.9", "2001:4860:4860::8888", "quad9.com"),
    (
        "Cisco",
        "208.67.222.222",
        "2001:4860:4860::8888",
        "cisco.com",
    ),
];

//////
//////
//////

pub fn ping_command(target_os: String, destination: String) -> String {
    let ping = if target_os == "linux" {
        println!("ping target: {destination}");
        let ping_arg = format!("ping {destination} -c 4");
        Command::new("sh")
            .arg("-c")
            .arg(ping_arg)
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

    //println!("{}", res);

    res.to_string()
}

//////
//////
//////

/// ping_straction returns a Vec[String] of the data
/// obtained from the ping command.
///
/// For the moment only works with linux ping command.
pub fn ping_extraction(statistics: String) -> Vec<String> {
    let mut results: Vec<String> = vec![];
    for line in statistics.lines() {
        if line.contains("packets") || line.contains("rtt") {
            results.push(line.to_string());
        }
    }

    results
}

//////
//////
//////

pub fn extrac_ping_statistics(rtt_line: &str) -> String {
    let keys: [&str; 4] = ["Most Fast", "Average", "Most Slow", "Mean Deaviation"];
    let mut result: String = "Ping results:\n".to_string();

    let line_split: Vec<_> = rtt_line.split("=").collect();

    let line_values: Vec<_> = line_split[1].split("/").collect();

    for i in 0..line_values.len() {
        result += &("\n".to_owned() + keys[i] + " - " + line_values[i].trim());
    }

    //println!("{result}");

    result
}

//////
//////
//////

pub fn dns_resolution() -> String {
    let mut result: String = String::new();

    for domain in PING_ROUTES {
        println!("{}", domain.0);
        for dns_or_ip in [domain.1, domain.2, domain.3] {
            let ping_result = ping_command(std::env::consts::OS.to_string(), dns_or_ip.to_string());
            //println!("ping result: {:?}", ping_result);
            if ping_result.len() == 0 || ping_result.contains("unreachable") {
                result +=
                    &(dns_or_ip.to_string() + " for " + domain.0 + " fail" + "\n --------- \n");
                continue;
            }
            let extraction_result = ping_extraction(ping_result);
            //println!("{:?}", extraction_result);
            let statistic_result: String = extrac_ping_statistics(&extraction_result[1]);
            result += &(statistic_result + "\n --------- \n");
        }
    }

    result
}
