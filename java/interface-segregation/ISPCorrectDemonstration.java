/**
 * Interface Segregation Principle (ISP) - CORRECT Example
 * 
 * This example demonstrates proper interface segregation where clients
 * only depend on the methods they actually need.
 * 
 * Solution: Break down the monolithic Human interface into smaller,
 * focused interfaces that can be implemented independently.
 */

// CORRECT: Small, focused interfaces following ISP
interface Workable {
    void work();
}

interface Biological {
    void eat();
    void sleep();
    void breathe();
}

interface Cognitive {
    void think();
    void learn();
}

interface Social {
    void socialize();
    void communicate();
}

interface Physical {
    void exercise();
    void move();
}

interface Reproductive {
    void reproduce();
}

// Robot-specific interfaces
interface Mechanical {
    void powerDown();
    void powerUp();
    void runDiagnostics();
}

interface Rechargeable {
    void recharge();
    int getBatteryLevel();
}

interface Programmable {
    void loadProgram(String program);
    void executeProgram();
}

// Advanced robot capabilities
interface NetworkEnabled {
    void connectToNetwork();
    void sendData(String data);
    void receiveData();
}

/**
 * Human implementation - implements relevant interfaces
 */
class Person implements Workable, Biological, Cognitive, Social, Physical, Reproductive {
    private String name;
    private int energy;
    private boolean isAwake;
    private String currentTask;
    
    public Person(String name) {
        this.name = name;
        this.energy = 100;
        this.isAwake = true;
    }
    
    // Workable interface
    @Override
    public void work() {
        if (!isAwake) {
            System.out.println(name + " cannot work while sleeping!");
            return;
        }
        System.out.println(name + " is working on: " + (currentTask != null ? currentTask : "general tasks"));
        energy -= 20;
    }
    
    // Biological interface
    @Override
    public void eat() {
        System.out.println(name + " is eating nutritious food");
        energy += 30;
    }
    
    @Override
    public void sleep() {
        System.out.println(name + " is sleeping peacefully");
        isAwake = false;
        energy = 100;
    }
    
    @Override
    public void breathe() {
        System.out.println(name + " is breathing fresh air");
    }
    
    // Cognitive interface
    @Override
    public void think() {
        System.out.println(name + " is thinking creatively and analytically");
        energy -= 5;
    }
    
    @Override
    public void learn() {
        System.out.println(name + " is learning new skills");
        energy -= 10;
    }
    
    // Social interface
    @Override
    public void socialize() {
        System.out.println(name + " is socializing with friends");
        energy -= 10;
    }
    
    @Override
    public void communicate() {
        System.out.println(name + " is communicating through speech");
    }
    
    // Physical interface
    @Override
    public void exercise() {
        System.out.println(name + " is doing physical exercise");
        energy -= 15;
    }
    
    @Override
    public void move() {
        System.out.println(name + " is walking around");
        energy -= 5;
    }
    
    // Reproductive interface
    @Override
    public void reproduce() {
        System.out.println(name + " can participate in human reproduction");
    }
    
    public void setCurrentTask(String task) {
        this.currentTask = task;
    }
}

/**
 * Basic Robot - only implements interfaces it needs
 */
class IndustrialRobot implements Workable, Mechanical, Rechargeable, Programmable {
    private String model;
    private int batteryLevel;
    private boolean isOperational;
    private String currentProgram;
    
    public IndustrialRobot(String model) {
        this.model = model;
        this.batteryLevel = 100;
        this.isOperational = true;
    }
    
    // Workable interface
    @Override
    public void work() {
        if (!isOperational) {
            System.out.println(model + " is not operational!");
            return;
        }
        if (currentProgram == null) {
            System.out.println(model + " has no program loaded!");
            return;
        }
        System.out.println(model + " robot is executing work program: " + currentProgram);
        batteryLevel -= 10;
    }
    
    // Mechanical interface
    @Override
    public void powerDown() {
        System.out.println(model + " is powering down all systems");
        isOperational = false;
    }
    
    @Override
    public void powerUp() {
        System.out.println(model + " is powering up systems");
        isOperational = true;
    }
    
    @Override
    public void runDiagnostics() {
        System.out.println(model + " is running comprehensive diagnostics");
        System.out.println("Battery: " + batteryLevel + "%, Status: " + 
                          (isOperational ? "Operational" : "Offline"));
    }
    
