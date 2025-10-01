// CORRECT: Each class has a single responsibility

// User entity - only handles user data
class User {
    constructor(username, email) {
        this.username = username;
        this.email = email;
    }
    
    getUsername() { return this.username; }
    getEmail() { return this.email; }
    
    setUsername(username) { this.username = username; }
    setEmail(email) { this.email = email; }
}

// User repository - only handles user persistence
class UserRepository {
    save(user) {
        console.log(`Saving user: ${user.getUsername()}`);
        // Database save logic
        return Promise.resolve(user);
    }
    
    update(user) {
        console.log(`Updating user: ${user.getUsername()}`);
        // Database update logic
        return Promise.resolve(user);
    }
    
    findById(id) {
        console.log(`Finding user with ID: ${id}`);
        // Database query logic
        return Promise.resolve(new User('john_doe', 'john@example.com'));
    }
}

// Email service - only handles email operations
class EmailService {
    sendWelcomeEmail(email) {
        console.log(`Sending welcome email to: ${email}`);
        // Email sending logic
        return Promise.resolve();
    }
    
    sendPasswordResetEmail(email) {
        console.log(`Sending password reset email to: ${email}`);
        // Email sending logic
        return Promise.resolve();
    }
    
    sendNotificationEmail(email, subject, message) {
        console.log(`Sending notification to ${email}: ${subject}`);
        // Email sending logic
        return Promise.resolve();
    }
}

// Logger - only handles logging
class ActivityLogger {
    logUserActivity(username, activity) {
        const timestamp = new Date().toISOString();
        console.log(`[LOG ${timestamp}] User ${username} performed: ${activity}`);
        // Logging logic
    }
    
    logError(error, context) {
        const timestamp = new Date().toISOString();
        console.error(`[ERROR ${timestamp}] ${context}: ${error.message}`);
        // Error logging logic
    }
}

// Validator - only handles validation
class UserValidator {
    validateEmail(email) {
        const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
        return email && emailRegex.test(email);
    }
    
    validateUsername(username) {
        return username && username.length >= 3 && username.length <= 20;
    }
    
    validateUserData(userData) {
        return this.validateEmail(userData.email) && 
               this.validateUsername(userData.username);
    }
}

// Formatter - only handles formatting
class UserFormatter {
    formatUserForDisplay(user) {
        return `${user.getUsername()} (${user.getEmail()})`;
    }
    
    formatUserForAPI(user) {
        return {
            username: user.getUsername(),
            email: user.getEmail()
        };
    }
}

// Analytics service - only handles analytics
class AnalyticsService {
    trackUserEvent(username, eventName, metadata = {}) {
        const timestamp = Date.now();
        console.log(`[ANALYTICS] ${username}: ${eventName}`, { timestamp, ...metadata });
        // Analytics tracking logic
    }
    
    trackUserRegistration(username) {
        this.trackUserEvent(username, 'user_registered');
    }
    
    trackUserUpdate(username) {
        this.trackUserEvent(username, 'user_updated');
    }
}

// User service - orchestrates operations using other services
class CorrectUserService {
    constructor() {
        this.userRepository = new UserRepository();
        this.emailService = new EmailService();
        this.logger = new ActivityLogger();
        this.validator = new UserValidator();
        this.formatter = new UserFormatter();
        this.analytics = new AnalyticsService();
    }
    
    async createUser(username, email) {
        try {
            // Validate input
            if (!this.validator.validateEmail(email)) {
                throw new Error('Invalid email format');
            }
            if (!this.validator.validateUsername(username)) {
                throw new Error('Invalid username format');
            }
            
            // Create and save user
            const user = new User(username, email);
            await this.userRepository.save(user);
            
            // Send welcome email
            await this.emailService.sendWelcomeEmail(email);
            
            // Log activity
            this.logger.logUserActivity(username, 'User created');
            
            // Track analytics
            this.analytics.trackUserRegistration(username);
            
            return user;
        } catch (error) {
            this.logger.logError(error, 'User creation failed');
            throw error;
        }
    }
    
    async updateUser(user, newUsername, newEmail) {
        try {
            // Validate input
            if (!this.validator.validateEmail(newEmail)) {
                throw new Error('Invalid email format');
            }
            if (!this.validator.validateUsername(newUsername)) {
                throw new Error('Invalid username format');
            }
            
            // Update user
            user.setUsername(newUsername);
            user.setEmail(newEmail);
            await this.userRepository.update(user);
            
            // Log activity
            this.logger.logUserActivity(newUsername, 'User updated');
            
            // Track analytics
            this.analytics.trackUserUpdate(newUsername);
            
            return user;
        } catch (error) {
            this.logger.logError(error, 'User update failed');
            throw error;
        }
    }
    
    formatUser(user) {
        return this.formatter.formatUserForDisplay(user);
    }
}

module.exports = {
    User,
    UserRepository,
    EmailService,
    ActivityLogger,
    UserValidator,
    UserFormatter,
    AnalyticsService,
    CorrectUserService
};