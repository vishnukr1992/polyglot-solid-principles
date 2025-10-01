package main

import (
	"fmt"
	"regexp"
	"time"
)

// CORRECT: Each struct/interface has a single responsibility

// User entity - only handles user data
type User struct {
	Username string
	Email    string
}

// NewUser creates a new user
func NewUser(username, email string) *User {
	return &User{
		Username: username,
		Email:    email,
	}
}

// User repository interface - defines user persistence operations
type UserRepository interface {
	Save(user *User) error
	Update(user *User) error
	FindByID(id string) (*User, error)
}

// Concrete implementation of UserRepository
type DatabaseUserRepository struct{}

func NewDatabaseUserRepository() *DatabaseUserRepository {
	return &DatabaseUserRepository{}
}

func (r *DatabaseUserRepository) Save(user *User) error {
	fmt.Printf("Saving user to database: %s\n", user.Username)
	// Database save logic
	return nil
}

func (r *DatabaseUserRepository) Update(user *User) error {
	fmt.Printf("Updating user in database: %s\n", user.Username)
	// Database update logic
	return nil
}

func (r *DatabaseUserRepository) FindByID(id string) (*User, error) {
	fmt.Printf("Finding user with ID: %s\n", id)
	// Database query logic
	return NewUser("john_doe", "john@example.com"), nil
}

// Email service interface - defines email operations
type EmailService interface {
	SendWelcomeEmail(email string) error
	SendPasswordResetEmail(email string) error
	SendNotificationEmail(email, subject, message string) error
}

// Concrete implementation of EmailService
type SMTPEmailService struct{}

func NewSMTPEmailService() *SMTPEmailService {
	return &SMTPEmailService{}
}

func (e *SMTPEmailService) SendWelcomeEmail(email string) error {
	fmt.Printf("Sending welcome email to: %s\n", email)
	// SMTP email sending logic
	return nil
}

func (e *SMTPEmailService) SendPasswordResetEmail(email string) error {
	fmt.Printf("Sending password reset email to: %s\n", email)
	// SMTP email sending logic
	return nil
}

func (e *SMTPEmailService) SendNotificationEmail(email, subject, message string) error {
	fmt.Printf("Sending notification to %s: %s\n", email, subject)
	// SMTP email sending logic
	return nil
}

// Logger interface - defines logging operations
type ActivityLogger interface {
	LogUserActivity(username, activity string)
	LogError(err error, context string)
}

// Concrete implementation of ActivityLogger
type FileActivityLogger struct{}

func NewFileActivityLogger() *FileActivityLogger {
	return &FileActivityLogger{}
}

func (l *FileActivityLogger) LogUserActivity(username, activity string) {
	timestamp := time.Now().Format(time.RFC3339)
	fmt.Printf("[LOG %s] User %s performed: %s\n", timestamp, username, activity)
	// File logging logic
}

func (l *FileActivityLogger) LogError(err error, context string) {
	timestamp := time.Now().Format(time.RFC3339)
	fmt.Printf("[ERROR %s] %s: %s\n", timestamp, context, err.Error())
	// Error logging logic
}

// Validator interface - defines validation operations
type UserValidator interface {
	ValidateEmail(email string) bool
	ValidateUsername(username string) bool
	ValidateUser(user *User) bool
}

// Concrete implementation of UserValidator
type DefaultUserValidator struct{}

func NewDefaultUserValidator() *DefaultUserValidator {
	return &DefaultUserValidator{}
}

func (v *DefaultUserValidator) ValidateEmail(email string) bool {
	emailRegex := regexp.MustCompile(`^[^\s@]+@[^\s@]+\.[^\s@]+$`)
	return emailRegex.MatchString(email)
}

func (v *DefaultUserValidator) ValidateUsername(username string) bool {
	return len(username) >= 3 && len(username) <= 20
}

func (v *DefaultUserValidator) ValidateUser(user *User) bool {
	return v.ValidateEmail(user.Email) && v.ValidateUsername(user.Username)
}

// Formatter interface - defines formatting operations
type UserFormatter interface {
	FormatUserForDisplay(user *User) string
	FormatUserForAPI(user *User) map[string]string
}

// Concrete implementation of UserFormatter
type DefaultUserFormatter struct{}

func NewDefaultUserFormatter() *DefaultUserFormatter {
	return &DefaultUserFormatter{}
}

func (f *DefaultUserFormatter) FormatUserForDisplay(user *User) string {
	return fmt.Sprintf("%s (%s)", user.Username, user.Email)
}

func (f *DefaultUserFormatter) FormatUserForAPI(user *User) map[string]string {
	return map[string]string{
		"username": user.Username,
		"email":    user.Email,
	}
}

