// CORRECT IMPLEMENTATION - FOLLOWS OPEN-CLOSED PRINCIPLE
// This approach uses polymorphism to allow extension without modification

// Base Transistor class (interface equivalent in JavaScript)
class Transistor {
    constructor() {
        if (this.constructor === Transistor) {
            throw new Error("Cannot instantiate abstract class Transistor");
        }
    }
    
    base(signal) {
        throw new Error("Method 'base' must be implemented");
    }
    
    collector(input) {
        throw new Error("Method 'collector' must be implemented");
    }
    
    output() {
        throw new Error("Method 'output' must be implemented");
    }
}

// BJT Transistor implementation
class BJTTransistor extends Transistor {
    constructor() {
        super();
        this.baseSignal = 0;
        this.collectorInput = 0;
    }
    
    base(signal) {
        this.baseSignal = signal;
    }
    
    collector(input) {
        this.collectorInput = input;
    }
    
    output() {
        // Simulated gain: collector output depends on base signal
        return this.collectorInput * (this.baseSignal * 0.1); // crude amplifier model
    }
}

// FET Transistor implementation - extends without modifying existing code
class FETTransistor extends Transistor {
    constructor() {
        super();
        this.gateVoltage = 0;
        this.drainCurrent = 0;
    }
    
    base(signal) {
        this.gateVoltage = signal;
    }
    
    collector(input) {
        this.drainCurrent = input;
    }
    
    output() {
        // FET specific behavior - different from BJT
        return this.drainCurrent * (this.gateVoltage * 0.15);
    }
}

// MOSFET Transistor implementation - another extension
class MOSFETTransistor extends Transistor {
    constructor() {
        super();
        this.gateVoltage = 0;
        this.drainCurrent = 0;
        this.threshold = 0.7;
    }
    
    base(signal) {
        this.gateVoltage = signal;
    }
    
    collector(input) {
        this.drainCurrent = input;
    }
    
    output() {
        // MOSFET specific behavior with threshold
        if (this.gateVoltage > this.threshold) {
            return this.drainCurrent * (this.gateVoltage - this.threshold) * 0.2;
        }
        return 0;
    }
}

// Amplifier circuit that follows OCP
class AmplifierCircuit {
    constructor() {
        this.transistors = [];
    }
    
    // This method doesn't need to change when new transistor types are added
    addTransistor(transistor) {
        if (!(transistor instanceof Transistor)) {
            throw new Error("Must be a Transistor instance");
        }
        this.transistors.push(transistor);
    }
    
    // This method works with any Transistor implementation
    amplifySignal(signal, input) {
        let totalOutput = 0.0;
        
        for (const transistor of this.transistors) {
            transistor.base(signal);
            transistor.collector(input);
            totalOutput += transistor.output();
        }
        
        return totalOutput;
    }
}

// Example usage demonstrating OCP compliance
function demonstrateCorrectOCP() {
    console.log("=== CORRECT OCP IMPLEMENTATION ===");
    console.log("This code follows the Open-Closed Principle");
    console.log("New transistor types can be added without modifying existing code");
    console.log();
    
    // Create amplifier circuit
    const amplifier = new AmplifierCircuit();
    
    // Add different transistor types
    amplifier.addTransistor(new BJTTransistor());
    amplifier.addTransistor(new FETTransistor());
    amplifier.addTransistor(new MOSFETTransistor());
    
    // Test the amplifier
    const signal = 2.0;
    const input = 5.0;
    
    const output = amplifier.amplifySignal(signal, input);
    console.log(`Input Signal: ${signal.toFixed(2)}`);
    console.log(`Input Current: ${input.toFixed(2)}`);
    console.log(`Amplified Output: ${output.toFixed(2)}`);
    
    console.log();
    console.log("Benefits of this approach:");
    console.log("1. New transistor types can be added without modifying existing code");
    console.log("2. Each transistor type is responsible for its own behavior");
    console.log("3. Polymorphism enables flexible design");
    console.log("4. Follows Single Responsibility Principle");
}

// Export for use in other modules
if (typeof module !== 'undefined' && module.exports) {
    module.exports = {
        Transistor,
        BJTTransistor,
        FETTransistor,
        MOSFETTransistor,
        AmplifierCircuit,
        demonstrateCorrectOCP
    };
}

// Run demonstration if this is the main module
if (typeof require !== 'undefined' && require.main === module) {
    demonstrateCorrectOCP();
}