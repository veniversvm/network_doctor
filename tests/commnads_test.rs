

#[cfg(test)]
mod test {
    use network_doctor::commands;
    
    #[test]
    fn test_ping_command_invalid_os() {
        // Este test verifica la rama "not linux" de tu función ping_command
        let result = commands::ping_command("windows".to_string());
        
        // Como hace un `echo Invalid Command`, el resultado debería contener ese texto (y un salto de línea)
        assert_eq!(result.trim(), "Invalid Command");
    }
}
