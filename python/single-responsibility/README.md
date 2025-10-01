# Single Responsibility Principle (SRP) - Python

The Single Responsibility Principle states that **a class should have only one reason to change**.

## Violation Example: `violation_user_service.py`

The violation example shows a class that has multiple responsibilities:

1. **User Management** - saving, updating, and deleting users
2. **Email Operations** - sending welcome, password reset, and notification emails  
3. **Logging** - recording user activities and errors
4. **Validation** - validating email, username, and password format
5. **Formatting** - formatting user data for display and API
6. **Analytics** - tracking user events and registration
7. **Database Operations** - connecting, executing queries, and closing connections
8. **Notifications** - sending push notifications and SMS

### Problems with this approach:
- **High coupling** - changes to email logic require modifying the user service
- **Difficult testing** - hard to test user operations independently from email/logging/analytics
- **Code duplication** - email, logging, and database logic might be duplicated across services
- **Violates SRP** - the class has multiple reasons to change
- **Poor reusability** - can't reuse individual services in other contexts
- **Hard to mock** - difficult to mock dependencies for testing
- **Maintenance nightmare** - changes to one feature can break others

## Correct Example: `correct_user_service.py`

The correct implementation separates responsibilities into different abstract base classes and concrete implementations:

### Abstract Base Classes (Interfaces):
1. **`UserRepository`** - Defines user persistence operations
2. **`EmailService`** - Defines email operations
3. **`ActivityLogger`** - Defines logging operations
4. **`UserValidator`** - Defines validation operations
5. **`UserFormatter`** - Defines formatting operations
6. **`AnalyticsService`** - Defines analytics operations
7. **`NotificationService`** - Defines notification operations

### Concrete Implementations:
1. **`User`** - Data entity representing a user with properties
2. **`DatabaseUserRepository`** - Handles user persistence to database
3. **`SMTPEmailService`** - Manages SMTP email operations
4. **`FileActivityLogger`** - Handles file-based logging
5. **`DefaultUserValidator`** - Validates user data with regex patterns
6. **`DefaultUserFormatter`** - Formats user data for display/API/CSV
7. **`GoogleAnalyticsService`** - Tracks events with Google Analytics
8. **`FirebaseNotificationService`** - Sends push and SMS notifications
9. **`CorrectUserService`** - Orchestrates operations using other services

### Benefits of this approach:
- **Single responsibility** - each class has one clear purpose
- **Easy testing** - each component can be tested independently with mocks
- **Loose coupling** - changes to one service don't affect others
- **Interface-based design** - easy to swap implementations
- **Dependency injection** - services are injected, making them testable
- **Reusability** - services can be reused across different parts of the application
- **Maintainability** - easier to understand, debug, and modify
- **Error handling** - proper exception handling and logging

### Python-Specific Benefits:
- **Abstract Base Classes** - using `ABC` and `@abstractmethod` for contracts
- **Properties** - using `@property` decorators for controlled access
- **Type hints** - proper type annotations for better IDE support
- **Optional parameters** - using `Optional[]` and `Dict[]` type hints
- **Factory pattern** - `create_user_service()` function for easy setup
- **Context managers** - potential for database connection management
- **Duck typing** - leveraging Python's dynamic nature with interfaces

## Key Takeaways

- Use Abstract Base Classes to define contracts and enable dependency injection
- Break down complex classes into smaller, focused components
- Each class should have a single, well-defined responsibility
- Use dependency injection in constructor methods
- Leverage Python's type system for better code documentation
- Use factory functions for easy service configuration
- This makes code more testable, maintainable, and flexible

## Usage Example

```python
from correct_user_service import create_user_service

# Create user service with all dependencies
user_service = create_user_service()

# Create a new user
user = user_service.create_user("john_doe", "john@example.com")
if user:
    print("User created:", user_service.format_user(user))
    
    # Update the user
    if user_service.update_user(user, "john_smith", "john.smith@example.com"):
        print("User updated:", user_service.format_user(user))
    else:
        print("Failed to update user")
else:
    print("Failed to create user")
```

## Running the Examples

```bash
# Run violation example
python violation_user_service.py

# Run correct example
python correct_user_service.py
```

## Testing Benefits

With the correct implementation, you can easily test individual components:

```python
# Mock example for testing
from unittest.mock import Mock

def test_user_creation():
    # Create mocks
    mock_repo = Mock(spec=UserRepository)
    mock_email = Mock(spec=EmailService)
    mock_logger = Mock(spec=ActivityLogger)
    # ... other mocks
    
    # Create service with mocks
    service = CorrectUserService(mock_repo, mock_email, mock_logger, ...)
    
    # Test user creation
    user = service.create_user("test", "test@example.com")
    
    # Verify interactions
    mock_repo.save.assert_called_once()
    mock_email.send_welcome_email.assert_called_once()
```