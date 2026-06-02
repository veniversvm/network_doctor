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

    for element in split_result.iter() {
        if element.contains("RUNNING") && !element.contains("127.0.0.1") {
            println!("{}", element);
        }
    }

    //println!("{:?}", split_result);
}
