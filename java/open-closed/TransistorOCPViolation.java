import java.util.ArrayList;
import java.util.List;

// VIOLATION OF OPEN-CLOSED PRINCIPLE
// This code violates OCP because:
// 1. Adding new transistor types requires modifying existing code
// 2. The AmplifierCircuit is tightly coupled to specific transistor implementations
// 3. No abstraction layer exists for different transistor types

public class TransistorOCPViolation {
    
    // Hard-coded transistor types - violates OCP
    public static final String BJT_TYPE = "BJT";
    public static final String FET_TYPE = "FET";
    public static final String MOSFET_TYPE = "MOSFET";
    
    // Monolithic transistor class that handles all types - violates SRP too
    static class TransistorViolation {
        private String transistorType;
        private double baseSignal;
        private double collectorInput;
        private double gateVoltage;  // for FET types
        private double drainCurrent; // for FET types
        
        public TransistorViolation(String transistorType) {
            this.transistorType = transistorType;
        }
        
        // Violates OCP: Adding new transistor behavior requires modifying this method
        public double processSignal(double signal, double input) {
            this.baseSignal = signal;
            this.collectorInput = input;
            
            // Hard-coded logic for different types - violates OCP
            switch (this.transistorType) {
                case BJT_TYPE:
                    // BJT specific processing
                    return this.collectorInput * (this.baseSignal * 0.1);
                case FET_TYPE:
                    // FET specific processing
                    this.gateVoltage = signal;
                    this.drainCurrent = input;
                    return this.drainCurrent * (this.gateVoltage * 0.15);
                case MOSFET_TYPE:
                    // MOSFET specific processing
                    this.gateVoltage = signal;
                    this.drainCurrent = input;
                    double threshold = 0.7;
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
    static class AmplifierCircuitViolation {
        private List<TransistorViolation> transistors;
        
        public AmplifierCircuitViolation() {
            this.transistors = new ArrayList<>();
        }
        
        public void addTransistor(String transistorType) {
            TransistorViolation transistor = new TransistorViolation(transistorType);
            this.transistors.add(transistor);
        }
        
        // Violates OCP: Adding new transistor types requires modifying this method
        public double amplifySignal(double signal, double input) {
            double totalOutput = 0.0;
            
            for (TransistorViolation transistor : this.transistors) {
                
                // This switch statement violates OCP - must be modified for each new type
                switch (transistor.transistorType) {
                    case BJT_TYPE:
                        double output = transistor.processSignal(signal, input);
                        totalOutput += output;
                        break;
                    case FET_TYPE:
                        output = transistor.processSignal(signal, input) * 1.1; // FET has higher gain
                        totalOutput += output;
                        break;
                    case MOSFET_TYPE:
                        output = transistor.processSignal(signal, input) * 1.2; // MOSFET has highest gain
                        totalOutput += output;
                        break;
                }
            }
            
            return totalOutput;
        }
    }
    
    public static void main(String[] args) {
        System.out.println("=== OCP VIOLATION EXAMPLE ===");
        System.out.println("This code violates the Open-Closed Principle");
        System.out.println("Adding new transistor types requires modifying existing code");
        System.out.println();
        
        // Create amplifier circuit
        AmplifierCircuitViolation amplifier = new AmplifierCircuitViolation();
        
        // Add different transistor types
        amplifier.addTransistor(BJT_TYPE);
        amplifier.addTransistor(FET_TYPE);
        amplifier.addTransistor(MOSFET_TYPE);
        
        // Test the amplifier
        double signal = 2.0;
        double input = 5.0;
        
        double output = amplifier.amplifySignal(signal, input);
        System.out.printf("Input Signal: %.2f%n", signal);
        System.out.printf("Input Current: %.2f%n", input);
        System.out.printf("Amplified Output: %.2f%n", output);
        
        System.out.println();
        System.out.println("Problems with this approach:");
        System.out.println("1. Adding new transistor types requires modifying amplifySignal method");
        System.out.println("2. processSignal method must be extended for each new type");
        System.out.println("3. No abstraction - tightly coupled to specific implementations");
        System.out.println("4. Violates Single Responsibility Principle as well");
    }
}