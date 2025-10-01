# Single Responsibility Principle (SRP) - Go

The Single Responsibility Principle states that **a struct should have only one reason to change**.

## Violation Example: `violation_user_service.go`

The violation example shows a struct that has multiple responsibilities:

1. **User Management** - saving and updating users
2. **Email Operations** - sending welcome and password reset emails  
3. **Logging** - recording user activities
4. **Validation** - validating email and username format
5. **Formatting** - formatting user data for display
6. **Analytics** - tracking user events
7. **Database Operations** - connecting and executing queries

### Problems with this approach:
- **High coupling** - changes to email logic require modifying the user service
- **Difficult testing** - hard to test user operations independently from email/logging/analytics
- **Code duplication** - email, logging, and database logic might be duplicated across services
- **Violates SRP** - the struct has multiple reasons to change
- **Poor reusability** - can't reuse individual services in other contexts
- **Hard to mock** - difficult to mock dependencies for testing

## Correct Example: `correct_user_service.go`

The correct implementation separates responsibilities into different interfaces and structs:

### Interfaces (Contracts):
1. **`UserRepository`** - Defines user persistence operations
2. **`EmailService`** - Defines email operations
3. **`ActivityLogger`** - Defines logging operations
4. **`UserValidator`** - Defines validation operations
5. **`UserFormatter`** - Defines formatting operations
6. **`AnalyticsService`** - Defines analytics operations

### Concrete Implementations:
1. **`User`** - Data entity representing a user
2. **`DatabaseUserRepository`** - Handles user persistence to database
3. **`SMTPEmailService`** - Manages SMTP email operations
4. **`FileActivityLogger`** - Handles file-based logging
5. **`DefaultUserValidator`** - Validates user data with regex
6. **`DefaultUserFormatter`** - Formats user data for display/API
7. **`GoogleAnalyticsService`** - Tracks events with Google Analytics
8. **`CorrectUserService`** - Orchestrates operations using other services

### Benefits of this approach:
- **Single responsibility** - each struct/interface has one clear purpose
- **Easy testing** - each component can be tested independently with mocks
- **Loose coupling** - changes to one service don't affect others
- **Interface-based design** - easy to swap implementations
- **Dependency injection** - services are injected, making them testable
- **Reusability** - services can be reused across different parts of the application
- **Maintainability** - easier to understand, debug, and modify

### Go-Specific Benefits:
- **Interface segregation** - small, focused interfaces
- **Structural typing** - no explicit interface implementation needed
- **Dependency injection** - constructor functions accept interfaces
- **Error handling** - proper error propagation and logging
- **Package organization** - each service could be in its own package

## Key Takeaways

- Use interfaces to define contracts and enable dependency injection
- Break down complex structs into smaller, focused components
- Each struct should have a single, well-defined responsibility
- Use constructor functions to inject dependencies
- Leverage Go's interface system for loose coupling
- This makes code more testable, maintainable, and flexible

## Usage Example

To run either example, uncomment the main function in the respective file and run:

```bash
# For violation example
go run violation_user_service.go

# For correct example  
go run correct_user_service.go
```

Or create a separate main.go file that imports and uses the services.