// Interface Segregation Principle (ISP) - CORRECT Example (Go)
//
// This example demonstrates proper interface segregation where clients
// only depend on the methods they actually need.
//
// Solution: Break down the monolithic Human interface into smaller,
// focused interfaces that can be implemented independently.

package main

import (
	"errors"
	"fmt"
)

// CORRECT: Small, focused interfaces following ISP
type Workable interface {
	Work()
}

type Biological interface {
	Eat() error
	Sleep() error
	Breathe() error
}

type Cognitive interface {
	Think()
	Learn() error
}

type Social interface {
	Socialize()
	Communicate()
}

type Physical interface {
	Exercise() error
	Move()
}

type Reproductive interface {
	Reproduce() error
}

// Robot-specific interfaces
type Mechanical interface {
	PowerDown()
	PowerUp()
	RunDiagnostics()
}

type Rechargeable interface {
	Recharge()
	GetBatteryLevel() int
}

type Programmable interface {
	LoadProgram(program string)
	ExecuteProgram() error
}

// Advanced robot capabilities
type NetworkEnabled interface {
	ConnectToNetwork() error
	SendData(data string) error
	ReceiveData() (string, error)
}

// Employee implementation - implements relevant interfaces
type Employee struct {
	Name        string
	Energy      int
	IsAwake     bool
	CurrentTask string
}

func NewEmployee(name string) *Employee {
	return &Employee{
		Name:    name,
		Energy:  100,
		IsAwake: true,
	}
}

// Workable interface
func (e *Employee) Work() {
	if !e.IsAwake {
		fmt.Printf("%s cannot work while sleeping!\n", e.Name)
		return
	}
	task := e.CurrentTask
	if task == "" {
		task = "general tasks"
	}
	fmt.Printf("%s is working on: %s\n", e.Name, task)
	e.Energy -= 20
}

// Biological interface
func (e *Employee) Eat() error {
	fmt.Printf("%s is eating nutritious food\n", e.Name)
	e.Energy += 30
	return nil
}

func (e *Employee) Sleep() error {
	fmt.Printf("%s is sleeping peacefully\n", e.Name)
	e.IsAwake = false
	e.Energy = 100
	return nil
}

func (e *Employee) Breathe() error {
	fmt.Printf("%s is breathing fresh air\n", e.Name)
	return nil
}

// Cognitive interface
func (e *Employee) Think() {
	fmt.Printf("%s is thinking creatively and analytically\n", e.Name)
	e.Energy -= 5
}

func (e *Employee) Learn() error {
	fmt.Printf("%s is learning new skills\n", e.Name)
	e.Energy -= 10
	return nil
}

// Social interface
func (e *Employee) Socialize() {
	fmt.Printf("%s is socializing with friends\n", e.Name)
	e.Energy -= 10
}

func (e *Employee) Communicate() {
	fmt.Printf("%s is communicating through speech\n", e.Name)
}

// Physical interface
func (e *Employee) Exercise() error {
	fmt.Printf("%s is doing physical exercise\n", e.Name)
	e.Energy -= 15
	return nil
}

func (e *Employee) Move() {
	fmt.Printf("%s is walking around\n", e.Name)
	e.Energy -= 5
}

// Reproductive interface
func (e *Employee) Reproduce() error {
	fmt.Printf("%s can participate in human reproduction\n", e.Name)
	return nil
}

func (e *Employee) SetCurrentTask(task string) {
	e.CurrentTask = task
}

// FactoryBot - only implements interfaces it needs
type FactoryBot struct {
	Model          string
	BatteryLevel   int
	IsOperational  bool
	CurrentProgram string
}

func NewFactoryBot(model string) *FactoryBot {
	return &FactoryBot{
		Model:         model,
		BatteryLevel:  100,
		IsOperational: true,
	}
}

