package main

import "fmt"

// VIOLATION OF OPEN-CLOSED PRINCIPLE
// This code violates OCP because:
// 1. Adding new transistor types requires modifying existing code
// 2. The AmplifierCircuit is tightly coupled to specific transistor implementations
// 3. No abstraction layer exists for different transistor types

// Hard-coded transistor types - violates OCP
const (
	BJT_TYPE    = "BJT"
	FET_TYPE    = "FET"
	MOSFET_TYPE = "MOSFET"
)

// Monolithic transistor struct that handles all types - violates SRP too
type TransistorViolation struct {
	transistorType string
	baseSignal     float64
	collectorInput float64
	gateVoltage    float64 // for FET types
	drainCurrent   float64 // for FET types
}

// Violates OCP: Adding new transistor behavior requires modifying this method
func (t *TransistorViolation) ProcessSignal(signal float64, input float64) float64 {
	t.baseSignal = signal
	t.collectorInput = input

	// Hard-coded logic for different types - violates OCP
	switch t.transistorType {
	case BJT_TYPE:
		// BJT specific processing
		return t.collectorInput * (t.baseSignal * 0.1)
	case FET_TYPE:
		// FET specific processing
		t.gateVoltage = signal
		t.drainCurrent = input
		return t.drainCurrent * (t.gateVoltage * 0.15)
	case MOSFET_TYPE:
		// MOSFET specific processing
		t.gateVoltage = signal
		t.drainCurrent = input
		threshold := 0.7
		if t.gateVoltage > threshold {
			return t.drainCurrent * (t.gateVoltage - threshold) * 0.2
		}
		return 0
	default:
		return 0
	}
}

// Amplifier circuit that violates OCP
type AmplifierCircuitViolation struct {
	transistors []TransistorViolation
}

func (a *AmplifierCircuitViolation) AddTransistor(transistorType string) {
	transistor := TransistorViolation{transistorType: transistorType}
	a.transistors = append(a.transistors, transistor)
}

// Violates OCP: Adding new transistor types requires modifying this method
func (a *AmplifierCircuitViolation) AmplifySignal(signal float64, input float64) float64 {
	totalOutput := 0.0

	for i := range a.transistors {
		transistor := &a.transistors[i]

		// This switch statement violates OCP - must be modified for each new type
		switch transistor.transistorType {
		case BJT_TYPE:
			output := transistor.ProcessSignal(signal, input)
			totalOutput += output
		case FET_TYPE:
			output := transistor.ProcessSignal(signal, input) * 1.1 // FET has higher gain
			totalOutput += output
		case MOSFET_TYPE:
			output := transistor.ProcessSignal(signal, input) * 1.2 // MOSFET has highest gain
			totalOutput += output
		}
	}

	return totalOutput
}

func main() {
	fmt.Println("=== OCP VIOLATION EXAMPLE ===")
	fmt.Println("This code violates the Open-Closed Principle")
	fmt.Println("Adding new transistor types requires modifying existing code")
	fmt.Println()

	// Create amplifier circuit
	amplifier := &AmplifierCircuitViolation{}

	// Add different transistor types
	amplifier.AddTransistor(BJT_TYPE)
	amplifier.AddTransistor(FET_TYPE)
	amplifier.AddTransistor(MOSFET_TYPE)

	// Test the amplifier
	signal := 2.0
	input := 5.0

	output := amplifier.AmplifySignal(signal, input)
	fmt.Printf("Input Signal: %.2f\n", signal)
	fmt.Printf("Input Current: %.2f\n", input)
	fmt.Printf("Amplified Output: %.2f\n", output)

	fmt.Println()
	fmt.Println("Problems with this approach:")
	fmt.Println("1. Adding new transistor types requires modifying AmplifySignal method")
	fmt.Println("2. ProcessSignal method must be extended for each new type")
	fmt.Println("3. No abstraction - tightly coupled to specific implementations")
	fmt.Println("4. Violates Single Responsibility Principle as well")
}
