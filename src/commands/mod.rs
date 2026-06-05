/// commands module
/// This module purporse is to execute the ping command to test the internet
/// connection of the system.
/// For this purpose a ping is done to IPV4, DNS and domain, IPV6 can't be
/// done at the momment because of lack of IPV6 conection of my current ISP.
/// The domains teste are: Google, Cloudflare, Quad9 and Cisco.
/// Finally the process must return a detailed statistic of the pings realized.
/// PENDING: multithreding execution
use std::process::Command;
use std::str;

/// The fowllowing types are use to mantain some order in the ping targets.
type IPV4<'a> = &'a str;
type IPV6<'a> = &'a str;
type DNS<'a> = &'a str;
// DomainName is the simple popular/comercial name of the ping target, ex: Google.
type DomainName<'a> = &'a str;
// IPDir stands for IP Direction, is a tuple of ping targets
type IPDir<'a> = (DomainName<'a>, IPV4<'a>, IPV6<'a>, DNS<'a>);

//////
//////
//////

/// PING_ROUTES is a static array of the Domains to be tested and their IP targets.
/// Can be expanded but I belive that 4 domains are more that enough.
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

/// ping_command executes the 'ping' and collects the result to be returned.
///
/// # paramters
/// target_os: linux, windows or mac. This will be elminated.
/// destination: target of the ping command.
///
/// The function will try to do the ping and recieve four (4) responses to be collected.
/// In the future instead of the target os, the proper macro will be used to compiled the project
/// in the specific target os.
pub fn ping_command(target_os: String, destination: String) -> String {
    let ping = if target_os == "linux" {
        println!("\rping target: {destination}");
        // \x1B[2J: Limpia la pantalla
        // \x1B[1;1H: Mueve el cursor a la línea 1, columna 1
        print!("\x1B[2J\x1B[1;1H");
        let ping_arg = format!("ping {destination} -c 4");
        Command::new("sh")
            .arg("-c")
            .arg(ping_arg)
            .output()
            .expect("failed process")
    } else {
        // To be eliminated
        println!("not linux");
        Command::new("sh")
            .arg("-c")
            .arg("echo Invalid Command")
            .output()
            .expect("Failed echo command")
    };

    // Command collect the result in Output struct define in std::process::Command.
    // Need to be tranformed in string slice to the transform in String.
    let res = str::from_utf8(&ping.stdout).expect("invalid utf-8");

    // return String
    res.to_string()
}

//////
//////
//////

/// ping_straction returns a Vec[String] of the data
/// obtained from the ping command.
/// Loops trough the result and return a line like this:
/// 'rtt min/avg/max/mdev = 45.139/47.724/50.190/2.076 ms'
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