// Workable interface
func (r *FactoryBot) Work() {
	if !r.IsOperational {
		fmt.Printf("%s is not operational!\n", r.Model)
		return
	}
	if r.CurrentProgram == "" {
		fmt.Printf("%s has no program loaded!\n", r.Model)
		return
	}
	fmt.Printf("%s robot is executing work program: %s\n", r.Model, r.CurrentProgram)
	r.BatteryLevel -= 10
}

// Mechanical interface
func (r *FactoryBot) PowerDown() {
	fmt.Printf("%s is powering down all systems\n", r.Model)
	r.IsOperational = false
}

func (r *FactoryBot) PowerUp() {
	fmt.Printf("%s is powering up systems\n", r.Model)
	r.IsOperational = true
}

func (r *FactoryBot) RunDiagnostics() {
	fmt.Printf("%s is running comprehensive diagnostics\n", r.Model)
	status := "Offline"
	if r.IsOperational {
		status = "Operational"
	}
	fmt.Printf("Battery: %d%%, Status: %s\n", r.BatteryLevel, status)
}

// Rechargeable interface
func (r *FactoryBot) Recharge() {
	fmt.Printf("%s is recharging battery to full capacity\n", r.Model)
	r.BatteryLevel = 100
}

func (r *FactoryBot) GetBatteryLevel() int {
	return r.BatteryLevel
}

// Programmable interface
func (r *FactoryBot) LoadProgram(program string) {
	fmt.Printf("%s is loading program: %s\n", r.Model, program)
	r.CurrentProgram = program
}

func (r *FactoryBot) ExecuteProgram() error {
	if r.CurrentProgram == "" {
		return errors.New("no program loaded")
	}
	fmt.Printf("%s is executing program: %s\n", r.Model, r.CurrentProgram)
	r.BatteryLevel -= 5
	return nil
}

// AndroidBot - implements more interfaces including some human-like ones
type AndroidBot struct {
	Name             string
	BatteryLevel     int
	IsOperational    bool
	CurrentProgram   string
	NetworkConnected bool
}

func NewAndroidBot(name string) *AndroidBot {
	return &AndroidBot{
		Name:             name,
		BatteryLevel:     100,
		IsOperational:    true,
		NetworkConnected: false,
	}
}

// Workable interface
func (h *AndroidBot) Work() {
	if !h.IsOperational {
		fmt.Printf("%s is not operational!\n", h.Name)
		return
	}
	fmt.Printf("%s humanoid robot is performing complex work tasks\n", h.Name)
	h.BatteryLevel -= 8
}

// Cognitive interface - robots can have AI cognition
func (h *AndroidBot) Think() {
	fmt.Printf("%s is processing information using AI algorithms\n", h.Name)
	h.BatteryLevel -= 3
}

func (h *AndroidBot) Learn() error {
	fmt.Printf("%s is updating neural networks with new data\n", h.Name)
	h.BatteryLevel -= 5
	return nil
}

// Social interface - humanoid robots can be social
func (h *AndroidBot) Socialize() {
	fmt.Printf("%s is engaging in social interaction protocols\n", h.Name)
	h.BatteryLevel -= 4
}

func (h *AndroidBot) Communicate() {
	fmt.Printf("%s is communicating through speech synthesis\n", h.Name)
}

// Physical interface - humanoid robots can move
func (h *AndroidBot) Move() {
	fmt.Printf("%s is walking with bipedal locomotion\n", h.Name)
	h.BatteryLevel -= 6
}

func (h *AndroidBot) Exercise() error {
	fmt.Printf("%s is performing calibration movements\n", h.Name)
	h.BatteryLevel -= 7
	return nil
}

// Mechanical interface
func (h *AndroidBot) PowerDown() {
	fmt.Printf("%s humanoid is entering sleep mode\n", h.Name)
	h.IsOperational = false
}

func (h *AndroidBot) PowerUp() {
	fmt.Printf("%s humanoid is activating all systems\n", h.Name)
	h.IsOperational = true
}

