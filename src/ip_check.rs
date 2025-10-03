pub fn get_public_ip() -> Result<String, Box<dyn std::error::Error>> {
    let command = "curl ifconfig.me";
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()?;

    let ip = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(ip)
}