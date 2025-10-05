// CORRECT IMPLEMENTATION - FOLLOWS OPEN-CLOSED PRINCIPLE
// This approach uses traits to allow extension without modification

use std::fmt::Debug;

/// Trait defining the interface for all transistor types
pub trait Transistor: Debug {
    /// Control input
    fn base(&mut self, signal: f64);
    /// Main input
    fn collector(&mut self, input: f64);
    /// Measured output (Collector - Emitter)
    fn output(&self) -> f64;
}

/// BJT Transistor implementation
#[derive(Debug)]
pub struct BJTTransistor {
    base_signal: f64,
    collector_input: f64,
}

impl BJTTransistor {
    pub fn new() -> Self {
        BJTTransistor {
            base_signal: 0.0,
            collector_input: 0.0,
        }
    }
}

impl Transistor for BJTTransistor {
    fn base(&mut self, signal: f64) {
        self.base_signal = signal;
    }
    
    fn collector(&mut self, input: f64) {
        self.collector_input = input;
    }
    
    fn output(&self) -> f64 {
        // Simulated gain: collector output depends on base signal
        self.collector_input * (self.base_signal * 0.1) // crude amplifier model
    }
}

/// FET Transistor implementation - extends without modifying existing code
#[derive(Debug)]
pub struct FETTransistor {
    gate_voltage: f64,
    drain_current: f64,
}

impl FETTransistor {
    pub fn new() -> Self {
        FETTransistor {
            gate_voltage: 0.0,
            drain_current: 0.0,
        }
    }
}

impl Transistor for FETTransistor {
    fn base(&mut self, signal: f64) {
        self.gate_voltage = signal;
    }
    
    fn collector(&mut self, input: f64) {
        self.drain_current = input;
    }
    
    fn output(&self) -> f64 {
        // FET specific behavior - different from BJT
        self.drain_current * (self.gate_voltage * 0.15)
    }
}

/// MOSFET Transistor implementation - another extension without modifying existing code
#[derive(Debug)]
pub struct MOSFETTransistor {
    gate_voltage: f64,
    drain_current: f64,
    threshold: f64,
}

impl MOSFETTransistor {
    pub fn new() -> Self {
        MOSFETTransistor {
            gate_voltage: 0.0,
            drain_current: 0.0,
            threshold: 0.7,
        }
    }
}

impl Transistor for MOSFETTransistor {
    fn base(&mut self, signal: f64) {
        self.gate_voltage = signal;
    }
    
    fn collector(&mut self, input: f64) {
        self.drain_current = input;
    }
    
    fn output(&self) -> f64 {
        // MOSFET specific behavior with threshold
        if self.gate_voltage > self.threshold {
            self.drain_current * (self.gate_voltage - self.threshold) * 0.2
        } else {
            0.0
        }
    }
}

/// Amplifier circuit that follows OCP
pub struct AmplifierCircuit {
    transistors: Vec<Box<dyn Transistor>>,
}

impl AmplifierCircuit {
    pub fn new() -> Self {
        AmplifierCircuit {
            transistors: Vec::new(),
        }
    }
    
    /// This method doesn't need to change when new transistor types are added
    pub fn add_transistor(&mut self, transistor: Box<dyn Transistor>) {
        self.transistors.push(transistor);
    }
    
    /// This method works with any Transistor implementation
    pub fn amplify_signal(&mut self, signal: f64, input: f64) -> f64 {
        let mut total_output = 0.0;
        
        for transistor in &mut self.transistors {
            transistor.base(signal);
            transistor.collector(input);
            total_output += transistor.output();
        }
        
        total_output
    }
}

/// Example usage demonstrating OCP compliance
pub fn demonstrate_correct_ocp() {
    println!("=== CORRECT OCP IMPLEMENTATION ===");
    println!("This code follows the Open-Closed Principle");
    println!("New transistor types can be added without modifying existing code");
    println!();
    
    // Create amplifier circuit
    let mut amplifier = AmplifierCircuit::new();
    
    // Add different transistor types
    amplifier.add_transistor(Box::new(BJTTransistor::new()));
    amplifier.add_transistor(Box::new(FETTransistor::new()));
    amplifier.add_transistor(Box::new(MOSFETTransistor::new()));
    
    // Test the amplifier
    let signal = 2.0;
    let input = 5.0;
    
    let output = amplifier.amplify_signal(signal, input);
    println!("Input Signal: {:.2}", signal);
    println!("Input Current: {:.2}", input);
    println!("Amplified Output: {:.2}", output);
    
    println!();
    println!("Benefits of this approach:");
    println!("1. New transistor types can be added without modifying existing code");
    println!("2. Each transistor type is responsible for its own behavior");
    println!("3. Traits enable flexible design");
    println!("4. Follows Single Responsibility Principle");
}

fn main() {
    demonstrate_correct_ocp();
}