func (h *AndroidBot) RunDiagnostics() {
	fmt.Printf("%s is running humanoid system diagnostics\n", h.Name)
	network := "Disconnected"
	if h.NetworkConnected {
		network = "Connected"
	}
	fmt.Printf("Battery: %d%%, Network: %s\n", h.BatteryLevel, network)
}

// Rechargeable interface
func (h *AndroidBot) Recharge() {
	fmt.Printf("%s is recharging via induction pad\n", h.Name)
	h.BatteryLevel = 100
}

func (h *AndroidBot) GetBatteryLevel() int {
	return h.BatteryLevel
}

// Programmable interface
func (h *AndroidBot) LoadProgram(program string) {
	fmt.Printf("%s is loading behavioral program: %s\n", h.Name, program)
	h.CurrentProgram = program
}

func (h *AndroidBot) ExecuteProgram() error {
	if h.CurrentProgram == "" {
		return errors.New("no behavioral program loaded")
	}
	fmt.Printf("%s is executing behavioral program: %s\n", h.Name, h.CurrentProgram)
	h.BatteryLevel -= 3
	return nil
}

// NetworkEnabled interface
func (h *AndroidBot) ConnectToNetwork() error {
	fmt.Printf("%s is connecting to robot network\n", h.Name)
	h.NetworkConnected = true
	return nil
}

func (h *AndroidBot) SendData(data string) error {
	if !h.NetworkConnected {
		return errors.New("not connected to network")
	}
	fmt.Printf("%s is sending data: %s\n", h.Name, data)
	return nil
}

func (h *AndroidBot) ReceiveData() (string, error) {
	if !h.NetworkConnected {
		return "", errors.New("not connected to network")
	}
	fmt.Printf("%s is receiving network data\n", h.Name)
	return "network_data_received", nil
}

// Client classes that only depend on interfaces they need
type TaskManager struct{}

// Only depends on Workable interface - works with any worker type
func (tm *TaskManager) AssignTask(worker Workable, task string) {
	fmt.Printf("Assigning task: %s\n", task)

	// Type-specific task assignment
	switch w := worker.(type) {
	case *Employee:
		w.SetCurrentTask(task)
	case Programmable:
		w.LoadProgram(task)
	}

	worker.Work()
}

// Only needs workers, doesn't care about other capabilities
func (tm *TaskManager) ManageWorkforce(workers ...Workable) {
	fmt.Printf("Managing workforce of %d workers\n", len(workers))
	for _, worker := range workers {
		worker.Work()
	}
}

type TechnicianManager struct{}

// Only depends on Mechanical interface
func (tm *TechnicianManager) PerformMaintenance(device Mechanical) {
	fmt.Println("Starting maintenance routine...")
	device.RunDiagnostics()
	device.PowerDown()
	fmt.Println("Maintenance completed")
	device.PowerUp()
}

// Only depends on Rechargeable interface
func (tm *TechnicianManager) ManageCharge(device Rechargeable) {
	if device.GetBatteryLevel() < 20 {
		fmt.Println("Battery low, initiating recharge")
		device.Recharge()
	} else {
		fmt.Printf("Battery sufficient: %d%%\n", device.GetBatteryLevel())
	}
}

type TrainingManager struct{}

// Only depends on Cognitive interface
func (tm *TrainingManager) ConductTraining(learner Cognitive) {
	fmt.Println("Starting cognitive training session...")
	learner.Think()
	if err := learner.Learn(); err != nil {
		fmt.Printf("Learning error: %s\n", err.Error())
	}
}

type EventCoordinator struct{}

// Only depends on Social interface
func (ec *EventCoordinator) OrganizeSocialEvent(participants ...Social) {
	fmt.Println("Organizing social interaction...")
	for _, participant := range participants {
		participant.Socialize()
		participant.Communicate()
	}
}

type HealthCareProvider struct{}

// Only depends on Biological interface
func (hcp *HealthCareProvider) ProvideCare(organism Biological) error {
	fmt.Println("Providing biological care...")

	if err := organism.Eat(); err != nil {
		return fmt.Errorf("feeding failed: %w", err)
	}

	if err := organism.Sleep(); err != nil {
		return fmt.Errorf("rest failed: %w", err)
	}

	if err := organism.Breathe(); err != nil {
		return fmt.Errorf("breathing failed: %w", err)
	}

	return nil
}

