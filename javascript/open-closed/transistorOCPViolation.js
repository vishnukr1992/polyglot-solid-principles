// VIOLATION OF OPEN-CLOSED PRINCIPLE
// This code violates OCP because:
// 1. Adding new transistor types requires modifying existing code
// 2. The AmplifierCircuit is tightly coupled to specific transistor implementations
// 3. No abstraction layer exists for different transistor types

// Hard-coded transistor types - violates OCP
const TransistorTypes = {
    BJT: "BJT",
    FET: "FET",
    MOSFET: "MOSFET"
};

// Monolithic transistor class that handles all types - violates SRP too
class TransistorViolation {
    constructor(transistorType) {
        this.transistorType = transistorType;
        this.baseSignal = 0;
        this.collectorInput = 0;
        this.gateVoltage = 0;  // for FET types
        this.drainCurrent = 0; // for FET types
    }
    
    // Violates OCP: Adding new transistor behavior requires modifying this method
    processSignal(signal, input) {
        this.baseSignal = signal;
        this.collectorInput = input;
        
        // Hard-coded logic for different types - violates OCP
        switch (this.transistorType) {
            case TransistorTypes.BJT:
                // BJT specific processing
                return this.collectorInput * (this.baseSignal * 0.1);
            case TransistorTypes.FET:
                // FET specific processing
                this.gateVoltage = signal;
                this.drainCurrent = input;
                return this.drainCurrent * (this.gateVoltage * 0.15);
            case TransistorTypes.MOSFET:
                // MOSFET specific processing
                this.gateVoltage = signal;
                this.drainCurrent = input;
                const threshold = 0.7;
                if (this.gateVoltage > threshold) {
                    return this.drainCurrent * (this.gateVoltage - threshold) * 0.2;
                }
                return 0;
            default:
                return 0;
        }
    }
}

// Amplifier circuit that violates OCP
class AmplifierCircuitViolation {
    constructor() {
        this.transistors = [];
    }
    
    addTransistor(transistorType) {
        const transistor = new TransistorViolation(transistorType);
        this.transistors.push(transistor);
    }
    
    // Violates OCP: Adding new transistor types requires modifying this method
    amplifySignal(signal, input) {
        let totalOutput = 0.0;
        
        for (const transistor of this.transistors) {
            
            // This switch statement violates OCP - must be modified for each new type
            switch (transistor.transistorType) {
                case TransistorTypes.BJT:
                    const bjtOutput = transistor.processSignal(signal, input);
                    totalOutput += bjtOutput;
                    break;
                case TransistorTypes.FET:
                    const fetOutput = transistor.processSignal(signal, input) * 1.1; // FET has higher gain
                    totalOutput += fetOutput;
                    break;
                case TransistorTypes.MOSFET:
                    const mosfetOutput = transistor.processSignal(signal, input) * 1.2; // MOSFET has highest gain
                    totalOutput += mosfetOutput;
                    break;
            }
        }
        
        return totalOutput;
    }
    
    // Another method that would need modification for new types - violates OCP
    getTransistorInfo() {
        const info = [];
        
        for (const transistor of this.transistors) {
            let description;
            
            // Yet another switch statement that violates OCP
            switch (transistor.transistorType) {
                case TransistorTypes.BJT:
                    description = "Bipolar Junction Transistor - Current controlled";
                    break;
                case TransistorTypes.FET:
                    description = "Field Effect Transistor - Voltage controlled";
                    break;
                case TransistorTypes.MOSFET:
                    description = "Metal Oxide Semiconductor FET - Enhanced mode";
                    break;
                default:
                    description = "Unknown transistor type";
            }
            
            info.push({
                type: transistor.transistorType,
                description: description
            });
        }
        
        return info;
    }
}

function demonstrateOCPViolation() {
    console.log("=== OCP VIOLATION EXAMPLE ===");
    console.log("This code violates the Open-Closed Principle");
    console.log("Adding new transistor types requires modifying existing code");
    console.log();
    
    // Create amplifier circuit
    const amplifier = new AmplifierCircuitViolation();
    
    // Add different transistor types
    amplifier.addTransistor(TransistorTypes.BJT);
    amplifier.addTransistor(TransistorTypes.FET);
    amplifier.addTransistor(TransistorTypes.MOSFET);
    
    // Test the amplifier
    const signal = 2.0;
    const input = 5.0;
    
    const output = amplifier.amplifySignal(signal, input);
    console.log(`Input Signal: ${signal.toFixed(2)}`);
    console.log(`Input Current: ${input.toFixed(2)}`);
    console.log(`Amplified Output: ${output.toFixed(2)}`);
    
    console.log();
    console.log("Transistor Information:");
    const info = amplifier.getTransistorInfo();
    info.forEach((item, index) => {
        console.log(`${index + 1}. ${item.type}: ${item.description}`);
    });
    
    console.log();
    console.log("Problems with this approach:");
    console.log("1. Adding new transistor types requires modifying amplifySignal method");
    console.log("2. processSignal method must be extended for each new type");
    console.log("3. getTransistorInfo method needs updates for each new type");
    console.log("4. No abstraction - tightly coupled to specific implementations");
    console.log("5. Violates Single Responsibility Principle as well");
    console.log("6. Multiple switch statements scattered throughout the code");
}

// Export for use in other modules
if (typeof module !== 'undefined' && module.exports) {
    module.exports = {
        TransistorTypes,
        TransistorViolation,
        AmplifierCircuitViolation,
        demonstrateOCPViolation
    };
}

// Run demonstration if this is the main module
if (typeof require !== 'undefined' && require.main === module) {
    demonstrateOCPViolation();
}