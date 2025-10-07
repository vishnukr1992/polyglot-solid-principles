// Interface Segregation Principle (ISP) - VIOLATION Example (Go)
//
// This example demonstrates how forcing a Robot to implement a Human interface
// violates the Interface Segregation Principle.
//
// Problem: Robot is forced to implement biological methods that make no sense
// for a robot, leading to empty implementations or panics.

package main

import (
	"errors"
	"fmt"
)

// VIOLATION: Monolithic interface forces all implementations to have methods they don't need
type Human interface {
	Work()
	Eat() error
	Sleep() error
	Breathe() error
	Think()
	Reproduce() error
	Exercise() error
	Socialize()
	FeelEmotions() error
	Dream() error
}

// Person implementation - naturally implements all interface methods
type Person struct {
	Name    string
	Energy  int
	IsAwake bool
}

func NewPerson(name string) *Person {
	return &Person{
		Name:    name,
		Energy:  100,
		IsAwake: true,
	}
}

func (p *Person) Work() {
	if !p.IsAwake {
		fmt.Printf("%s cannot work while sleeping!\n", p.Name)
		return
	}
	fmt.Printf("%s is working productively\n", p.Name)
	p.Energy -= 20
}

func (p *Person) Eat() error {
	fmt.Printf("%s is eating delicious food\n", p.Name)
	p.Energy += 30
	return nil
}

func (p *Person) Sleep() error {
	fmt.Printf("%s is sleeping peacefully\n", p.Name)
	p.IsAwake = false
	p.Energy = 100
	return nil
}

func (p *Person) Breathe() error {
	fmt.Printf("%s is breathing fresh air\n", p.Name)
	return nil
}

func (p *Person) Think() {
	fmt.Printf("%s is thinking creatively\n", p.Name)
	p.Energy -= 5
}

func (p *Person) Reproduce() error {
	fmt.Printf("%s can participate in reproduction\n", p.Name)
	return nil
}

func (p *Person) Exercise() error {
	fmt.Printf("%s is doing physical exercise\n", p.Name)
	p.Energy -= 15
	return nil
}

func (p *Person) Socialize() {
	fmt.Printf("%s is socializing with friends\n", p.Name)
	p.Energy -= 10
}

func (p *Person) FeelEmotions() error {
	fmt.Printf("%s is experiencing complex emotions\n", p.Name)
	return nil
}

func (p *Person) Dream() error {
	if !p.IsAwake {
		fmt.Printf("%s is dreaming while sleeping\n", p.Name)
	}
	return nil
}

// VIOLATION: Robot forced to implement Human interface
// This violates ISP because Robot doesn't need most of these methods
type IndustrialRobot struct {
	Model         string
	BatteryLevel  int
	IsOperational bool
}

func NewIndustrialRobot(model string) *IndustrialRobot {
	return &IndustrialRobot{
		Model:         model,
		BatteryLevel:  100,
		IsOperational: true,
	}
}

func (r *IndustrialRobot) Work() {
	if !r.IsOperational {
		fmt.Printf("%s is not operational!\n", r.Model)
		return
	}
	fmt.Printf("%s robot is performing industrial work\n", r.Model)
	r.BatteryLevel -= 10
}

// VIOLATION: Forced to implement irrelevant methods
func (r *IndustrialRobot) Eat() error {
	// Makes no sense for a robot!
	return errors.New("robots don't eat food")
}

func (r *IndustrialRobot) Sleep() error {
	// Robots don't sleep, they power down
	return errors.New("robots don't sleep! Use PowerDown() instead")
}

func (r *IndustrialRobot) Breathe() error {
	// Robots don't breathe
	return errors.New("robots don't breathe air")
}

func (r *IndustrialRobot) Think() {
	// Different kind of thinking (computational)
	fmt.Printf("%s is processing data algorithmically\n", r.Model)
	r.BatteryLevel -= 2
}

func (r *IndustrialRobot) Reproduce() error {
	// Robots are manufactured, not reproduced
	return errors.New("robots are manufactured, not reproduced")
}

func (r *IndustrialRobot) Exercise() error {
	// Robots don't need exercise
	return errors.New("robots don't need exercise")
}

func (r *IndustrialRobot) Socialize() {
	// Basic communication only
	fmt.Printf("%s is communicating via network protocols\n", r.Model)
}

