"""
CORRECT: Each class has a single responsibility
Demonstrates proper separation of concerns in Python
"""

import re
import datetime
from abc import ABC, abstractmethod
from typing import Optional, Dict, List, Any


# User entity - only handles user data
class User:
    def __init__(self, username: str, email: str):
        self._username = username
        self._email = email
    
    @property
    def username(self) -> str:
        return self._username
    
    @property
    def email(self) -> str:
        return self._email
    
    @username.setter
    def username(self, value: str) -> None:
        self._username = value
    
    @email.setter
    def email(self, value: str) -> None:
        self._email = value


# User repository interface - defines user persistence operations
class UserRepository(ABC):
    @abstractmethod
    def save(self, user: User) -> bool:
        pass
    
    @abstractmethod
    def update(self, user: User) -> bool:
        pass
    
    @abstractmethod
    def find_by_id(self, user_id: str) -> Optional[User]:
        pass
    
    @abstractmethod
    def delete(self, user: User) -> bool:
        pass


# Concrete implementation of UserRepository
class DatabaseUserRepository(UserRepository):
    def __init__(self):
        self.connection = None
    
    def _connect(self) -> None:
        """Private method to handle database connection"""
        if not self.connection:
            print("Connecting to database...")
            self.connection = "database_connection"
    
    def save(self, user: User) -> bool:
        self._connect()
        print(f"Saving user to database: {user.username}")
        # Database save logic
        return True
    
    def update(self, user: User) -> bool:
        self._connect()
        print(f"Updating user in database: {user.username}")
        # Database update logic
        return True
    
    def find_by_id(self, user_id: str) -> Optional[User]:
        self._connect()
        print(f"Finding user with ID: {user_id}")
        # Database query logic
        return User("john_doe", "john@example.com")
    
    def delete(self, user: User) -> bool:
        self._connect()
        print(f"Deleting user from database: {user.username}")
        # Database delete logic
        return True


# Email service interface - defines email operations
class EmailService(ABC):
    @abstractmethod
    def send_welcome_email(self, email: str) -> bool:
        pass
    
    @abstractmethod
    def send_password_reset_email(self, email: str) -> bool:
        pass
    
    @abstractmethod
    def send_notification_email(self, email: str, subject: str, message: str) -> bool:
        pass


# Concrete implementation of EmailService
class SMTPEmailService(EmailService):
    def __init__(self, smtp_server: str = "smtp.example.com"):
        self.smtp_server = smtp_server
    
    def send_welcome_email(self, email: str) -> bool:
        print(f"Sending welcome email via SMTP to: {email}")
        # SMTP email sending logic
        return True
    
    def send_password_reset_email(self, email: str) -> bool:
        print(f"Sending password reset email via SMTP to: {email}")
        # SMTP email sending logic
        return True
    
    def send_notification_email(self, email: str, subject: str, message: str) -> bool:
        print(f"Sending notification via SMTP to {email}: {subject}")
        # SMTP email sending logic
        return True


# Logger interface - defines logging operations
class ActivityLogger(ABC):
    @abstractmethod
    def log_user_activity(self, username: str, activity: str) -> None:
        pass
    
    @abstractmethod
    def log_error(self, error: str, context: str) -> None:
        pass


# Concrete implementation of ActivityLogger
class FileActivityLogger(ActivityLogger):
    def __init__(self, log_file: str = "activity.log"):
        self.log_file = log_file
    
    def log_user_activity(self, username: str, activity: str) -> None:
        timestamp = datetime.datetime.now().isoformat()
        log_message = f"[LOG {timestamp}] User {username} performed: {activity}"
        print(log_message)
        # Write to file logic
        self._write_to_file(log_message)
    
    def log_error(self, error: str, context: str) -> None:
        timestamp = datetime.datetime.now().isoformat()
        log_message = f"[ERROR {timestamp}] {context}: {error}"
        print(log_message)
        # Write to file logic
        self._write_to_file(log_message)
    
    def _write_to_file(self, message: str) -> None:
        """Private method to write to log file"""
        # File writing logic would go here
        pass