    // Rechargeable interface
    @Override
    public void recharge() {
        System.out.println(model + " is recharging battery to full capacity");
        batteryLevel = 100;
    }
    
    @Override
    public int getBatteryLevel() {
        return batteryLevel;
    }
    
    // Programmable interface
    @Override
    public void loadProgram(String program) {
        System.out.println(model + " is loading program: " + program);
        this.currentProgram = program;
    }
    
    @Override
    public void executeProgram() {
        if (currentProgram == null) {
            System.out.println(model + " has no program to execute!");
            return;
        }
        System.out.println(model + " is executing program: " + currentProgram);
        batteryLevel -= 5;
    }
}

/**
 * Advanced Humanoid Robot - implements more interfaces including some human-like ones
 */
class HumanoidRobot implements Workable, Cognitive, Social, Physical, 
                              Mechanical, Rechargeable, Programmable, NetworkEnabled {
    private String name;
    private int batteryLevel;
    private boolean isOperational;
    private String currentProgram;
    private boolean networkConnected;
    
    public HumanoidRobot(String name) {
        this.name = name;
        this.batteryLevel = 100;
        this.isOperational = true;
        this.networkConnected = false;
    }
    
    // Workable interface
    @Override
    public void work() {
        if (!isOperational) {
            System.out.println(name + " is not operational!");
            return;
        }
        System.out.println(name + " humanoid robot is performing complex work tasks");
        batteryLevel -= 8;
    }
    
    // Cognitive interface - robots can have AI cognition
    @Override
    public void think() {
        System.out.println(name + " is processing information using AI algorithms");
        batteryLevel -= 3;
    }
    
    @Override
    public void learn() {
        System.out.println(name + " is updating neural networks with new data");
        batteryLevel -= 5;
    }
    
    // Social interface - humanoid robots can be social
    @Override
    public void socialize() {
        System.out.println(name + " is engaging in social interaction protocols");
        batteryLevel -= 4;
    }
    
    @Override
    public void communicate() {
        System.out.println(name + " is communicating through speech synthesis");
    }
    
    // Physical interface - humanoid robots can move
    @Override
    public void move() {
        System.out.println(name + " is walking with bipedal locomotion");
        batteryLevel -= 6;
    }
    
    @Override
    public void exercise() {
        System.out.println(name + " is performing calibration movements");
        batteryLevel -= 7;
    }
    
    // Mechanical, Rechargeable, Programmable interfaces (inherited from IndustrialRobot logic)
    @Override
    public void powerDown() {
        System.out.println(name + " humanoid is entering sleep mode");
        isOperational = false;
    }
    
    @Override
    public void powerUp() {
        System.out.println(name + " humanoid is activating all systems");
        isOperational = true;
    }
    
    @Override
    public void runDiagnostics() {
        System.out.println(name + " is running humanoid system diagnostics");
        System.out.println("Battery: " + batteryLevel + "%, Network: " + 
                          (networkConnected ? "Connected" : "Disconnected"));
    }
    
    @Override
    public void recharge() {
        System.out.println(name + " is recharging via induction pad");
        batteryLevel = 100;
    }
    
    @Override
    public int getBatteryLevel() {
        return batteryLevel;
    }
    
    @Override
    public void loadProgram(String program) {
        System.out.println(name + " is loading behavioral program: " + program);
        this.currentProgram = program;
    }
    
    @Override
    public void executeProgram() {
        if (currentProgram == null) {
            System.out.println(name + " has no behavioral program loaded!");
            return;
        }
        System.out.println(name + " is executing behavioral program: " + currentProgram);
        batteryLevel -= 3;
    }
    
    // NetworkEnabled interface
    @Override
    public void connectToNetwork() {
        System.out.println(name + " is connecting to robot network");
        networkConnected = true;
    }
    
    @Override
    public void sendData(String data) {
        if (!networkConnected) {
            System.out.println(name + " is not connected to network!");
            return;
        }
        System.out.println(name + " is sending data: " + data);
    }
    
    @Override
    public void receiveData() {
        if (!networkConnected) {
            System.out.println(name + " is not connected to network!");
            return;
        }
        System.out.println(name + " is receiving network data");
    }
}

/**
 * Client classes that only depend on interfaces they need
 */
class WorkManager {
    // Only depends on Workable interface - works with any worker type
    public static void assignTask(Workable worker, String task) {
        System.out.println("Assigning task: " + task);
        if (worker instanceof Person) {
            ((Person) worker).setCurrentTask(task);
        } else if (worker instanceof Programmable) {
            ((Programmable) worker).loadProgram(task);
        }
        worker.work();
    }
    
