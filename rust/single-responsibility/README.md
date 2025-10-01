# Single Responsibility Principle (SRP) - Rust

The Single Responsibility Principle states that **a struct should have only one reason to change**.

## Violation Example: `violation_user_service.rs`

The violation example shows a struct that has multiple responsibilities:

1. **User Management** - saving, updating, and deleting users
2. **Email Operations** - sending welcome, password reset, and notification emails  
3. **Logging** - recording user activities and errors
4. **Validation** - validating email, username, and password format
5. **Formatting** - formatting user data for display and API
6. **Analytics** - tracking user events and registration
7. **Database Operations** - connecting, executing queries, and closing connections
8. **Notifications** - sending push notifications and SMS
9. **File Operations** - exporting to CSV and backup functionality

### Problems with this approach:
- **High coupling** - changes to email logic require modifying the user service
- **Difficult testing** - hard to test user operations independently from email/logging/analytics
- **Code duplication** - email, logging, and database logic might be duplicated across services
- **Violates SRP** - the struct has multiple reasons to change
- **Poor reusability** - can't reuse individual services in other contexts
- **Hard to mock** - difficult to mock dependencies for testing
- **Maintenance nightmare** - changes to one feature can break others
- **Memory safety concerns** - complex state management in a single struct

## Correct Example: `correct_user_service.rs`

The correct implementation separates responsibilities into different traits and structs:

### Traits (Interfaces):
1. **`UserRepository`** - Defines user persistence operations
2. **`EmailService`** - Defines email operations
3. **`ActivityLogger`** - Defines logging operations
4. **`UserValidator`** - Defines validation operations
5. **`UserFormatter`** - Defines formatting operations
6. **`AnalyticsService`** - Defines analytics operations
7. **`NotificationService`** - Defines notification operations

### Concrete Implementations:
1. **`User`** - Data entity representing a user with controlled access
2. **`DatabaseUserRepository`** - Handles user persistence to database
3. **`SMTPEmailService`** - Manages SMTP email operations
4. **`FileActivityLogger`** - Handles file-based logging
5. **`DefaultUserValidator`** - Validates user data with regex patterns
6. **`DefaultUserFormatter`** - Formats user data for display/API/CSV
7. **`GoogleAnalyticsService`** - Tracks events with Google Analytics
8. **`FirebaseNotificationService`** - Sends push and SMS notifications
9. **`CorrectUserService`** - Orchestrates operations using other services

### Benefits of this approach:
- **Single responsibility** - each struct/trait has one clear purpose
- **Easy testing** - each component can be tested independently with mocks
- **Loose coupling** - changes to one service don't affect others
- **Trait-based design** - easy to swap implementations
- **Dependency injection** - services are injected via generics, making them testable
- **Reusability** - services can be reused across different parts of the application
- **Maintainability** - easier to understand, debug, and modify
- **Memory safety** - Rust's ownership system prevents data races
- **Zero-cost abstractions** - traits are resolved at compile time

### Rust-Specific Benefits:
- **Trait system** - powerful interface definition with default implementations
- **Generics** - compile-time polymorphism for dependency injection
- **Ownership** - automatic memory management without garbage collection
- **Error handling** - explicit error handling with `Result<T, E>`
- **Pattern matching** - powerful control flow with `match` and `if let`
- **Zero runtime cost** - traits and generics have no runtime overhead
- **Thread safety** - built-in thread safety with `Send` and `Sync` traits

## Key Takeaways

- Use traits to define contracts and enable dependency injection
- Break down complex structs into smaller, focused components
- Each struct should have a single, well-defined responsibility
- Use generics for compile-time dependency injection
- Leverage Rust's ownership system for safe memory management
- Use `Result<T, E>` for explicit error handling
- This makes code more testable, maintainable, and memory-safe

## Cargo.toml Dependencies

To run these examples, you'll need these dependencies in your `Cargo.toml`:

```toml
[dependencies]
chrono = { version = "0.4", features = ["serde"] }
regex = "1.0"
```

## Usage Example

To run either example, uncomment the main function in the respective file and run:

```bash
# For violation example
cargo run --bin violation_user_service

# For correct example  
cargo run --bin correct_user_service
```

Or create a separate `main.rs` file:

```rust
use correct_user_service::create_user_service;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create user service with all dependencies
    let user_service = create_user_service();
    
    // Create a new user
    let mut user = user_service.create_user(
        "john_doe".to_string(), 
        "john@example.com".to_string()
    )?;
    println!("User created: {}", user_service.format_user(&user));
    
    // Update the user
    user_service.update_user(
        &mut user, 
        "john_smith".to_string(), 
        "john.smith@example.com".to_string()
    )?;
    println!("User updated: {}", user_service.format_user(&user));
    
    Ok(())
}
```

## Testing Benefits

With the correct implementation, you can easily test individual components:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    // Mock implementation for testing
    struct MockUserRepository;
    
    impl UserRepository for MockUserRepository {
        fn save(&self, user: &User) -> Result<bool, String> {
            // Mock implementation
            Ok(true)
        }
        
        // ... other methods
    }
    
    #[test]
    fn test_user_creation() {
        let mock_repo = MockUserRepository;
        // ... other mocks
        
        let service = CorrectUserService::new(
            mock_repo,
            // ... other dependencies
        );
        
        let result = service.create_user("test".to_string(), "test@example.com".to_string());
        assert!(result.is_ok());
    }
}
```

## Performance Considerations

- **Zero-cost abstractions** - traits are resolved at compile time
- **No heap allocations** - careful use of borrowing reduces allocations
- **Memory safety** - no runtime overhead for memory safety
- **Compile-time optimization** - monomorphization optimizes generic code