# Validator interface - defines validation operations
class UserValidator(ABC):
    @abstractmethod
    def validate_email(self, email: str) -> bool:
        pass
    
    @abstractmethod
    def validate_username(self, username: str) -> bool:
        pass
    
    @abstractmethod
    def validate_user(self, user: User) -> bool:
        pass


# Concrete implementation of UserValidator
class DefaultUserValidator(UserValidator):
    def validate_email(self, email: str) -> bool:
        email_pattern = r'^[^\s@]+@[^\s@]+\.[^\s@]+$'
        return bool(re.match(email_pattern, email))
    
    def validate_username(self, username: str) -> bool:
        return (3 <= len(username) <= 20 and 
                username.replace('_', '').isalnum())
    
    def validate_password(self, password: str) -> bool:
        """Additional validation method for passwords"""
        return (len(password) >= 8 and 
                any(c.isupper() for c in password) and
                any(c.islower() for c in password) and
                any(c.isdigit() for c in password))
    
    def validate_user(self, user: User) -> bool:
        return (self.validate_email(user.email) and 
                self.validate_username(user.username))


# Formatter interface - defines formatting operations
class UserFormatter(ABC):
    @abstractmethod
    def format_user_for_display(self, user: User) -> str:
        pass
    
    @abstractmethod
    def format_user_for_api(self, user: User) -> Dict[str, Any]:
        pass


# Concrete implementation of UserFormatter
class DefaultUserFormatter(UserFormatter):
    def format_user_for_display(self, user: User) -> str:
        return f"{user.username} ({user.email})"
    
    def format_user_for_api(self, user: User) -> Dict[str, Any]:
        return {
            "username": user.username,
            "email": user.email,
            "display_name": self.format_user_for_display(user)
        }
    
    def format_user_for_csv(self, user: User) -> str:
        """Additional formatting method for CSV export"""
        return f"{user.username},{user.email}"


# Analytics service interface - defines analytics operations
class AnalyticsService(ABC):
    @abstractmethod
    def track_user_event(self, username: str, event_name: str, metadata: Optional[Dict[str, Any]] = None) -> None:
        pass
    
    @abstractmethod
    def track_user_registration(self, username: str) -> None:
        pass
    
    @abstractmethod
    def track_user_update(self, username: str) -> None:
        pass


# Concrete implementation of AnalyticsService
class GoogleAnalyticsService(AnalyticsService):
    def __init__(self, tracking_id: str = "GA-XXXXX-X"):
        self.tracking_id = tracking_id
    
    def track_user_event(self, username: str, event_name: str, metadata: Optional[Dict[str, Any]] = None) -> None:
        timestamp = datetime.datetime.now().timestamp()
        print(f"[ANALYTICS {self.tracking_id}] {username}: {event_name} (timestamp: {timestamp})")
        if metadata:
            print(f"  Metadata: {metadata}")
        # Google Analytics tracking logic
    
    def track_user_registration(self, username: str) -> None:
        self.track_user_event(username, "user_registered")
    
    def track_user_update(self, username: str) -> None:
        self.track_user_event(username, "user_updated")
    
    def track_user_login(self, username: str, ip_address: str) -> None:
        """Additional tracking method for login events"""
        self.track_user_event(username, "user_login", {"ip_address": ip_address})


# Notification service interface - defines notification operations
class NotificationService(ABC):
    @abstractmethod
    def send_push_notification(self, username: str, message: str) -> bool:
        pass
    
    @abstractmethod
    def send_sms_notification(self, phone: str, message: str) -> bool:
        pass


# Concrete implementation of NotificationService
class FirebaseNotificationService(NotificationService):
    def __init__(self, api_key: str = "firebase-api-key"):
        self.api_key = api_key
    
    def send_push_notification(self, username: str, message: str) -> bool:
        print(f"Sending Firebase push notification to {username}: {message}")
        # Firebase push notification logic
        return True
    
    def send_sms_notification(self, phone: str, message: str) -> bool:
        print(f"Sending SMS via Firebase to {phone}: {message}")
        # SMS sending logic
        return True


