use std::thread::sleep;
use std::time::Duration;

use ip_update_checker::config::Config;
use ip_update_checker::json_handler::ToConfig;
use ip_update_checker::{constants, ip_check, json_handler};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use serde_json::Value;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    constants::setup();

    loop {
        // The config is read in each loop to allow for dynamic changes
        let config = json_handler::read_json_as_value(&constants::get_config_path()).to_config();

        // Debug print the config (to be removed later)
        config.print();

        // Get the current public IP
        let public_ip = match ip_check::get_public_ip() {
            Ok(ip) => ip,
            Err(e) => {
                eprintln!("Error getting public IP: {}", e);
                sleep(Duration::from_secs(config.check_interval_minutes * 60));
                continue;
            }
        };

        // Debug print the public IP (to be removed later)
        println!("Public IP: {}", public_ip);

        // If the IP hasn't changed, wait and check again
        if public_ip == config.ip_address {
            println!("IP has not changed.");

            sleep(Duration::from_secs(config.check_interval_minutes * 60));
            continue;
        } 
        // If the IP has changed, update the config and send an email
        else {
            println!("IP has changed! Old: {}, New: {}", config.ip_address, public_ip);
            json_handler::write_config("ip_address", Value::String(public_ip));
            let _ = send_email(config);
        }
    }
}

fn send_email(config: Config) -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Define the email
    let email = Message::builder()
        .from(
            format!("IP Change Notifier <{}>", config.email_address)
                .parse()
                .unwrap(),
        )
        .to(format!("Recipient Name <{}>", config.recipient_address)
            .parse()
            .unwrap())
        .subject("Your IP Changed!")
        .body(String::from("Hello, this is a test email from Rust!"))
        .unwrap();

    // Set up the SMTP client
    let creds = Credentials::new(config.email_address, config.email_password);

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay(&config.email_smtp_host)?
        .port(config.email_smtp_port)
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => eprintln!("Could not send email: {:?}", e),
    };
    Ok(())
}