/// extract_ping_statistics thake a Vector of floats inf string
/// like the one retuned by extraction_ping_values
/// ["34.677", "36.169", "37.476", "0.994 ms"]
/// and returns a single string in a easy to read format onlythe results
/// obtained by the ping.
/// Example:
/// " Most Fast - 34.677
///   Average - 36.169
///   Most Slow - 37.476
///   Mean Deaviation - 0.994 ms"
pub fn extract_ping_statistics(ping_values: Vec<&str>) -> String {
    // We take advantage of the positional distribution of the values.
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

/// extraction_ping_values take a string slice like this
/// 'rtt min/avg/max/mdev = 34.677/36.169/37.476/0.994 ms'
/// and return a Vector of String like this:
/// ["34.677", "36.169", "37.476", "0.994 ms"]
pub fn extraction_ping_values(rtt_line: &str) -> Vec<&str> {
    let line_split: Vec<_> = rtt_line.split("=").collect();

    line_split[1].split("/").collect()
}

//////
//////
//////

/// clean_value recieves a float numeric string slice and
/// returns a f32 value.
///
/// Is used to process the obtained values from the ping command.
pub fn clean_value(value: &str) -> f32 {
    // clean the string from description
    let v = value.replace("ms", "").replace("pipe 2", "");

    //println!("converting {value} *{v}*");
    v.trim()
        .parse::<f32>()
        .expect("conversion to f32 for {value} failed")
}
//////
//////
//////

/// packets_transmitted recieves the fowllowing line:
/// 5 packets transmitted, 5 received, 0% packet loss, time 4004ms
/// returns (u32, u32)
///
/// In this case we want the first and second numeric value and transform
/// into integer values.
/// The first value in the tuple retuner are the packet transmitted or send
/// and the second are the recieved as response.
///
pub fn packet_analizer(packet_line: &str) -> (u32, u32) {
    let value: Vec<_> = packet_line.split(",").collect();
    //println!("{:?}", value);

    let patckets_transmitted: Vec<_> = value[0].split(" ").collect();
    let patckets_recieve: Vec<_> = value[1].split(" ").collect();

    //println!("{:?}", patckets_recieve);
    //println!("{:?}", patckets_transmitted);

    let pt = patckets_transmitted[0]
        .parse::<u32>()
        .expect("Patcket recieve conversion failed");
    let pr = patckets_recieve[1]
        .parse::<u32>()
        .expect("Packet loss conversion failed");

    //println!("{} {}", pr, pr);

    (pt, pr)
}

///////
//////
//////

/// calculate_packets_loss is used to accurate calculate the
/// loss of packets in different levels of acuumulation, to be
/// precise, the indivual loss of every taget, the accumulate loss
/// of every domain and the total accumalte loss of all packages.
///
/// Recieves two u32 values a return a float32
fn calculate_packets_loss(packets_transmitted: u32, packets_recieved: u32) -> f32 {
    if packets_transmitted == 0 {
        return 0.0;
    }

    100.0 - (packets_recieved as f32 / packets_transmitted as f32) * 100.0
}

//////
//////
//////
pub fn dns_resolution() -> String {
    let mut result: String = String::new();

    let mut avrg_fast: f32 = 0.0;
    let mut avrg_average: f32 = 0.0;
    let mut avrg_slow: f32 = 0.0;
    let mut avrg_deviation: f32 = 0.0;
    let mut total_packets_transmitted: u32 = 0;
    let mut total_packet_recieved: u32 = 0;
    let mut j = 0;

    // Loop in every domain tuplet (Google, Cisco, etc...).
    for domain in PING_ROUTES {
        println!("{}", domain.0);

        result += &(domain.0.to_uppercase() + "\n"); // Append DNS name
        // acc of ping values
        let mut fast: f32 = 0.0;
        let mut average: f32 = 0.0;
        let mut slow: f32 = 0.0;
        let mut deviation: f32 = 0.0;
        let mut packets_transmitted: u32 = 0;
        let mut packets_recieve: u32 = 0;
        let mut i = 0;

        // looping trough dns, ipv4 and ipv6
        // TODO: make async
        for dns_or_ip in [domain.1, domain.2, domain.3] {
            let ping_result = ping_command(std::env::consts::OS.to_string(), dns_or_ip.to_string());

            // check if ping result was succefull, in case of failed ping the
            // proper message is append to the result a jump to the next
            // iteration.
            if ping_result.is_empty() || ping_result.contains("unreachable") {
                result +=
                    &(dns_or_ip.to_string() + " for " + domain.0 + " fail" + "\n --------- \n");
                continue;
            }

            let extraction_result = ping_extraction(ping_result);
            let packets = packet_analizer(&extraction_result[0]);
            let (pt, pr) = packets;
            packets_transmitted += pt;
            packets_recieve += pr;
            let pl = calculate_packets_loss(pt, pr);
            // Info of time per ping
            let ping_values = extraction_ping_values(&extraction_result[1]);
            // println!("Pig values: {:?}", ping_values);
            // TODO: Move to a new func
            fast += clean_value(ping_values[0]);
            avrg_fast += fast;
            average += clean_value(ping_values[1]);
            avrg_average += average;
            slow += clean_value(ping_values[2]);
            avrg_slow += slow;
            deviation += clean_value(ping_values[3]);
            avrg_deviation += deviation;
            let statistic_result: String = extract_ping_statistics(ping_values);
            let packets_str = format!(
                "Transmitted {} - Packets Recieve {} - Packets Loss {}%\n",
                pt, pr, pl
            );
            result += &packets_str;
            result += &(statistic_result + "\n --------- \n");
            i += 1;
            j += 1;
        }
        let packets_lossed = calculate_packets_loss(packets_transmitted, packets_recieve);
        let string = format!(
            "Averages for {}\nAvrg Most Fast = {}\nAvrg Average = {}\nAvrg Slow = {}\nAvrg Mean Deviation = {}\nAvg Packets Transmitted {} - Avg Packets Recieve {} - Avg Packets Loss {}%",
            domain.0,
            (fast / i as f32),
            (average / i as f32),
            (slow / i as f32),
            (deviation / i as f32),
            packets_transmitted,
            packets_recieve,
            packets_lossed
        );
        result.push_str(&string);
        result += "\n --------- \n --------- \n\n";
        total_packets_transmitted += packets_transmitted;
        total_packet_recieved += packets_recieve;
    }

    let total_pl = calculate_packets_loss(total_packets_transmitted, total_packet_recieved);

    let string = format!(
        "Total averages\nAvrg Most Fast = {}\nAvrg Average = {}\nAvrg Slow = {}\nAvrg Mean Deviation = {}\nTotal Packets Transmitted {} - Total Packets Recieved {} - Total Packets Loss {}%",
        (avrg_fast / j as f32),
        (avrg_average / j as f32),
        (avrg_slow / j as f32),
        (avrg_deviation / j as f32),
        total_packets_transmitted,
        total_packet_recieved,
        total_pl
    );
    result.push_str(&string);
    result += "\n --------- \n\n";

    result
}
