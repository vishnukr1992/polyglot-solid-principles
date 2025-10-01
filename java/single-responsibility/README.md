# Single Responsibility Principle (SRP) - Java

The Single Responsibility Principle states that **a class should have only one reason to change**.

## Violation Example: `ViolationUserService.java`

The violation example shows a class that has multiple responsibilities:

1. **User Management** - saving and updating users
2. **Email Operations** - sending welcome and password reset emails  
3. **Logging** - recording user activities
4. **Validation** - validating email format

### Problems with this approach:
- **High coupling** - changes to email logic require modifying the user service
- **Difficult testing** - hard to test user operations independently from email/logging
- **Code duplication** - email and logging logic might be duplicated across services
- **Violates SRP** - the class has multiple reasons to change

## Correct Example: `CorrectUserService.java`

The correct implementation separates responsibilities into different classes:

1. **`User`** - Data entity representing a user
2. **`UserRepository`** - Handles user persistence operations
3. **`EmailService`** - Manages all email operations
4. **`ActivityLogger`** - Handles logging functionality
5. **`UserValidator`** - Validates user data
6. **`CorrectUserService`** - Orchestrates operations using other services

### Benefits of this approach:
- **Single responsibility** - each class has one clear purpose
- **Easy testing** - each component can be tested independently
- **Loose coupling** - changes to one service don't affect others
- **Reusability** - services can be reused in other parts of the application
- **Maintainability** - easier to understand and modify

## Key Takeaways

- Break down complex classes into smaller, focused classes
- Each class should have a single, well-defined responsibility
- Use dependency injection to combine services
- This makes code more testable, maintainable, and flexible