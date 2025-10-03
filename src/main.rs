use std::thread::sleep;
use std::time::Duration;

use public_ip_notifier::config::Config;
use public_ip_notifier::json_handler::ToConfig;
use public_ip_notifier::{constants, ip_check, json_handler};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use serde_json::Value;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    constants::setup();

    let cli_args = std::env::args().collect::<Vec<String>>();
    

    if cli_args.len() > 1 {

        match cli_args[1].as_str() {
            "-h" => {
                help();
            }
            "-c" => {
                if cli_args.len() != 4 {
                    eprintln!("Error: -c requires exactly 2 arguments: <property> <value>\nSee -h for more info.");
                    return Ok(());
                }
                let property = &cli_args[2];
                let value = &cli_args[3];

                match property.as_str() {
                    "emailSMTPPort" | "checkIntervalMinutes" => {
                        // Check if value is a valid u64
                        if value.parse::<u64>().is_err() {
                            eprintln!("Error: {} must be a valid u64 integer.", property);
                            return Ok(());
                        }

                        // Port number check
                        if property == "email_smtp_port" && (value.parse::<u64>().unwrap() > 65535) {
                            eprintln!("Error: email_smtp_port must be between 1 and 65535.");
                            return Ok(());
                        }

                        // All checks passed
                        json_handler::write_config(property, Value::Number(value.parse::<u64>().unwrap().into()));
                        return Ok(());
                    }
                    _ => json_handler::write_config(property, Value::String(value.to_string()))
                }
            }
            "-p" => {
                let config = json_handler::read_json_as_value(&constants::get_config_path()).to_config();
                config.print();
            }
            "-t" => {
                let config = json_handler::read_json_as_value(&constants::get_config_path()).to_config();
                let ip = ip_check::get_public_ip().unwrap_or("0.0.0.0".into());
                let _ = send_email(config, ip);
            }
            _ => {}
        }

        return Ok(());
    }

    loop {
        // The config is read in each loop to allow for dynamic changes
        let config = json_handler::read_json_as_value(&constants::get_config_path()).to_config();

        // Debug print the config (to be removed later)
        config.print();

        // Get the current public IP
        let public_ip = match ip_check::get_public_ip() {
            Ok(ip) => ip,
            Err(e) => {
                eprintln!("Error getting public IP: {:?}", e);
                sleep(Duration::from_secs(config.check_interval_minutes * 60));
                continue;
            }
        };

        // Debug print the public IP (to be removed later)
        println!("Public IP: {}", public_ip);

        // If the IP hasn't changed, wait and check again
        if public_ip == config.ip_address {
            println!("IP has not changed.");
        } 
        // If the IP has changed, update the config and send an email
        else {
            println!("IP has changed! Old: {}, New: {}", config.ip_address, public_ip);
            json_handler::write_config("ip_address", Value::String(public_ip.clone()));
            let _ = send_email(config.clone(), public_ip);
        }

        // Wait for the specified interval before checking again
        sleep(Duration::from_secs(&config.check_interval_minutes * 60));
    }
}

fn send_email(config: Config, new_ip_address: String) -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Define the email
    let email = Message::builder()
        .from(
            format!("IP Change Notifier <{}>", config.email_address)
                .parse()
                .unwrap(),
        )
        .to(format!("{}", config.recipient_address)
            .parse()
            .unwrap())
        .subject("Your IP Changed!")
        .body(format!("Hello,\nYour public IP has changed to {}.", new_ip_address)) 
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

fn help() {
    println!("Display this message: -h");
    println!("Set the value of something in the config: -c <property> <value>\nemailAddress, emailPassword, emailSMTPHost, emailSMTPPort, ipAddress, recipientAddress");
    println!("Print config: -p");
    println!("Send test email: -t");
}