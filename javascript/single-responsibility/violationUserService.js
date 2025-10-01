// VIOLATION: This class has multiple responsibilities
// It handles user data, email sending, and logging
class ViolationUserService {
    constructor(username, email) {
        this.username = username;
        this.email = email;
    }
    
    // User management responsibility
    saveUser() {
        console.log(`Saving user: ${this.username}`);
        // Save user to database
    }
    
    updateUser(newUsername, newEmail) {
        this.username = newUsername;
        this.email = newEmail;
        console.log('User updated');
    }
    
    // Email responsibility (should be separate)
    sendWelcomeEmail() {
        console.log(`Sending welcome email to: ${this.email}`);
        // Email sending logic
    }
    
    sendPasswordResetEmail() {
        console.log(`Sending password reset email to: ${this.email}`);
        // Email sending logic
    }
    
    // Logging responsibility (should be separate)
    logUserActivity(activity) {
        console.log(`[LOG] User ${this.username} performed: ${activity}`);
        // Logging logic
    }
    
    // Validation responsibility (should be separate)
    validateEmail() {
        return this.email && this.email.includes('@');
    }
    
    // Formatting responsibility (should be separate)
    formatUserForDisplay() {
        return `${this.username} (${this.email})`;
    }
    
    // Analytics responsibility (should be separate)
    trackUserEvent(eventName) {
        console.log(`[ANALYTICS] ${this.username}: ${eventName}`);
        // Analytics tracking logic
    }
}

module.exports = ViolationUserService;