func (r *IndustrialRobot) FeelEmotions() error {
	// Robots don't feel emotions
	return errors.New("robots don't have emotions")
}

func (r *IndustrialRobot) Dream() error {
	// Robots don't dream
	return errors.New("robots don't dream")
}

// Robot-specific methods that don't fit in Human interface
func (r *IndustrialRobot) PowerDown() {
	fmt.Printf("%s is powering down systems\n", r.Model)
	r.IsOperational = false
}

func (r *IndustrialRobot) Recharge() {
	fmt.Printf("%s is recharging battery\n", r.Model)
	r.BatteryLevel = 100
}

func (r *IndustrialRobot) RunDiagnostics() {
	fmt.Printf("%s is running system diagnostics\n", r.Model)
	status := "Offline"
	if r.IsOperational {
		status = "Operational"
	}
	fmt.Printf("Battery: %d%%, Status: %s\n", r.BatteryLevel, status)
}

func (r *IndustrialRobot) ExecuteProgram(program string) {
	fmt.Printf("%s is executing program: %s\n", r.Model, program)
}

// Client code that expects Human interface
type WorkManager struct{}

// This method expects all Human interface methods to work
func (wm *WorkManager) ManageWorker(worker Human) {
	fmt.Println("Managing worker...")

	// This works for both
	worker.Work()

	// These fail for Robot!
	if err := worker.Eat(); err != nil {
		fmt.Printf("Error: %s\n", err.Error())
	}

	if err := worker.Sleep(); err != nil {
		fmt.Printf("Error: %s\n", err.Error())
	}

	worker.Think() // Different behavior for robot

	if err := worker.Exercise(); err != nil {
		fmt.Printf("Error: %s\n", err.Error())
	}

	if err := worker.FeelEmotions(); err != nil {
		fmt.Printf("Error: %s\n", err.Error())
	}
}

// This method only needs work capability but is forced to accept full Human interface
func (wm *WorkManager) AssignWork(worker Human, task string) {
	fmt.Printf("Assigning task: %s\n", task)
	// We only need Work() method, but must accept entire Human interface
	worker.Work()
}

// Expects Human biological needs
func (wm *WorkManager) ProvideCare(human Human) error {
	fmt.Println("Providing human care...")

	if err := human.Eat(); err != nil {
		return fmt.Errorf("failed to provide food: %w", err)
	}

	if err := human.Sleep(); err != nil {
		return fmt.Errorf("failed to provide rest: %w", err)
	}

	if err := human.Breathe(); err != nil {
		return fmt.Errorf("failed to provide air: %w", err)
	}

	return nil
}

// Demonstrates the ISP violation
func mainInCorrect() {
	fmt.Println("=== Interface Segregation Principle - VIOLATION (Go) ===")
	fmt.Println()

	fmt.Println("1. Creating human worker:")
	person := NewPerson("Alice")

	fmt.Println("\n2. Creating robot worker:")
	robot := NewIndustrialRobot("R2D2-Industrial")

	wm := &WorkManager{}

	fmt.Println("\n3. Managing human worker (works fine):")
	wm.ManageWorker(person)

	fmt.Println("\n4. Managing robot worker (many failures!):")
	wm.ManageWorker(robot)

	fmt.Println("\n5. Attempting to provide human care to robot:")
	if err := wm.ProvideCare(robot); err != nil {
		fmt.Printf("Failed to provide care: %s\n", err.Error())
	}

	fmt.Println("\n6. Problems with this design:")
	fmt.Println("   - Robot forced to implement methods it doesn't need")
	fmt.Println("   - Many methods return errors instead of working")
	fmt.Println("   - Clients can't rely on interface methods working")
	fmt.Println("   - Robot-specific methods don't fit Human interface")
	fmt.Println("   - Violates ISP: clients forced to depend on methods they don't use")

	// Try to access robot-specific functionality
	fmt.Println("\n7. Robot-specific operations (require type assertion):")
	// Since robot is already *IndustrialRobot, we can call methods directly
	robot.PowerDown()
	robot.Recharge()
	robot.RunDiagnostics()
	robot.ExecuteProgram("Industrial Assembly v2.1")

	fmt.Println("\n=== Conclusion ===")
	fmt.Println("The monolithic Human interface forces Robot to implement")
	fmt.Println("irrelevant methods, violating the Interface Segregation Principle.")
	fmt.Println("Clients depending on Human interface cannot rely on all methods working.")
}
