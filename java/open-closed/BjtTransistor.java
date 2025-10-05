// CORRECT IMPLEMENTATION - FOLLOWS OPEN-CLOSED PRINCIPLE
// This class is closed for modification but open for extension

public class BJTTransistor implements Transistor {
    private double baseSignal;
    private double collectorInput;
    
    @Override
    public void base(double signal) {
        this.baseSignal = signal;
    }
    
    @Override
    public void collector(double input) {
        this.collectorInput = input;
    }
    
    @Override
    public double output() {
        // Simulated gain: collector output depends on base signal
        return this.collectorInput * (this.baseSignal * 0.1); // crude amplifier model
    }
}

// Example of extending without modifying existing code
class FETTransistor implements Transistor {
    private double gateVoltage;
    private double drainCurrent;
    
    @Override
    public void base(double signal) {
        this.gateVoltage = signal;
    }
    
    @Override
    public void collector(double input) {
        this.drainCurrent = input;
    }
    
    @Override
    public double output() {
        // FET specific behavior - different from BJT
        return this.drainCurrent * (this.gateVoltage * 0.15);
    }
}

// Another extension without modifying existing code
class MOSFETTransistor implements Transistor {
    private double gateVoltage;
    private double drainCurrent;
    private static final double THRESHOLD = 0.7;
    
    @Override
    public void base(double signal) {
        this.gateVoltage = signal;
    }
    
    @Override
    public void collector(double input) {
        this.drainCurrent = input;
    }
    
    @Override
    public double output() {
        // MOSFET specific behavior with threshold
        if (this.gateVoltage > THRESHOLD) {
            return this.drainCurrent * (this.gateVoltage - THRESHOLD) * 0.2;
        }
        return 0;
    }
}

// Amplifier circuit that follows OCP
class AmplifierCircuit {
    private java.util.List<Transistor> transistors;
    
    public AmplifierCircuit() {
        this.transistors = new java.util.ArrayList<>();
    }
    
    // This method doesn't need to change when new transistor types are added
    public void addTransistor(Transistor transistor) {
        this.transistors.add(transistor);
    }
    
    // This method works with any Transistor implementation
    public double amplifySignal(double signal, double input) {
        double totalOutput = 0.0;
        
        for (Transistor transistor : this.transistors) {
            transistor.base(signal);
            transistor.collector(input);
            totalOutput += transistor.output();
        }
        
        return totalOutput;
    }
}