// Analytics service interface - defines analytics operations
type AnalyticsService interface {
	TrackUserEvent(username, eventName string, metadata map[string]interface{})
	TrackUserRegistration(username string)
	TrackUserUpdate(username string)
}

// Concrete implementation of AnalyticsService
type GoogleAnalyticsService struct{}

func NewGoogleAnalyticsService() *GoogleAnalyticsService {
	return &GoogleAnalyticsService{}
}

func (a *GoogleAnalyticsService) TrackUserEvent(username, eventName string, metadata map[string]interface{}) {
	timestamp := time.Now().Unix()
	fmt.Printf("[ANALYTICS] %s: %s (timestamp: %d)\n", username, eventName, timestamp)
	// Analytics tracking logic
}

func (a *GoogleAnalyticsService) TrackUserRegistration(username string) {
	a.TrackUserEvent(username, "user_registered", nil)
}

func (a *GoogleAnalyticsService) TrackUserUpdate(username string) {
	a.TrackUserEvent(username, "user_updated", nil)
}

// User service - orchestrates operations using other services
type CorrectUserService struct {
	userRepo  UserRepository
	emailSvc  EmailService
	logger    ActivityLogger
	validator UserValidator
	formatter UserFormatter
	analytics AnalyticsService
}

// NewCorrectUserService creates a new user service with all dependencies
func NewCorrectUserService(
	userRepo UserRepository,
	emailSvc EmailService,
	logger ActivityLogger,
	validator UserValidator,
	formatter UserFormatter,
	analytics AnalyticsService,
) *CorrectUserService {
	return &CorrectUserService{
		userRepo:  userRepo,
		emailSvc:  emailSvc,
		logger:    logger,
		validator: validator,
		formatter: formatter,
		analytics: analytics,
	}
}

func (s *CorrectUserService) CreateUser(username, email string) (*User, error) {
	// Validate input
	user := NewUser(username, email)
	if !s.validator.ValidateUser(user) {
		err := fmt.Errorf("invalid user data: username=%s, email=%s", username, email)
		s.logger.LogError(err, "User creation failed")
		return nil, err
	}

	// Save user
	if err := s.userRepo.Save(user); err != nil {
		s.logger.LogError(err, "Failed to save user")
		return nil, err
	}

	// Send welcome email
	if err := s.emailSvc.SendWelcomeEmail(email); err != nil {
		s.logger.LogError(err, "Failed to send welcome email")
		// Don't return error for email failure, just log it
	}

	// Log activity
	s.logger.LogUserActivity(username, "User created")

	// Track analytics
	s.analytics.TrackUserRegistration(username)

	return user, nil
}

func (s *CorrectUserService) UpdateUser(user *User, newUsername, newEmail string) error {
	// Validate input
	tempUser := NewUser(newUsername, newEmail)
	if !s.validator.ValidateUser(tempUser) {
		err := fmt.Errorf("invalid user data: username=%s, email=%s", newUsername, newEmail)
		s.logger.LogError(err, "User update failed")
		return err
	}

	// Update user
	user.Username = newUsername
	user.Email = newEmail

	if err := s.userRepo.Update(user); err != nil {
		s.logger.LogError(err, "Failed to update user")
		return err
	}

	// Log activity
	s.logger.LogUserActivity(newUsername, "User updated")

	// Track analytics
	s.analytics.TrackUserUpdate(newUsername)

	return nil
}

func (s *CorrectUserService) FormatUser(user *User) string {
	return s.formatter.FormatUserForDisplay(user)
}

// Example usage with dependency injection - uncomment to run
/*
func main() {
	// Create service dependencies
	userRepo := NewDatabaseUserRepository()
	emailSvc := NewSMTPEmailService()
	logger := NewFileActivityLogger()
	validator := NewDefaultUserValidator()
	formatter := NewDefaultUserFormatter()
	analytics := NewGoogleAnalyticsService()

	// Create user service with dependencies
	userService := NewCorrectUserService(
		userRepo,
		emailSvc,
		logger,
		validator,
		formatter,
		analytics,
	)

	// Create a new user
	user, err := userService.CreateUser("john_doe", "john@example.com")
	if err != nil {
		log.Fatal("Failed to create user:", err)
	}

	fmt.Println("User created:", userService.FormatUser(user))

	// Update the user
	err = userService.UpdateUser(user, "john_smith", "john.smith@example.com")
	if err != nil {
		log.Fatal("Failed to update user:", err)
	}

	fmt.Println("User updated:", userService.FormatUser(user))
}
*/