    // Only needs workers, doesn't care about other capabilities
    public static void manageWorkforce(Workable... workers) {
        System.out.println("Managing workforce of " + workers.length + " workers");
        for (Workable worker : workers) {
            worker.work();
        }
    }
}

class MaintenanceManager {
    // Only depends on Mechanical interface
    public static void performMaintenance(Mechanical device) {
        System.out.println("Starting maintenance routine...");
        device.runDiagnostics();
        device.powerDown();
        // Perform maintenance
        System.out.println("Maintenance completed");
        device.powerUp();
    }
    
    // Only depends on Rechargeable interface
    public static void manageCharge(Rechargeable device) {
        if (device.getBatteryLevel() < 20) {
            System.out.println("Battery low, initiating recharge");
            device.recharge();
        } else {
            System.out.println("Battery sufficient: " + device.getBatteryLevel() + "%");
        }
    }
}

class EducationManager {
    // Only depends on Cognitive interface
    public static void conductTraining(Cognitive learner) {
        System.out.println("Starting cognitive training session...");
        learner.think();
        learner.learn();
    }
}

class SocialCoordinator {
    // Only depends on Social interface
    public static void organizeSocialEvent(Social... participants) {
        System.out.println("Organizing social interaction...");
        for (Social participant : participants) {
            participant.socialize();
            participant.communicate();
        }
    }
}

/**
 * Demonstrates proper Interface Segregation
 */
public class ISPCorrectDemonstration {
    public static void main(String[] args) {
        System.out.println("=== Interface Segregation Principle - CORRECT ===\n");
        
        // Create different types of workers
        Person alice = new Person("Alice");
        IndustrialRobot factoryBot = new IndustrialRobot("FactoryBot-3000");
        HumanoidRobot android = new HumanoidRobot("Android-Sara");
        
        System.out.println("1. Work Management (only needs Workable interface):");
        WorkManager.assignTask(alice, "Design new product");
        WorkManager.assignTask(factoryBot, "Assembly line task");
        WorkManager.assignTask(android, "Customer service");
        
        System.out.println("\n2. Managing workforce (polymorphic with Workable):");
        WorkManager.manageWorkforce(alice, factoryBot, android);
        
        System.out.println("\n3. Maintenance (only for Mechanical devices):");
        MaintenanceManager.performMaintenance(factoryBot);
        MaintenanceManager.performMaintenance(android);
        // Note: Cannot pass alice here - she's not Mechanical!
        
        System.out.println("\n4. Battery Management (only for Rechargeable devices):");
        MaintenanceManager.manageCharge(factoryBot);
        MaintenanceManager.manageCharge(android);
        // Note: Cannot pass alice here - she's not Rechargeable!
        
        System.out.println("\n5. Cognitive Training (for thinking entities):");
        EducationManager.conductTraining(alice);
        EducationManager.conductTraining(android);  // Humanoid can think too!
        // Note: Cannot pass factoryBot - it's not Cognitive!
        
        System.out.println("\n6. Social Coordination (for social entities):");
        SocialCoordinator.organizeSocialEvent(alice, android);
        // Note: Cannot pass factoryBot - it's not Social!
        
        System.out.println("\n7. Network Operations (only for NetworkEnabled):");
        if (android instanceof NetworkEnabled) {
            NetworkEnabled networkBot = (NetworkEnabled) android;
            networkBot.connectToNetwork();
            networkBot.sendData("Status update");
            networkBot.receiveData();
        }
        
        System.out.println("\n=== Benefits of Interface Segregation ===");
        System.out.println("✓ Clients only depend on methods they actually use");
        System.out.println("✓ No forced implementation of irrelevant methods");
        System.out.println("✓ High cohesion within each interface");
        System.out.println("✓ Easy to extend with new capabilities");
        System.out.println("✓ Better testability and maintainability");
        System.out.println("✓ Follows Single Responsibility at interface level");
        
        System.out.println("\n=== Interface Composition Examples ===");
        System.out.println("Person implements: Workable + Biological + Cognitive + Social + Physical + Reproductive");
        System.out.println("IndustrialRobot implements: Workable + Mechanical + Rechargeable + Programmable");
        System.out.println("HumanoidRobot implements: Workable + Cognitive + Social + Physical + Mechanical + Rechargeable + Programmable + NetworkEnabled");
    }
}