// VIOLATION: This struct has multiple responsibilities
// It handles user data, email sending, logging, validation, analytics, and database operations

use std::collections::HashMap;
use chrono::{DateTime, Utc};
use regex::Regex;

#[derive(Debug, Clone)]
pub struct ViolationUserService {
    pub username: String,
    pub email: String,
    connection: Option<String>,
}

impl ViolationUserService {
    pub fn new(username: String, email: String) -> Self {
        ViolationUserService {
            username,
            email,
            connection: None,
        }
    }
    
    // User management responsibility
    pub fn save_user(&mut self) -> Result<bool, String> {
        println!("Saving user: {}", self.username);
        // Database save logic
        Ok(true)
    }
    
    pub fn update_user(&mut self, new_username: String, new_email: String) -> Result<bool, String> {
        self.username = new_username;
        self.email = new_email;
        println!("User updated");
        Ok(true)
    }
    
    pub fn delete_user(&self) -> Result<bool, String> {
        println!("Deleting user: {}", self.username);
        // Database delete logic
        Ok(true)
    }
    
    // Email responsibility (should be separate)
    pub fn send_welcome_email(&self) -> Result<bool, String> {
        println!("Sending welcome email to: {}", self.email);
        // Email sending logic
        Ok(true)
    }
    
    pub fn send_password_reset_email(&self) -> Result<bool, String> {
        println!("Sending password reset email to: {}", self.email);
        // Email sending logic
        Ok(true)
    }
    
    pub fn send_notification_email(&self, subject: &str, message: &str) -> Result<bool, String> {
        println!("Sending notification to {}: {}", self.email, subject);
        // Email sending logic
        Ok(true)
    }
    
    // Logging responsibility (should be separate)
    pub fn log_user_activity(&self, activity: &str) {
        let timestamp = Utc::now();
        println!("[LOG {}] User {} performed: {}", timestamp.format("%Y-%m-%d %H:%M:%S"), self.username, activity);
        // Logging logic
    }
    
    pub fn log_error(&self, error: &str, context: &str) {
        let timestamp = Utc::now();
        println!("[ERROR {}] {}: {}", timestamp.format("%Y-%m-%d %H:%M:%S"), context, error);
        // Error logging logic
    }
    
    // Validation responsibility (should be separate)
    pub fn validate_email(&self) -> bool {
        let email_regex = Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();
        email_regex.is_match(&self.email)
    }
    
    pub fn validate_username(&self) -> bool {
        self.username.len() >= 3 && self.username.len() <= 20 && 
        self.username.chars().all(|c| c.is_alphanumeric() || c == '_')
    }
    
    pub fn validate_password(&self, password: &str) -> bool {
        password.len() >= 8 &&
        password.chars().any(|c| c.is_uppercase()) &&
        password.chars().any(|c| c.is_lowercase()) &&
        password.chars().any(|c| c.is_numeric())
    }
    
    // Formatting responsibility (should be separate)
    pub fn format_user_for_display(&self) -> String {
        format!("{} ({})", self.username, self.email)
    }
    
    pub fn format_user_for_api(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("username".to_string(), self.username.clone());
        map.insert("email".to_string(), self.email.clone());
        map.insert("display_name".to_string(), self.format_user_for_display());
        map
    }
    
    // Analytics responsibility (should be separate)
    pub fn track_user_event(&self, event_name: &str, metadata: Option<HashMap<String, String>>) {
        let timestamp = Utc::now().timestamp();
        println!("[ANALYTICS] {}: {} (timestamp: {})", self.username, event_name, timestamp);
        if let Some(meta) = metadata {
            println!("  Metadata: {:?}", meta);
        }
        // Analytics tracking logic
    }
    
    pub fn track_user_registration(&self) {
        self.track_user_event("user_registered", None);
    }
    
    pub fn track_user_login(&self, ip_address: &str) {
        let mut metadata = HashMap::new();
        metadata.insert("ip_address".to_string(), ip_address.to_string());
        self.track_user_event("user_login", Some(metadata));
    }
    
    // Database responsibility (should be separate)
    pub fn connect_to_database(&mut self) -> Result<bool, String> {
        println!("Connecting to database...");
        self.connection = Some("database_connection".to_string());
        Ok(true)
    }
    
    pub fn execute_query(&self, query: &str) -> Result<Vec<String>, String> {
        println!("Executing query: {}", query);
        // Query execution logic
        Ok(vec![])
    }
    
    pub fn close_database_connection(&mut self) {
        println!("Closing database connection");
        self.connection = None;
    }
    
    // Notification responsibility (should be separate)
    pub fn send_push_notification(&self, message: &str) -> Result<bool, String> {
        println!("Sending push notification to {}: {}", self.username, message);
        // Push notification logic
        Ok(true)
    }
    
    pub fn send_sms_notification(&self, phone: &str, message: &str) -> Result<bool, String> {
        println!("Sending SMS to {}: {}", phone, message);
        // SMS sending logic
        Ok(true)
    }
    
    // File handling responsibility (should be separate)
    pub fn export_user_to_csv(&self) -> Result<String, String> {
        let csv_data = format!("{},{}", self.username, self.email);
        println!("Exporting user to CSV: {}", csv_data);
        // CSV export logic
        Ok(csv_data)
    }
    
    pub fn backup_user_data(&self) -> Result<bool, String> {
        println!("Backing up user data for: {}", self.username);
        // Backup logic
        Ok(true)
    }
}

// Example usage - uncomment the main function to run
/*
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut user_service = ViolationUserService::new(
        "john_doe".to_string(),
        "john@example.com".to_string(),
    );
    
    // Validate user data
    if !user_service.validate_email() || !user_service.validate_username() {
        println!("Invalid user data");
        return Ok(());
    }
    
    // Connect to database and save user
    user_service.connect_to_database()?;
    user_service.save_user()?;
    
    // Send welcome email
    user_service.send_welcome_email()?;
    
    // Log activity
    user_service.log_user_activity("User registered");
    
    // Track analytics
    user_service.track_user_registration();
    
    // Display user
    println!("User display: {}", user_service.format_user_for_display());
    
    // Export to CSV
    let csv_data = user_service.export_user_to_csv()?;
    
    // Close database connection
    user_service.close_database_connection();
    
    Ok(())
}
*/