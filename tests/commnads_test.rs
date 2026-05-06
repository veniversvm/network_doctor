#[cfg(test)]
mod test {
    use network_doctor::commands;

    //////
    //////
    //////
    #[test]
    fn test_ping_command_invalid_os() {
        // Este test verifica la rama "not linux" de tu función ping_command
        let result = commands::ping_command("windows".to_string(), "8.8.8.8".to_string());

        // Como hace un `echo Invalid Command`, el resultado debería contener ese texto (y un salto de línea)
        assert_eq!(result.trim(), "Invalid Command");
    }

    //////
    //////
    //////

    #[test]
    fn test_ping_extraction() {
        let expected: Vec<String> = vec![
            "5 packets transmitted, 4 received, 20% packet loss, time 4006ms".to_string(),
            "rtt min/avg/max/mdev = 34.677/35.532/38.528/0.994 ms".to_string(),
        ];
        let test_string: String = "PING 8.8.8.8 (8.8.8.8) 56(84) bytes of data.\n
64 bytes from 8.8.8.8: icmp_seq=1 ttl=115 time=34.7 ms
64 bytes from 8.8.8.8: icmp_seq=2 ttl=115 time=37.5 ms\n
64 bytes from 8.8.8.8: icmp_seq=3 ttl=115 time=36.3 ms\n
64 bytes from 8.8.8.8: icmp_seq=4 ttl=115 time=36.2 ms\n
--- 8.8.8.8 ping statistics ---\n
5 packets transmitted, 4 received, 20% packet loss, time 4006ms\n
rtt min/avg/max/mdev = 34.677/35.532/38.528/0.994 ms\n"
            .to_string();

        let result = commands::ping_extraction(test_string.clone());
        assert_eq!(result, expected);
    }

    //////
    //////
    //////

    #[test]
    fn test_extract_ping_statistics() {
        let expected = "Ping results:

Most Fast - 34.677
Average - 36.169
Most Slow - 37.476
Mean Deaviation - 0.994 ms";
        let test_str: &str = "rtt min/avg/max/mdev = 34.677/36.169/37.476/0.994 ms";
        let result = commands::extrac_ping_statistics(test_str);

        assert_eq!(result, expected);
    }
}
