/// Configuration structure for the application
/// Holds email settings, recipient info, check interval, and IP address.
/// 
/// Fields
/// * `email_address`: The email address used to send notifications.
/// * `email_password`: The password or app-specific password for the email account.
/// * `email_smtp_host`: The SMTP host for the email service.
/// * `email_smtp_port`: The SMTP port for the email service.
/// * `recipient_address`: The email address of the recipient who will receive notifications.
/// * `check_interval_minutes`: The interval in minutes to check for IP changes.
/// * `ip_address`: The last known IP address.
#[derive(Debug, Clone)]
pub struct Config {
    /// The email address used to send notifications.
    pub email_address: String,
    /// The username to authenticate with (often the same as the email_address)
    pub username: String,
    /// The password or app-specific password for the email account.
    pub email_password: String,
    /// The SMTP host for the email service.
    pub email_smtp_host: String,
    /// The SMTP port for the email service.
    pub email_smtp_port: u16,
    /// The email address of the recipient who will receive notifications.
    pub recipient_address: String,
    /// The interval in minutes to check for IP changes.
    pub check_interval_minutes: u64,
    /// The last known IP address.
    pub ip_address: String,
    /// The number of sequential failures in checking the IP address.
    pub sequential_failures: u32,
    /// The threshold of sequential failures before sending an alert email.
    pub failure_threshold: u32,
}

impl Config {
    /// Creates a new `Config` instance with the provided parameters.
    /// The `ip_address` field is initialized to an empty string.
    /// # Parameters
    /// * `email_address`: The email address used to send notifications.
    /// * `email_password`: The password or app-specific password for the email account.
    /// * `email_smtp_host`: The SMTP host for the email service.
    /// * `email_smtp_port`: The SMTP port for the email service.
    /// * `recipient_address`: The email address of the recipient who will receive notifications.
    /// * `check_interval_minutes`: The interval in minutes to check for IP changes.
    /// * `ip_address`: The last known IP address.
    /// # Returns
    /// * `Config` - A new instance of the `Config` struct.
    pub fn new(
        email_address: String,
        username: String,
        email_password: String,
        email_smtp_host: String,
        email_smtp_port: u16,
        recipient_address: String,
        check_interval_minutes: u64,
        ip_address: String,
        sequential_failures: u32,
        failure_threshold: u32,
    ) -> Self {
        Config {
            email_address,
            username,
            email_password,
            email_smtp_host,
            email_smtp_port,
            recipient_address,
            check_interval_minutes,
            ip_address,
            sequential_failures,
            failure_threshold,
        }
    }

    /// Prints the configuration details to the console for debugging purposes.
    pub fn print(&self) {
        println!("Email Address: {}", self.email_address);
        println!("Username: {}", self.username);
        println!("Email Password: {}", self.email_password);
        println!("SMTP Host: {}", self.email_smtp_host);
        println!("SMTP Port: {}", self.email_smtp_port);
        println!("Recipient Address: {}", self.recipient_address);
        println!("Check Interval (minutes): {}", self.check_interval_minutes);
        println!("Last Known IP Address: {}", self.ip_address);
    }

    /// Converts the `Config` instance to a JSON value.
    /// # Returns
    /// * `serde_json::Value` - A JSON representation of the `Config` instance
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "emailAddress": self.email_address,
            "username": self.username,
            "emailPassword": self.email_password,
            "emailSMTPHost": self.email_smtp_host,
            "emailSMTPPort": self.email_smtp_port,
            "recipientAddress": self.recipient_address,
            "checkIntervalMinutes": self.check_interval_minutes,
            "ipAddress": self.ip_address,
        })
    }
}