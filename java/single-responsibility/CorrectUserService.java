// CORRECT: Each class has a single responsibility

// User entity - only handles user data
class User {
    private String username;
    private String email;
    
    public User(String username, String email) {
        this.username = username;
        this.email = email;
    }
    
    public String getUsername() { return username; }
    public String getEmail() { return email; }
    
    public void setUsername(String username) { this.username = username; }
    public void setEmail(String email) { this.email = email; }
}

// User repository - only handles user persistence
class UserRepository {
    public void save(User user) {
        System.out.println("Saving user: " + user.getUsername());
        // Database save logic
    }
    
    public void update(User user) {
        System.out.println("Updating user: " + user.getUsername());
        // Database update logic
    }
}

// Email service - only handles email operations
class EmailService {
    public void sendWelcomeEmail(String email) {
        System.out.println("Sending welcome email to: " + email);
        // Email sending logic
    }
    
    public void sendPasswordResetEmail(String email) {
        System.out.println("Sending password reset email to: " + email);
        // Email sending logic
    }
}

// Logger - only handles logging
class ActivityLogger {
    public void logUserActivity(String username, String activity) {
        System.out.println("[LOG] User " + username + " performed: " + activity);
        // Logging logic
    }
}

// Validator - only handles validation
class UserValidator {
    public boolean validateEmail(String email) {
        return email != null && email.contains("@");
    }
}

// User service - orchestrates operations using other services
public class CorrectUserService {
    private final UserRepository userRepository;
    private final EmailService emailService;
    private final ActivityLogger logger;
    private final UserValidator validator;
    
    public CorrectUserService() {
        this.userRepository = new UserRepository();
        this.emailService = new EmailService();
        this.logger = new ActivityLogger();
        this.validator = new UserValidator();
    }
    
    public void createUser(String username, String email) {
        if (!validator.validateEmail(email)) {
            throw new IllegalArgumentException("Invalid email format");
        }
        
        User user = new User(username, email);
        userRepository.save(user);
        emailService.sendWelcomeEmail(email);
        logger.logUserActivity(username, "User created");
    }
    
    public void updateUser(User user, String newUsername, String newEmail) {
        if (!validator.validateEmail(newEmail)) {
            throw new IllegalArgumentException("Invalid email format");
        }
        
        user.setUsername(newUsername);
        user.setEmail(newEmail);
        userRepository.update(user);
        logger.logUserActivity(newUsername, "User updated");
    }
}