// CORRECT: Each struct/trait has a single responsibility
// Demonstrates proper separation of concerns in Rust

use std::collections::HashMap;
use chrono::{DateTime, Utc};
use regex::Regex;

// User entity - only handles user data
#[derive(Debug, Clone)]
pub struct User {
    username: String,
    email: String,
}

impl User {
    pub fn new(username: String, email: String) -> Self {
        User { username, email }
    }
    
    pub fn username(&self) -> &str {
        &self.username
    }
    
    pub fn email(&self) -> &str {
        &self.email
    }
    
    pub fn set_username(&mut self, username: String) {
        self.username = username;
    }
    
    pub fn set_email(&mut self, email: String) {
        self.email = email;
    }
}

// User repository trait - defines user persistence operations
pub trait UserRepository {
    fn save(&self, user: &User) -> Result<bool, String>;
    fn update(&self, user: &User) -> Result<bool, String>;
    fn find_by_id(&self, user_id: &str) -> Result<Option<User>, String>;
    fn delete(&self, user: &User) -> Result<bool, String>;
}

// Concrete implementation of UserRepository
pub struct DatabaseUserRepository {
    connection: Option<String>,
}

impl DatabaseUserRepository {
    pub fn new() -> Self {
        DatabaseUserRepository {
            connection: None,
        }
    }
    
    fn connect(&mut self) -> Result<(), String> {
        if self.connection.is_none() {
            println!("Connecting to database...");
            self.connection = Some("database_connection".to_string());
        }
        Ok(())
    }
}

impl UserRepository for DatabaseUserRepository {
    fn save(&self, user: &User) -> Result<bool, String> {
        println!("Saving user to database: {}", user.username());
        // Database save logic
        Ok(true)
    }
    
    fn update(&self, user: &User) -> Result<bool, String> {
        println!("Updating user in database: {}", user.username());
        // Database update logic
        Ok(true)
    }
    
    fn find_by_id(&self, user_id: &str) -> Result<Option<User>, String> {
        println!("Finding user with ID: {}", user_id);
        // Database query logic
        Ok(Some(User::new("john_doe".to_string(), "john@example.com".to_string())))
    }
    
    fn delete(&self, user: &User) -> Result<bool, String> {
        println!("Deleting user from database: {}", user.username());
        // Database delete logic
        Ok(true)
    }
}

// Email service trait - defines email operations
pub trait EmailService {
    fn send_welcome_email(&self, email: &str) -> Result<bool, String>;
    fn send_password_reset_email(&self, email: &str) -> Result<bool, String>;
    fn send_notification_email(&self, email: &str, subject: &str, message: &str) -> Result<bool, String>;
}

// Concrete implementation of EmailService
pub struct SMTPEmailService {
    smtp_server: String,
}

impl SMTPEmailService {
    pub fn new(smtp_server: String) -> Self {
        SMTPEmailService { smtp_server }
    }
}

impl EmailService for SMTPEmailService {
    fn send_welcome_email(&self, email: &str) -> Result<bool, String> {
        println!("Sending welcome email via SMTP ({}) to: {}", self.smtp_server, email);
        // SMTP email sending logic
        Ok(true)
    }
    
    fn send_password_reset_email(&self, email: &str) -> Result<bool, String> {
        println!("Sending password reset email via SMTP ({}) to: {}", self.smtp_server, email);
        // SMTP email sending logic
        Ok(true)
    }
    
    fn send_notification_email(&self, email: &str, subject: &str, message: &str) -> Result<bool, String> {
        println!("Sending notification via SMTP ({}) to {}: {}", self.smtp_server, email, subject);
        // SMTP email sending logic
        Ok(true)
    }
}

// Logger trait - defines logging operations
pub trait ActivityLogger {
    fn log_user_activity(&self, username: &str, activity: &str);
    fn log_error(&self, error: &str, context: &str);
}

// Concrete implementation of ActivityLogger
pub struct FileActivityLogger {
    log_file: String,
}

impl FileActivityLogger {
    pub fn new(log_file: String) -> Self {
        FileActivityLogger { log_file }
    }
    
    fn write_to_file(&self, message: &str) -> Result<(), String> {
        // File writing logic would go here
        println!("Writing to {}: {}", self.log_file, message);
        Ok(())
    }
}

impl ActivityLogger for FileActivityLogger {
    fn log_user_activity(&self, username: &str, activity: &str) {
        let timestamp = Utc::now();
        let log_message = format!("[LOG {}] User {} performed: {}", 
                                timestamp.format("%Y-%m-%d %H:%M:%S"), username, activity);
        println!("{}", log_message);
        self.write_to_file(&log_message).unwrap_or_else(|e| {
            eprintln!("Failed to write to log file: {}", e);
        });
    }
    
