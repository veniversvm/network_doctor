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

pub fn extrac_ping_statistics(ping_values: Vec<&str>) -> String {
    let keys: [&str; 4] = ["Most Fast", "Average", "Most Slow", "Mean Deaviation"];
    let mut result: String = "Ping results:\n".to_string();

    for i in 0..ping_values.len() {
        result += &("\n".to_owned() + keys[i] + " - " + ping_values[i].trim()).replace("ms", "");
    }

    //println!("{result}");

    result
}

//////
//////
//////

pub fn extraction_ping_values(rtt_line: &str) -> Vec<&str> {
    let line_split: Vec<_> = rtt_line.split("=").collect();

    line_split[1].split("/").collect()
}

//////
//////
//////

pub fn clean_value(value: &str) -> f32 {
    let v = value.replace("ms", "").replace("pipe 2", "");

    //println!("converting {value} *{v}*");
    v.trim()
        .parse::<f32>()
        .expect("conversion to f32 for {value} failed")
}

//////
//////
//////
pub fn dns_resolution() -> String {
    let mut result: String = String::new();

    for domain in PING_ROUTES {
        println!("{}", domain.0);

        result += &(domain.0.to_uppercase() + "\n"); // Append DNS name
        // acc of ping values
        let mut fast: f32 = 0.0;
        let mut average: f32 = 0.0;
        let mut slow: f32 = 0.0;
        let mut deviation: f32 = 0.0;
        let mut i = 0;

        // looping trough dns, ipv4 and ipv6
        // TODO: make async
        for dns_or_ip in [domain.1, domain.2, domain.3] {
            let ping_result = ping_command(std::env::consts::OS.to_string(), dns_or_ip.to_string());

            // check if ping result
            if ping_result.is_empty() || ping_result.contains("unreachable") {
                result +=
                    &(dns_or_ip.to_string() + " for " + domain.0 + " fail" + "\n --------- \n");
                continue;
            }

            let extraction_result = ping_extraction(ping_result);
            let ping_values = extraction_ping_values(&extraction_result[1]);
            // println!("Pig values: {:?}", ping_values);
            fast += clean_value(ping_values[0]);
            average += clean_value(ping_values[1]);
            slow += clean_value(ping_values[2]);
            deviation += clean_value(ping_values[3]);
            let statistic_result: String = extrac_ping_statistics(ping_values);
            result += &(statistic_result + "\n --------- \n");
            i += 1;
        }
        let string = format!(
            "Avrg Most Fast = {}\nAvrg Average = {}\nAvrg Slow = {}\nAvrg Mean Deviation = {}",
            (fast / i as f32),
            (average / i as f32),
            (slow / i as f32),
            (deviation / i as f32)
        );
        result.push_str(&string);
        result += "\n --------- \n\n";
    }

    result
}
