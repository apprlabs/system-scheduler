use std::process::Command;

pub fn get_command_output(command: &mut Command) -> Result<String, String> {
    match command.output() {
        Ok(output) => {
            if output.stderr.len() > 0 {
                Err(format!("Error occurred in command: {}", output.stderr.get(0).unwrap()))
            } else {
                Ok(String::from_utf8(output.stdout).unwrap())
            }
        }
        Err(err) => Err(err.to_string())
    }
}