    fn log_error(&self, error: &str, context: &str) {
        let timestamp = Utc::now();
        let log_message = format!("[ERROR {}] {}: {}", 
                                timestamp.format("%Y-%m-%d %H:%M:%S"), context, error);
        println!("{}", log_message);
        self.write_to_file(&log_message).unwrap_or_else(|e| {
            eprintln!("Failed to write to log file: {}", e);
        });
    }
}

// Validator trait - defines validation operations
pub trait UserValidator {
    fn validate_email(&self, email: &str) -> bool;
    fn validate_username(&self, username: &str) -> bool;
    fn validate_user(&self, user: &User) -> bool;
}

// Concrete implementation of UserValidator
pub struct DefaultUserValidator {
    email_regex: Regex,
}

impl DefaultUserValidator {
    pub fn new() -> Self {
        DefaultUserValidator {
            email_regex: Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap(),
        }
    }
    
    pub fn validate_password(&self, password: &str) -> bool {
        password.len() >= 8 &&
        password.chars().any(|c| c.is_uppercase()) &&
        password.chars().any(|c| c.is_lowercase()) &&
        password.chars().any(|c| c.is_numeric())
    }
}

impl UserValidator for DefaultUserValidator {
    fn validate_email(&self, email: &str) -> bool {
        self.email_regex.is_match(email)
    }
    
    fn validate_username(&self, username: &str) -> bool {
        username.len() >= 3 && username.len() <= 20 && 
        username.chars().all(|c| c.is_alphanumeric() || c == '_')
    }
    
    fn validate_user(&self, user: &User) -> bool {
        self.validate_email(user.email()) && self.validate_username(user.username())
    }
}

// Formatter trait - defines formatting operations
pub trait UserFormatter {
    fn format_user_for_display(&self, user: &User) -> String;
    fn format_user_for_api(&self, user: &User) -> HashMap<String, String>;
}

// Concrete implementation of UserFormatter
pub struct DefaultUserFormatter;

impl DefaultUserFormatter {
    pub fn new() -> Self {
        DefaultUserFormatter
    }
    
    pub fn format_user_for_csv(&self, user: &User) -> String {
        format!("{},{}", user.username(), user.email())
    }
}

impl UserFormatter for DefaultUserFormatter {
    fn format_user_for_display(&self, user: &User) -> String {
        format!("{} ({})", user.username(), user.email())
    }
    
    fn format_user_for_api(&self, user: &User) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("username".to_string(), user.username().to_string());
        map.insert("email".to_string(), user.email().to_string());
        map.insert("display_name".to_string(), self.format_user_for_display(user));
        map
    }
}

// Analytics service trait - defines analytics operations
pub trait AnalyticsService {
    fn track_user_event(&self, username: &str, event_name: &str, metadata: Option<HashMap<String, String>>);
    fn track_user_registration(&self, username: &str);
    fn track_user_update(&self, username: &str);
}

// Concrete implementation of AnalyticsService
pub struct GoogleAnalyticsService {
    tracking_id: String,
}

impl GoogleAnalyticsService {
    pub fn new(tracking_id: String) -> Self {
        GoogleAnalyticsService { tracking_id }
    }
    
    pub fn track_user_login(&self, username: &str, ip_address: &str) {
        let mut metadata = HashMap::new();
        metadata.insert("ip_address".to_string(), ip_address.to_string());
        self.track_user_event(username, "user_login", Some(metadata));
    }
}

impl AnalyticsService for GoogleAnalyticsService {
    fn track_user_event(&self, username: &str, event_name: &str, metadata: Option<HashMap<String, String>>) {
        let timestamp = Utc::now().timestamp();
        println!("[ANALYTICS {}] {}: {} (timestamp: {})", self.tracking_id, username, event_name, timestamp);
        if let Some(meta) = metadata {
            println!("  Metadata: {:?}", meta);
        }
        // Google Analytics tracking logic
    }
    
    fn track_user_registration(&self, username: &str) {
        self.track_user_event(username, "user_registered", None);
    }
    
    fn track_user_update(&self, username: &str) {
        self.track_user_event(username, "user_updated", None);
    }
}

// Notification service trait - defines notification operations
pub trait NotificationService {
    fn send_push_notification(&self, username: &str, message: &str) -> Result<bool, String>;
    fn send_sms_notification(&self, phone: &str, message: &str) -> Result<bool, String>;
}

// Concrete implementation of NotificationService
pub struct FirebaseNotificationService {
    api_key: String,
}

impl FirebaseNotificationService {
    pub fn new(api_key: String) -> Self {
        FirebaseNotificationService { api_key }
    }
}

impl NotificationService for FirebaseNotificationService {
    fn send_push_notification(&self, username: &str, message: &str) -> Result<bool, String> {
        println!("Sending Firebase push notification (API: {}) to {}: {}", self.api_key, username, message);
        // Firebase push notification logic
        Ok(true)
    }
    
    fn send_sms_notification(&self, phone: &str, message: &str) -> Result<bool, String> {
        println!("Sending SMS via Firebase (API: {}) to {}: {}", self.api_key, phone, message);
        // SMS sending logic
        Ok(true)
    }
}

