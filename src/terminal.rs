use std::io;
use std::process;

pub fn run(command: &str) -> Result<String, io::Error> {
    let output: process::Output = process::Command::new("bash")
        .arg("-c")
        .arg(command)
        .output()
        .unwrap();
    if output.status.success() {
        let stdout: String = String::from_utf8(output.stdout).unwrap();
        Ok(stdout)
    } else {
        let stderr: String = String::from_utf8(output.stderr).unwrap();
        Ok(stderr)
    }
}
