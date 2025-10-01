// VIOLATION: This class has multiple responsibilities
// It handles user data, email sending, and logging
public class ViolationUserService {
    private String username;
    private String email;
    
    public ViolationUserService(String username, String email) {
        this.username = username;
        this.email = email;
    }
    
    // User management responsibility
    public void saveUser() {
        // Save user to database
        System.out.println("Saving user: " + username);
    }
    
    public void updateUser(String newUsername, String newEmail) {
        this.username = newUsername;
        this.email = newEmail;
        System.out.println("User updated");
    }
    
    // Email responsibility (should be separate)
    public void sendWelcomeEmail() {
        System.out.println("Sending welcome email to: " + email);
        // Email sending logic
    }
    
    public void sendPasswordResetEmail() {
        System.out.println("Sending password reset email to: " + email);
        // Email sending logic
    }
    
    // Logging responsibility (should be separate)
    public void logUserActivity(String activity) {
        System.out.println("[LOG] User " + username + " performed: " + activity);
        // Logging logic
    }
    
    // Validation responsibility (should be separate)
    public boolean validateEmail() {
        return email != null && email.contains("@");
    }
}