# User service - orchestrates operations using other services
class CorrectUserService:
    def __init__(self, 
                 user_repo: UserRepository,
                 email_service: EmailService,
                 logger: ActivityLogger,
                 validator: UserValidator,
                 formatter: UserFormatter,
                 analytics: AnalyticsService,
                 notification_service: Optional[NotificationService] = None):
        self.user_repo = user_repo
        self.email_service = email_service
        self.logger = logger
        self.validator = validator
        self.formatter = formatter
        self.analytics = analytics
        self.notification_service = notification_service
    
    def create_user(self, username: str, email: str) -> Optional[User]:
        """Create a new user with validation and notifications"""
        try:
            # Create user object
            user = User(username, email)
            
            # Validate user data
            if not self.validator.validate_user(user):
                error_msg = f"Invalid user data: username={username}, email={email}"
                self.logger.log_error(error_msg, "User creation failed")
                return None
            
            # Save user
            if not self.user_repo.save(user):
                self.logger.log_error("Failed to save user", "User creation failed")
                return None
            
            # Send welcome email
            try:
                self.email_service.send_welcome_email(email)
            except Exception as e:
                self.logger.log_error(str(e), "Failed to send welcome email")
                # Don't fail user creation if email fails
            
            # Log activity
            self.logger.log_user_activity(username, "User created")
            
            # Track analytics
            self.analytics.track_user_registration(username)
            
            # Send push notification if service is available
            if self.notification_service:
                self.notification_service.send_push_notification(
                    username, "Welcome! Your account has been created."
                )
            
            return user
            
        except Exception as e:
            self.logger.log_error(str(e), "User creation failed")
            return None
    
    def update_user(self, user: User, new_username: str, new_email: str) -> bool:
        """Update user information with validation"""
        try:
            # Validate new data
            temp_user = User(new_username, new_email)
            if not self.validator.validate_user(temp_user):
                error_msg = f"Invalid user data: username={new_username}, email={new_email}"
                self.logger.log_error(error_msg, "User update failed")
                return False
            
            # Update user
            user.username = new_username
            user.email = new_email
            
            if not self.user_repo.update(user):
                self.logger.log_error("Failed to update user", "User update failed")
                return False
            
            # Log activity
            self.logger.log_user_activity(new_username, "User updated")
            
            # Track analytics
            self.analytics.track_user_update(new_username)
            
            return True
            
        except Exception as e:
            self.logger.log_error(str(e), "User update failed")
            return False
    
    def format_user(self, user: User) -> str:
        """Format user for display"""
        return self.formatter.format_user_for_display(user)
    
    def get_user_api_data(self, user: User) -> Dict[str, Any]:
        """Get user data formatted for API response"""
        return self.formatter.format_user_for_api(user)


# Factory function to create a fully configured user service
def create_user_service() -> CorrectUserService:
    """Factory function to create user service with all dependencies"""
    user_repo = DatabaseUserRepository()
    email_service = SMTPEmailService()
    logger = FileActivityLogger()
    validator = DefaultUserValidator()
    formatter = DefaultUserFormatter()
    analytics = GoogleAnalyticsService()
    notification_service = FirebaseNotificationService()
    
    return CorrectUserService(
        user_repo=user_repo,
        email_service=email_service,
        logger=logger,
        validator=validator,
        formatter=formatter,
        analytics=analytics,
        notification_service=notification_service
    )


# Example usage with dependency injection
if __name__ == "__main__":
    # Create user service with all dependencies
    user_service = create_user_service()
    
    # Create a new user
    user = user_service.create_user("john_doe", "john@example.com")
    if user:
        print("User created:", user_service.format_user(user))
        
        # Update the user
        if user_service.update_user(user, "john_smith", "john.smith@example.com"):
            print("User updated:", user_service.format_user(user))
            print("API data:", user_service.get_user_api_data(user))
        else:
            print("Failed to update user")
    else:
        print("Failed to create user")