// Demonstrates proper Interface Segregation
func mainCorrect() {
	fmt.Println("=== Interface Segregation Principle - CORRECT (Go) ===")
	fmt.Println()

	// Create different types of workers
	alice := NewEmployee("Alice")
	factoryBot := NewFactoryBot("FactoryBot-3000")
	android := NewAndroidBot("Android-Sara")

	tm := &TaskManager{}
	techMgr := &TechnicianManager{}
	trainMgr := &TrainingManager{}
	eventCoord := &EventCoordinator{}
	healthProv := &HealthCareProvider{}

	fmt.Println("1. Work Management (only needs Workable interface):")
	tm.AssignTask(alice, "Design new product")
	tm.AssignTask(factoryBot, "Assembly line task")
	tm.AssignTask(android, "Customer service")

	fmt.Println()
	fmt.Println("2. Managing workforce (polymorphic with Workable):")
	tm.ManageWorkforce(alice, factoryBot, android)

	fmt.Println()
	fmt.Println("3. Maintenance (only for Mechanical devices):")
	// Note: Cannot pass alice here - she doesn't implement Mechanical!
	techMgr.PerformMaintenance(factoryBot)
	techMgr.PerformMaintenance(android)

	fmt.Println()
	fmt.Println("4. Battery Management (only for Rechargeable devices):")
	// Note: Cannot pass alice here - she doesn't implement Rechargeable!
	techMgr.ManageCharge(factoryBot)
	techMgr.ManageCharge(android)

	fmt.Println()
	fmt.Println("5. Cognitive Training (for thinking entities):")
	trainMgr.ConductTraining(alice)
	trainMgr.ConductTraining(android) // Humanoid can think too!
	// Note: Cannot pass factoryBot - it doesn't implement Cognitive!

	fmt.Println()
	fmt.Println("6. Social Coordination (for social entities):")
	eventCoord.OrganizeSocialEvent(alice, android)
	// Note: Cannot pass factoryBot - it doesn't implement Social!

	fmt.Println()
	fmt.Println("7. Biological Care (only for living organisms):")
	if err := healthProv.ProvideCare(alice); err != nil {
		fmt.Printf("Care failed: %s\n", err.Error())
	}
	// Note: Cannot pass robots - they don't implement Biological!

	fmt.Println()
	fmt.Println("8. Network Operations (only for NetworkEnabled):")
	// Use type assertion to check if android implements NetworkEnabled
	if netDevice, ok := interface{}(android).(NetworkEnabled); ok {
		if err := netDevice.ConnectToNetwork(); err == nil {
			netDevice.SendData("Status update")
			_, _ = netDevice.ReceiveData()
		}
	}

	fmt.Println()
	fmt.Println("=== Benefits of Interface Segregation ===")
	fmt.Println("✓ Clients only depend on methods they actually use")
	fmt.Println("✓ No forced implementation of irrelevant methods")
	fmt.Println("✓ High cohesion within each interface")
	fmt.Println("✓ Easy to extend with new capabilities")
	fmt.Println("✓ Better testability and maintainability")
	fmt.Println("✓ Follows Single Responsibility at interface level")
	fmt.Println("✓ Go's implicit interface satisfaction enables clean design")

	fmt.Println()
	fmt.Println("=== Interface Composition Examples ===")
	fmt.Println("Employee implements: Workable + Biological + Cognitive + Social + Physical + Reproductive")
	fmt.Println("FactoryBot implements: Workable + Mechanical + Rechargeable + Programmable")
	fmt.Println("AndroidBot implements: Workable + Cognitive + Social + Physical + Mechanical + Rechargeable + Programmable + NetworkEnabled")
}

func mainIncorrect() {
	// Run the correct demonstration
	mainCorrect()
}
