"""
VIOLATION: This class has multiple responsibilities
It handles user data, email sending, logging, validation, analytics, and database operations
"""

import re
import datetime
from typing import Optional


class ViolationUserService:
    def __init__(self, username: str, email: str):
        self.username = username
        self.email = email
        self.connection = None
    
    # User management responsibility
    def save_user(self) -> bool:
        """Save user to database"""
        print(f"Saving user: {self.username}")
        # Database save logic
        return True
    
    def update_user(self, new_username: str, new_email: str) -> bool:
        """Update user information"""
        self.username = new_username
        self.email = new_email
        print("User updated")
        return True
    
    def delete_user(self) -> bool:
        """Delete user from database"""
        print(f"Deleting user: {self.username}")
        # Database delete logic
        return True
    
    # Email responsibility (should be separate)
    def send_welcome_email(self) -> bool:
        """Send welcome email to user"""
        print(f"Sending welcome email to: {self.email}")
        # Email sending logic
        return True
    
    def send_password_reset_email(self) -> bool:
        """Send password reset email"""
        print(f"Sending password reset email to: {self.email}")
        # Email sending logic
        return True
    
    def send_notification_email(self, subject: str, message: str) -> bool:
        """Send notification email"""
        print(f"Sending notification to {self.email}: {subject}")
        # Email sending logic
        return True
    
    # Logging responsibility (should be separate)
    def log_user_activity(self, activity: str) -> None:
        """Log user activity"""
        timestamp = datetime.datetime.now().isoformat()
        print(f"[LOG {timestamp}] User {self.username} performed: {activity}")
        # Logging logic
    
    def log_error(self, error: str, context: str) -> None:
        """Log error"""
        timestamp = datetime.datetime.now().isoformat()
        print(f"[ERROR {timestamp}] {context}: {error}")
        # Error logging logic
    
    # Validation responsibility (should be separate)
    def validate_email(self) -> bool:
        """Validate email format"""
        email_pattern = r'^[^\s@]+@[^\s@]+\.[^\s@]+$'
        return bool(re.match(email_pattern, self.email))
    
    def validate_username(self) -> bool:
        """Validate username format"""
        return 3 <= len(self.username) <= 20 and self.username.isalnum()
    
    def validate_password(self, password: str) -> bool:
        """Validate password strength"""
        return (len(password) >= 8 and 
                any(c.isupper() for c in password) and
                any(c.islower() for c in password) and
                any(c.isdigit() for c in password))
    
    # Formatting responsibility (should be separate)
    def format_user_for_display(self) -> str:
        """Format user for display"""
        return f"{self.username} ({self.email})"
    
    def format_user_for_api(self) -> dict:
        """Format user for API response"""
        return {
            "username": self.username,
            "email": self.email,
            "display_name": self.format_user_for_display()
        }
    
    # Analytics responsibility (should be separate)
    def track_user_event(self, event_name: str, metadata: Optional[dict] = None) -> None:
        """Track user event for analytics"""
        timestamp = datetime.datetime.now().timestamp()
        print(f"[ANALYTICS] {self.username}: {event_name} (timestamp: {timestamp})")
        if metadata:
            print(f"  Metadata: {metadata}")
        # Analytics tracking logic
    
    def track_user_registration(self) -> None:
        """Track user registration event"""
        self.track_user_event("user_registered")
    
    def track_user_login(self, ip_address: str) -> None:
        """Track user login event"""
        self.track_user_event("user_login", {"ip_address": ip_address})
    
    # Database responsibility (should be separate)
    def connect_to_database(self) -> bool:
        """Connect to database"""
        print("Connecting to database...")
        self.connection = "database_connection"
        return True
    
    def execute_query(self, query: str) -> list:
        """Execute database query"""
        print(f"Executing query: {query}")
        # Query execution logic
        return []
    
    def close_database_connection(self) -> None:
        """Close database connection"""
        print("Closing database connection")
        self.connection = None
    
    # Notification responsibility (should be separate)
    def send_push_notification(self, message: str) -> bool:
        """Send push notification"""
        print(f"Sending push notification to {self.username}: {message}")
        # Push notification logic
        return True
    
    def send_sms_notification(self, phone: str, message: str) -> bool:
        """Send SMS notification"""
        print(f"Sending SMS to {phone}: {message}")
        # SMS sending logic
        return True


# Example usage
if __name__ == "__main__":
    user_service = ViolationUserService("john_doe", "john@example.com")
    
    # Validate user data
    if not user_service.validate_email() or not user_service.validate_username():
        print("Invalid user data")
        exit(1)
    
    # Connect to database and save user
    user_service.connect_to_database()
    user_service.save_user()
    
    # Send welcome email
    user_service.send_welcome_email()
    
    # Log activity
    user_service.log_user_activity("User registered")
    
    # Track analytics
    user_service.track_user_registration()
    
    # Display user
    print("User display:", user_service.format_user_for_display())
    
    # Close database connection
    user_service.close_database_connection()