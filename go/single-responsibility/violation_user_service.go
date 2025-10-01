package main

import (
	"fmt"
	"strings"
	"time"
)

// VIOLATION: This struct has multiple responsibilities
// It handles user data, email sending, logging, validation, and analytics
type ViolationUserService struct {
	Username string
	Email    string
}

// NewViolationUserService creates a new user service
func NewViolationUserService(username, email string) *ViolationUserService {
	return &ViolationUserService{
		Username: username,
		Email:    email,
	}
}

// User management responsibility
func (u *ViolationUserService) SaveUser() error {
	fmt.Printf("Saving user: %s\n", u.Username)
	// Database save logic
	return nil
}

func (u *ViolationUserService) UpdateUser(newUsername, newEmail string) error {
	u.Username = newUsername
	u.Email = newEmail
	fmt.Println("User updated")
	return nil
}

// Email responsibility (should be separate)
func (u *ViolationUserService) SendWelcomeEmail() error {
	fmt.Printf("Sending welcome email to: %s\n", u.Email)
	// Email sending logic
	return nil
}

func (u *ViolationUserService) SendPasswordResetEmail() error {
	fmt.Printf("Sending password reset email to: %s\n", u.Email)
	// Email sending logic
	return nil
}

// Logging responsibility (should be separate)
func (u *ViolationUserService) LogUserActivity(activity string) {
	timestamp := time.Now().Format(time.RFC3339)
	fmt.Printf("[LOG %s] User %s performed: %s\n", timestamp, u.Username, activity)
	// Logging logic
}

// Validation responsibility (should be separate)
func (u *ViolationUserService) ValidateEmail() bool {
	return u.Email != "" && strings.Contains(u.Email, "@")
}

func (u *ViolationUserService) ValidateUsername() bool {
	return len(u.Username) >= 3 && len(u.Username) <= 20
}

// Formatting responsibility (should be separate)
func (u *ViolationUserService) FormatUserForDisplay() string {
	return fmt.Sprintf("%s (%s)", u.Username, u.Email)
}

// Analytics responsibility (should be separate)
func (u *ViolationUserService) TrackUserEvent(eventName string) {
	fmt.Printf("[ANALYTICS] %s: %s\n", u.Username, eventName)
	// Analytics tracking logic
}

// Database responsibility (should be separate)
func (u *ViolationUserService) ConnectToDatabase() error {
	fmt.Println("Connecting to database...")
	// Database connection logic
	return nil
}

func (u *ViolationUserService) ExecuteQuery(query string) error {
	fmt.Printf("Executing query: %s\n", query)
	// Query execution logic
	return nil
}

// Example usage - uncomment to run
/*
func main() {
	userService := NewViolationUserService("john_doe", "john@example.com")

	if !userService.ValidateEmail() || !userService.ValidateUsername() {
		log.Fatal("Invalid user data")
	}

	userService.ConnectToDatabase()
	userService.SaveUser()
	userService.SendWelcomeEmail()
	userService.LogUserActivity("User registered")
	userService.TrackUserEvent("user_registration")

	fmt.Println("User display:", userService.FormatUserForDisplay())
}
*/
