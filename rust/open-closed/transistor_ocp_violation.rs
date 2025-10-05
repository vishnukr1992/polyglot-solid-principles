// VIOLATION OF OPEN-CLOSED PRINCIPLE
// This code violates OCP because:
// 1. Adding new transistor types requires modifying existing code
// 2. The AmplifierCircuit is tightly coupled to specific transistor implementations
// 3. No abstraction layer exists for different transistor types

use std::fmt;

/// Hard-coded transistor types - violates OCP
#[derive(Debug, Clone, PartialEq)]
pub enum TransistorType {
    BJT,
    FET,
    MOSFET,
}

impl fmt::Display for TransistorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TransistorType::BJT => write!(f, "BJT"),
            TransistorType::FET => write!(f, "FET"),
            TransistorType::MOSFET => write!(f, "MOSFET"),
        }
    }
}

/// Monolithic transistor struct that handles all types - violates SRP too
#[derive(Debug)]
pub struct TransistorViolation {
    transistor_type: TransistorType,
    base_signal: f64,
    collector_input: f64,
    gate_voltage: f64,  // for FET types
    drain_current: f64, // for FET types
}

impl TransistorViolation {
    pub fn new(transistor_type: TransistorType) -> Self {
        TransistorViolation {
            transistor_type,
            base_signal: 0.0,
            collector_input: 0.0,
            gate_voltage: 0.0,
            drain_current: 0.0,
        }
    }
    
    /// Violates OCP: Adding new transistor behavior requires modifying this method
    pub fn process_signal(&mut self, signal: f64, input: f64) -> f64 {
        self.base_signal = signal;
        self.collector_input = input;
        
        // Hard-coded logic for different types - violates OCP
        match self.transistor_type {
            TransistorType::BJT => {
                // BJT specific processing
                self.collector_input * (self.base_signal * 0.1)
            }
            TransistorType::FET => {
                // FET specific processing
                self.gate_voltage = signal;
                self.drain_current = input;
                self.drain_current * (self.gate_voltage * 0.15)
            }
            TransistorType::MOSFET => {
                // MOSFET specific processing
                self.gate_voltage = signal;
                self.drain_current = input;
                let threshold = 0.7;
                if self.gate_voltage > threshold {
                    self.drain_current * (self.gate_voltage - threshold) * 0.2
                } else {
                    0.0
                }
            }
        }
    }
    
    pub fn get_type(&self) -> &TransistorType {
        &self.transistor_type
    }
}

/// Amplifier circuit that violates OCP
pub struct AmplifierCircuitViolation {
    transistors: Vec<TransistorViolation>,
}

impl AmplifierCircuitViolation {
    pub fn new() -> Self {
        AmplifierCircuitViolation {
            transistors: Vec::new(),
        }
    }
    
    pub fn add_transistor(&mut self, transistor_type: TransistorType) {
        let transistor = TransistorViolation::new(transistor_type);
        self.transistors.push(transistor);
    }
    
    /// Violates OCP: Adding new transistor types requires modifying this method
    pub fn amplify_signal(&mut self, signal: f64, input: f64) -> f64 {
        let mut total_output = 0.0;
        
        for transistor in &mut self.transistors {
            // This match statement violates OCP - must be modified for each new type
            let output = match transistor.transistor_type {
                TransistorType::BJT => {
                    transistor.process_signal(signal, input)
                }
                TransistorType::FET => {
                    transistor.process_signal(signal, input) * 1.1 // FET has higher gain
                }
                TransistorType::MOSFET => {
                    transistor.process_signal(signal, input) * 1.2 // MOSFET has highest gain
                }
            };
            total_output += output;
        }
        
        total_output
    }
    
    /// Another method that would need modification for new types - violates OCP
    pub fn get_transistor_info(&self) -> Vec<(String, String)> {
        let mut info = Vec::new();
        
        for transistor in &self.transistors {
            // Yet another match statement that violates OCP
            let description = match transistor.transistor_type {
                TransistorType::BJT => "Bipolar Junction Transistor - Current controlled",
                TransistorType::FET => "Field Effect Transistor - Voltage controlled",
                TransistorType::MOSFET => "Metal Oxide Semiconductor FET - Enhanced mode",
            };
            
            info.push((transistor.transistor_type.to_string(), description.to_string()));
        }
        
        info
    }
    
    /// Yet another method requiring modification for new types - violates OCP
    pub fn calculate_power_consumption(&self) -> f64 {
        let mut total_power = 0.0;
        
        for transistor in &self.transistors {
            // Another match statement that violates OCP
            let power = match transistor.transistor_type {
                TransistorType::BJT => {
                    // BJT power calculation
                    transistor.base_signal * transistor.collector_input * 0.05
                }
                TransistorType::FET => {
                    // FET power calculation
                    transistor.gate_voltage * transistor.drain_current * 0.03
                }
                TransistorType::MOSFET => {
                    // MOSFET power calculation
                    if transistor.gate_voltage > 0.7 {
                        transistor.gate_voltage * transistor.drain_current * 0.02
                    } else {
                        0.0
                    }
                }
            };
            total_power += power;
        }
        
        total_power
    }
}

/// Example usage demonstrating OCP violation
pub fn demonstrate_ocp_violation() {
    println!("=== OCP VIOLATION EXAMPLE ===");
    println!("This code violates the Open-Closed Principle");
    println!("Adding new transistor types requires modifying existing code");
    println!();
    
    // Create amplifier circuit
    let mut amplifier = AmplifierCircuitViolation::new();
    
    // Add different transistor types
    amplifier.add_transistor(TransistorType::BJT);
    amplifier.add_transistor(TransistorType::FET);
    amplifier.add_transistor(TransistorType::MOSFET);
    
    // Test the amplifier
    let signal = 2.0;
    let input = 5.0;
    
    let output = amplifier.amplify_signal(signal, input);
    println!("Input Signal: {:.2}", signal);
    println!("Input Current: {:.2}", input);
    println!("Amplified Output: {:.2}", output);
    
    // Test additional functionality
    let power = amplifier.calculate_power_consumption();
    println!("Power Consumption: {:.2}W", power);
    
    println!();
    println!("Transistor Information:");
    let info = amplifier.get_transistor_info();
    for (i, (transistor_type, description)) in info.iter().enumerate() {
        println!("{}. {}: {}", i + 1, transistor_type, description);
    }
    
    println!();
    println!("Problems with this approach:");
    println!("1. Adding new transistor types requires modifying amplify_signal method");
    println!("2. process_signal method must be extended for each new type");
    println!("3. get_transistor_info method needs updates for each new type");
    println!("4. calculate_power_consumption method needs updates for each new type");
    println!("5. No abstraction - tightly coupled to specific implementations");
    println!("6. Violates Single Responsibility Principle as well");
    println!("7. Multiple match statements scattered throughout the code");
}

fn main() {
    demonstrate_ocp_violation();
}