// User service - orchestrates operations using other services
pub struct CorrectUserService<R, E, L, V, F, A, N>
where
    R: UserRepository,
    E: EmailService,
    L: ActivityLogger,
    V: UserValidator,
    F: UserFormatter,
    A: AnalyticsService,
    N: NotificationService,
{
    user_repo: R,
    email_service: E,
    logger: L,
    validator: V,
    formatter: F,
    analytics: A,
    notification_service: Option<N>,
}

impl<R, E, L, V, F, A, N> CorrectUserService<R, E, L, V, F, A, N>
where
    R: UserRepository,
    E: EmailService,
    L: ActivityLogger,
    V: UserValidator,
    F: UserFormatter,
    A: AnalyticsService,
    N: NotificationService,
{
    pub fn new(
        user_repo: R,
        email_service: E,
        logger: L,
        validator: V,
        formatter: F,
        analytics: A,
        notification_service: Option<N>,
    ) -> Self {
        CorrectUserService {
            user_repo,
            email_service,
            logger,
            validator,
            formatter,
            analytics,
            notification_service,
        }
    }
    
    pub fn create_user(&self, username: String, email: String) -> Result<User, String> {
        // Create user object
        let user = User::new(username.clone(), email.clone());
        
        // Validate user data
        if !self.validator.validate_user(&user) {
            let error_msg = format!("Invalid user data: username={}, email={}", username, email);
            self.logger.log_error(&error_msg, "User creation failed");
            return Err(error_msg);
        }
        
        // Save user
        self.user_repo.save(&user).map_err(|e| {
            self.logger.log_error(&e, "Failed to save user");
            e
        })?;
        
        // Send welcome email
        if let Err(e) = self.email_service.send_welcome_email(&email) {
            self.logger.log_error(&e, "Failed to send welcome email");
            // Don't fail user creation if email fails
        }
        
        // Log activity
        self.logger.log_user_activity(&username, "User created");
        
        // Track analytics
        self.analytics.track_user_registration(&username);
        
        // Send push notification if service is available
        if let Some(ref notification_service) = self.notification_service {
            if let Err(e) = notification_service.send_push_notification(&username, "Welcome! Your account has been created.") {
                self.logger.log_error(&e, "Failed to send push notification");
            }
        }
        
        Ok(user)
    }
    
    pub fn update_user(&self, user: &mut User, new_username: String, new_email: String) -> Result<(), String> {
        // Validate new data
        let temp_user = User::new(new_username.clone(), new_email.clone());
        if !self.validator.validate_user(&temp_user) {
            let error_msg = format!("Invalid user data: username={}, email={}", new_username, new_email);
            self.logger.log_error(&error_msg, "User update failed");
            return Err(error_msg);
        }
        
        // Update user
        user.set_username(new_username.clone());
        user.set_email(new_email);
        
        self.user_repo.update(user).map_err(|e| {
            self.logger.log_error(&e, "Failed to update user");
            e
        })?;
        
        // Log activity
        self.logger.log_user_activity(&new_username, "User updated");
        
        // Track analytics
        self.analytics.track_user_update(&new_username);
        
        Ok(())
    }
    
    pub fn format_user(&self, user: &User) -> String {
        self.formatter.format_user_for_display(user)
    }
    
    pub fn get_user_api_data(&self, user: &User) -> HashMap<String, String> {
        self.formatter.format_user_for_api(user)
    }
}

// Factory function to create a fully configured user service
pub fn create_user_service() -> CorrectUserService<
    DatabaseUserRepository,
    SMTPEmailService,
    FileActivityLogger,
    DefaultUserValidator,
    DefaultUserFormatter,
    GoogleAnalyticsService,
    FirebaseNotificationService,
> {
    let user_repo = DatabaseUserRepository::new();
    let email_service = SMTPEmailService::new("smtp.example.com".to_string());
    let logger = FileActivityLogger::new("activity.log".to_string());
    let validator = DefaultUserValidator::new();
    let formatter = DefaultUserFormatter::new();
    let analytics = GoogleAnalyticsService::new("GA-XXXXX-X".to_string());
    let notification_service = FirebaseNotificationService::new("firebase-api-key".to_string());
    
    CorrectUserService::new(
        user_repo,
        email_service,
        logger,
        validator,
        formatter,
        analytics,
        Some(notification_service),
    )
}

// Example usage with dependency injection - uncomment to run
/*
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create user service with all dependencies
    let user_service = create_user_service();
    
    // Create a new user
    let mut user = user_service.create_user("john_doe".to_string(), "john@example.com".to_string())?;
    println!("User created: {}", user_service.format_user(&user));
    
    // Update the user
    user_service.update_user(&mut user, "john_smith".to_string(), "john.smith@example.com".to_string())?;
    println!("User updated: {}", user_service.format_user(&user));
    println!("API data: {:?}", user_service.get_user_api_data(&user));
    
    Ok(())
}
*/