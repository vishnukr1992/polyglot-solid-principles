/**
 * Interface Segregation Principle (ISP) - VIOLATION Example
 * 
 * This example demonstrates how forcing a Robot to implement a Human interface
 * violates the Interface Segregation Principle.
 * 
 * Problem: Robot is forced to implement biological methods that make no sense
 * for a robot, leading to empty implementations or exceptions.
 */

// VIOLATION: Monolithic interface forces all implementations to have methods they don't need
interface Human {
    void work();
    void eat();
    void sleep();
    void breathe();
    void think();
    void reproduce();
    void exercise();
    void socialize();
}

/**
 * Human implementation - naturally implements all interface methods
 */
class Person implements Human {
    private String name;
    private int energy;
    private boolean isAwake;
    
    public Person(String name) {
        this.name = name;
        this.energy = 100;
        this.isAwake = true;
    }
    
    @Override
    public void work() {
        if (!isAwake) {
            System.out.println(name + " cannot work while sleeping!");
            return;
        }
        System.out.println(name + " is working productively");
        energy -= 20;
    }
    
    @Override
    public void eat() {
        System.out.println(name + " is eating food");
        energy += 30;
    }
    
    @Override
    public void sleep() {
        System.out.println(name + " is sleeping");
        isAwake = false;
        energy = 100;
    }
    
    @Override
    public void breathe() {
        System.out.println(name + " is breathing oxygen");
        // Biological function - essential for humans
    }
    
    @Override
    public void think() {
        System.out.println(name + " is thinking creatively");
        energy -= 5;
    }
    
    @Override
    public void reproduce() {
        System.out.println(name + " can participate in reproduction");
    }
    
    @Override
    public void exercise() {
        System.out.println(name + " is exercising");
        energy -= 15;
    }
    
    @Override
    public void socialize() {
        System.out.println(name + " is socializing with others");
        energy -= 10;
    }
}

/**
 * VIOLATION: Robot forced to implement Human interface
 * This violates ISP because Robot doesn't need most of these methods
 */
class IndustrialRobot implements Human {
    private String model;
    private int batteryLevel;
    private boolean isOperational;
    
    public IndustrialRobot(String model) {
        this.model = model;
        this.batteryLevel = 100;
        this.isOperational = true;
    }
    
    @Override
    public void work() {
        if (!isOperational) {
            System.out.println(model + " is not operational!");
            return;
        }
        System.out.println(model + " robot is performing industrial work");
        batteryLevel -= 10;
    }
    
    // VIOLATION: Forced to implement irrelevant methods
    @Override
    public void eat() {
        // Makes no sense for a robot!
        throw new UnsupportedOperationException("Robots don't eat food!");
    }
    
    @Override
    public void sleep() {
        // Robots don't sleep, they power down
        throw new UnsupportedOperationException("Robots don't sleep! Use powerDown() instead");
    }
    
    @Override
    public void breathe() {
        // Robots don't breathe
        throw new UnsupportedOperationException("Robots don't breathe air!");
    }
    
    @Override
    public void think() {
        // Different kind of thinking (computational)
        System.out.println(model + " is processing data");
        batteryLevel -= 2;
    }
    
    @Override
    public void reproduce() {
        // Robots are manufactured, not reproduced
        throw new UnsupportedOperationException("Robots are manufactured, not reproduced!");
    }
    
    @Override
    public void exercise() {
        // Robots don't need exercise
        throw new UnsupportedOperationException("Robots don't need exercise!");
    }
    
    @Override
    public void socialize() {
        // Basic communication only
        System.out.println(model + " is communicating via network protocols");
    }
    
    // Robot-specific methods that don't fit in Human interface
    public void powerDown() {
        System.out.println(model + " is powering down");
        isOperational = false;
    }
    
    public void recharge() {
        System.out.println(model + " is recharging battery");
        batteryLevel = 100;
    }
    
    public void runDiagnostics() {
        System.out.println(model + " is running system diagnostics");
    }
}

/**
 * Client code that expects Human interface
 */
class WorkManager {
    // This method expects all Human interface methods to work
    public static void manageWorker(Human worker) {
        System.out.println("Managing worker...");
        
        // This works for both
        worker.work();
        
        // These fail for Robot!
        try {
            worker.eat();  // Robot will throw exception
        } catch (UnsupportedOperationException e) {
            System.out.println("Error: " + e.getMessage());
        }
        
        try {
            worker.sleep(); // Robot will throw exception
        } catch (UnsupportedOperationException e) {
            System.out.println("Error: " + e.getMessage());
        }
        
        worker.think(); // Different behavior for robot
        
        try {
            worker.exercise(); // Robot will throw exception
        } catch (UnsupportedOperationException e) {
            System.out.println("Error: " + e.getMessage());
        }
    }
    
    // This method only needs work capability but is forced to accept full Human interface
    public static void assignWork(Human worker) {
        // We only need work() method, but must accept entire Human interface
        worker.work();
    }
}

/**
 * Demonstrates the ISP violation
 */
public class ISPViolationDemonstration {
    public static void main(String[] args) {
        System.out.println("=== Interface Segregation Principle - VIOLATION ===\n");
        
        System.out.println("1. Creating human worker:");
        Human person = new Person("Alice");
        
        System.out.println("\n2. Creating robot worker:");
        Human robot = new IndustrialRobot("R2D2-Industrial");
        
        System.out.println("\n3. Managing human worker (works fine):");
        WorkManager.manageWorker(person);
        
        System.out.println("\n4. Managing robot worker (many failures!):");
        WorkManager.manageWorker(robot);
        
        System.out.println("\n5. Problems with this design:");
        System.out.println("   - Robot forced to implement methods it doesn't need");
        System.out.println("   - Many methods throw UnsupportedOperationException");
        System.out.println("   - Clients can't rely on interface methods working");
        System.out.println("   - Robot-specific methods (powerDown, recharge) don't fit interface");
        System.out.println("   - Violates ISP: clients forced to depend on methods they don't use");
        
        // Try to access robot-specific functionality
        if (robot instanceof IndustrialRobot) {
            IndustrialRobot robotRef = (IndustrialRobot) robot;
            System.out.println("\n6. Robot-specific operations (require casting):");
            robotRef.powerDown();
            robotRef.recharge();
            robotRef.runDiagnostics();
        }
        
        System.out.println("\n=== Conclusion ===");
        System.out.println("The monolithic Human interface forces Robot to implement");
        System.out.println("irrelevant methods, violating the Interface Segregation Principle.");
        System.out.println("Clients depending on Human interface cannot rely on all methods working.");
    }
}