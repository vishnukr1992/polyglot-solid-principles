/**
 * Interface Segregation Principle (ISP) - CORRECT Example (JavaScript)
 * 
 * This example demonstrates proper interface segregation where clients
 * only depend on the methods they actually need.
 * 
 * Solution: Break down the monolithic Human "interface" into smaller,
 * focused contracts that can be implemented independently.
 */

/**
 * CORRECT: Small, focused "interfaces" (expected contracts) following ISP
 * In JavaScript, we simulate interfaces through duck typing and expected method contracts
 */

// Workable contract - anything that can perform work
const WorkableContract = {
    work: 'function'
};

// Biological contract - living organisms
const BiologicalContract = {
    eat: 'function',
    sleep: 'function',
    breathe: 'function'
};

// Cognitive contract - thinking entities
const CognitiveContract = {
    think: 'function',
    learn: 'function'
};

// Social contract - entities that can interact socially
const SocialContract = {
    socialize: 'function',
    communicate: 'function'
};

// Physical contract - entities that can move and exercise
const PhysicalContract = {
    exercise: 'function',
    move: 'function'
};

// Mechanical contract - machines with power and diagnostic capabilities
const MechanicalContract = {
    powerDown: 'function',
    powerUp: 'function',
    runDiagnostics: 'function'
};

// Rechargeable contract - devices with battery management
const RechargeableContract = {
    recharge: 'function',
    getBatteryLevel: 'function'
};

// Programmable contract - devices that can execute programs
const ProgrammableContract = {
    loadProgram: 'function',
    executeProgram: 'function'
};

// NetworkEnabled contract - devices with network capabilities
const NetworkEnabledContract = {
    connectToNetwork: 'function',
    sendData: 'function',
    receiveData: 'function'
};

/**
 * Helper function to check if object follows a contract
 */
function followsContract(obj, contract) {
    return Object.keys(contract).every(method => typeof obj[method] === contract[method]);
}

/**
 * Helper function to get contracts followed by an object
 */
function getFollowedContracts(obj) {
    const contracts = {
        Workable: WorkableContract,
        Biological: BiologicalContract,
        Cognitive: CognitiveContract,
        Social: SocialContract,
        Physical: PhysicalContract,
        Mechanical: MechanicalContract,
        Rechargeable: RechargeableContract,
        Programmable: ProgrammableContract,
        NetworkEnabled: NetworkEnabledContract
    };
    
    return Object.keys(contracts).filter(name => followsContract(obj, contracts[name]));
}

/**
 * Human implementation - implements relevant contracts
 */
class Person {
    constructor(name) {
        this.name = name;
        this.energy = 100;
        this.isAwake = true;
        this.currentTask = null;
    }
    
    // Workable contract
    work() {
        if (!this.isAwake) {
            console.log(`${this.name} cannot work while sleeping!`);
            return;
        }
        console.log(`${this.name} is working on: ${this.currentTask || 'general tasks'}`);
        this.energy -= 20;
    }
    
    // Biological contract
    eat() {
        console.log(`${this.name} is eating nutritious food`);
        this.energy += 30;
    }
    
    sleep() {
        console.log(`${this.name} is sleeping peacefully`);
        this.isAwake = false;
        this.energy = 100;
    }
    
    breathe() {
        console.log(`${this.name} is breathing fresh air`);
    }
    
    // Cognitive contract
    think() {
        console.log(`${this.name} is thinking creatively and analytically`);
        this.energy -= 5;
    }
    
    learn() {
        console.log(`${this.name} is learning new skills`);
        this.energy -= 10;
    }
    
    // Social contract
    socialize() {
        console.log(`${this.name} is socializing with friends`);
        this.energy -= 10;
    }
    
    communicate() {
        console.log(`${this.name} is communicating through speech`);
    }
    
    // Physical contract
    exercise() {
        console.log(`${this.name} is doing physical exercise`);
        this.energy -= 15;
    }
    
    move() {
        console.log(`${this.name} is walking around`);
        this.energy -= 5;
    }
    
    setCurrentTask(task) {
        this.currentTask = task;
    }
}

/**
 * Basic Robot - only implements contracts it needs
 */
class IndustrialRobot {
    constructor(model) {
        this.model = model;
        this.batteryLevel = 100;
        this.isOperational = true;
        this.currentProgram = null;
    }
    
    // Workable contract
    work() {
        if (!this.isOperational) {
            console.log(`${this.model} is not operational!`);
            return;
        }
        if (!this.currentProgram) {
            console.log(`${this.model} has no program loaded!`);
            return;
        }
        console.log(`${this.model} robot is executing work program: ${this.currentProgram}`);
        this.batteryLevel -= 10;
    }
    
    // Mechanical contract
    powerDown() {
        console.log(`${this.model} is powering down all systems`);
        this.isOperational = false;
    }
    
    powerUp() {
        console.log(`${this.model} is powering up systems`);
        this.isOperational = true;
    }
    
    runDiagnostics() {
        console.log(`${this.model} is running comprehensive diagnostics`);
        console.log(`Battery: ${this.batteryLevel}%, Status: ${this.isOperational ? 'Operational' : 'Offline'}`);
    }
    
    // Rechargeable contract
    recharge() {
        console.log(`${this.model} is recharging battery to full capacity`);
        this.batteryLevel = 100;
    }
    
    getBatteryLevel() {
        return this.batteryLevel;
    }
    
    // Programmable contract
    loadProgram(program) {
        console.log(`${this.model} is loading program: ${program}`);
        this.currentProgram = program;
    }
    
    executeProgram() {
        if (!this.currentProgram) {
            console.log(`${this.model} has no program to execute!`);
            return;
        }
        console.log(`${this.model} is executing program: ${this.currentProgram}`);
        this.batteryLevel -= 5;
    }
}

/**
 * Advanced Humanoid Robot - implements more contracts including some human-like ones
 */
class HumanoidRobot {
    constructor(name) {
        this.name = name;
        this.batteryLevel = 100;
        this.isOperational = true;
        this.currentProgram = null;
        this.networkConnected = false;
    }
    
    // Workable contract
    work() {
        if (!this.isOperational) {
            console.log(`${this.name} is not operational!`);
            return;
        }
        console.log(`${this.name} humanoid robot is performing complex work tasks`);
        this.batteryLevel -= 8;
    }
    
    // Cognitive contract - robots can have AI cognition
    think() {
        console.log(`${this.name} is processing information using AI algorithms`);
        this.batteryLevel -= 3;
    }
    
    learn() {
        console.log(`${this.name} is updating neural networks with new data`);
        this.batteryLevel -= 5;
    }
    
    // Social contract - humanoid robots can be social
    socialize() {
        console.log(`${this.name} is engaging in social interaction protocols`);
        this.batteryLevel -= 4;
    }
    
    communicate() {
        console.log(`${this.name} is communicating through speech synthesis`);
    }
    
    // Physical contract - humanoid robots can move
    move() {
        console.log(`${this.name} is walking with bipedal locomotion`);
        this.batteryLevel -= 6;
    }
    
    exercise() {
        console.log(`${this.name} is performing calibration movements`);
        this.batteryLevel -= 7;
    }
    
    // Mechanical contract
    powerDown() {
        console.log(`${this.name} humanoid is entering sleep mode`);
        this.isOperational = false;
    }
    
    powerUp() {
        console.log(`${this.name} humanoid is activating all systems`);
        this.isOperational = true;
    }
    
    runDiagnostics() {
        console.log(`${this.name} is running humanoid system diagnostics`);
        console.log(`Battery: ${this.batteryLevel}%, Network: ${this.networkConnected ? 'Connected' : 'Disconnected'}`);
    }
    
    // Rechargeable contract
    recharge() {
        console.log(`${this.name} is recharging via induction pad`);
        this.batteryLevel = 100;
    }
    
    getBatteryLevel() {
        return this.batteryLevel;
    }
    
    // Programmable contract
    loadProgram(program) {
        console.log(`${this.name} is loading behavioral program: ${program}`);
        this.currentProgram = program;
    }
    
    executeProgram() {
        if (!this.currentProgram) {
            console.log(`${this.name} has no behavioral program loaded!`);
            return;
        }
        console.log(`${this.name} is executing behavioral program: ${this.currentProgram}`);
        this.batteryLevel -= 3;
    }
    
    // NetworkEnabled contract
    connectToNetwork() {
        console.log(`${this.name} is connecting to robot network`);
        this.networkConnected = true;
    }
    
    sendData(data) {
        if (!this.networkConnected) {
            console.log(`${this.name} is not connected to network!`);
            return;
        }
        console.log(`${this.name} is sending data: ${data}`);
    }
    
    receiveData() {
        if (!this.networkConnected) {
            console.log(`${this.name} is not connected to network!`);
            return;
        }
        console.log(`${this.name} is receiving network data`);
    }
}

/**
 * Client classes that only depend on contracts they need
 */
class WorkManager {
    // Only depends on Workable contract - works with any worker type
    static assignTask(worker, task) {
        if (!followsContract(worker, WorkableContract)) {
            throw new Error("Worker must follow Workable contract");
        }
        
        console.log(`Assigning task: ${task}`);
        if (worker.setCurrentTask) {
            worker.setCurrentTask(task);
        } else if (followsContract(worker, ProgrammableContract)) {
            worker.loadProgram(task);
        }
        worker.work();
    }
    
    // Only needs workers, doesn't care about other capabilities
    static manageWorkforce(...workers) {
        console.log(`Managing workforce of ${workers.length} workers`);
        workers.forEach(worker => {
            if (followsContract(worker, WorkableContract)) {
                worker.work();
            } else {
                console.log("Warning: Worker doesn't follow Workable contract");
            }
        });
    }
}

class MaintenanceManager {
    // Only depends on Mechanical contract
    static performMaintenance(device) {
        if (!followsContract(device, MechanicalContract)) {
            throw new Error("Device must follow Mechanical contract");
        }
        
        console.log("Starting maintenance routine...");
        device.runDiagnostics();
        device.powerDown();
        console.log("Maintenance completed");
        device.powerUp();
    }
    
    // Only depends on Rechargeable contract
    static manageCharge(device) {
        if (!followsContract(device, RechargeableContract)) {
            throw new Error("Device must follow Rechargeable contract");
        }
        
        if (device.getBatteryLevel() < 20) {
            console.log("Battery low, initiating recharge");
            device.recharge();
        } else {
            console.log(`Battery sufficient: ${device.getBatteryLevel()}%`);
        }
    }
}

class EducationManager {
    // Only depends on Cognitive contract
    static conductTraining(learner) {
        if (!followsContract(learner, CognitiveContract)) {
            throw new Error("Learner must follow Cognitive contract");
        }
        
        console.log("Starting cognitive training session...");
        learner.think();
        learner.learn();
    }
}

class SocialCoordinator {
    // Only depends on Social contract
    static organizeSocialEvent(...participants) {
        console.log("Organizing social interaction...");
        participants.forEach(participant => {
            if (followsContract(participant, SocialContract)) {
                participant.socialize();
                participant.communicate();
            } else {
                console.log("Warning: Participant doesn't follow Social contract");
            }
        });
    }
}

class BiologicalCareProvider {
    // Only depends on Biological contract
    static provideCare(organism) {
        if (!followsContract(organism, BiologicalContract)) {
            throw new Error("Organism must follow Biological contract");
        }
        
        console.log("Providing biological care...");
        organism.eat();
        organism.sleep();
        organism.breathe();
    }
}

/**
 * Demonstrates proper Interface Segregation in JavaScript
 */
function demonstrateISPCorrect() {
    console.log("=== Interface Segregation Principle - CORRECT (JavaScript) ===");
    console.log("");
    
    // Create different types of workers
    const alice = new Person("Alice");
    const factoryBot = new IndustrialRobot("FactoryBot-3000");
    const android = new HumanoidRobot("Android-Sara");
    
    console.log("1. Analyzing contract compliance:");
    console.log(`Alice follows contracts: ${getFollowedContracts(alice).join(', ')}`);
    console.log(`FactoryBot follows contracts: ${getFollowedContracts(factoryBot).join(', ')}`);
    console.log(`Android follows contracts: ${getFollowedContracts(android).join(', ')}`);
    
    console.log("");
    console.log("2. Work Management (only needs Workable contract):");
    WorkManager.assignTask(alice, "Design new product");
    WorkManager.assignTask(factoryBot, "Assembly line task");
    WorkManager.assignTask(android, "Customer service");
    
    console.log("");
    console.log("3. Managing workforce (polymorphic with Workable):");
    WorkManager.manageWorkforce(alice, factoryBot, android);
    
    console.log("");
    console.log("4. Maintenance (only for Mechanical devices):");
    // Note: Cannot pass alice here - she doesn't follow Mechanical contract!
    MaintenanceManager.performMaintenance(factoryBot);
    MaintenanceManager.performMaintenance(android);
    
    console.log("");
    console.log("5. Battery Management (only for Rechargeable devices):");
    // Note: Cannot pass alice here - she doesn't follow Rechargeable contract!
    MaintenanceManager.manageCharge(factoryBot);
    MaintenanceManager.manageCharge(android);
    
    console.log("");
    console.log("6. Cognitive Training (for thinking entities):");
    EducationManager.conductTraining(alice);
    EducationManager.conductTraining(android);  // Humanoid can think too!
    // Note: Cannot pass factoryBot - it doesn't follow Cognitive contract!
    
    console.log("");
    console.log("7. Social Coordination (for social entities):");
    SocialCoordinator.organizeSocialEvent(alice, android);
    // Note: Cannot pass factoryBot - it doesn't follow Social contract!
    
    console.log("");
    console.log("8. Biological Care (only for living organisms):");
    BiologicalCareProvider.provideCare(alice);
    // Note: Cannot pass robots - they don't follow Biological contract!
    
    console.log("");
    console.log("9. Network Operations (only for NetworkEnabled):");
    if (followsContract(android, NetworkEnabledContract)) {
        android.connectToNetwork();
        android.sendData("Status update");
        android.receiveData();
    }
    
    console.log("");
    console.log("=== Benefits of Interface Segregation ===");
    console.log("✓ Clients only depend on methods they actually use");
    console.log("✓ No forced implementation of irrelevant methods");
    console.log("✓ High cohesion within each contract");
    console.log("✓ Easy to extend with new capabilities");
    console.log("✓ Better testability and maintainability");
    console.log("✓ Duck typing works naturally with segregated contracts");
    
    console.log("");
    console.log("=== Contract Composition Examples ===");
    console.log("Person implements: Workable + Biological + Cognitive + Social + Physical");
    console.log("IndustrialRobot implements: Workable + Mechanical + Rechargeable + Programmable");
    console.log("HumanoidRobot implements: Workable + Cognitive + Social + Physical + Mechanical + Rechargeable + Programmable + NetworkEnabled");
    
    console.log("");
    console.log("10. Testing contract violations:");
    try {
        // This should fail - factoryBot doesn't follow Cognitive contract
        EducationManager.conductTraining(factoryBot);
    } catch (error) {
        console.log(`Expected error: ${error.message}`);
    }
    
    try {
        // This should fail - android doesn't follow Biological contract
        BiologicalCareProvider.provideCare(android);
    } catch (error) {
        console.log(`Expected error: ${error.message}`);
    }
}

// Export for use in other modules
if (typeof module !== 'undefined' && module.exports) {
    module.exports = {
        Person,
        IndustrialRobot,
        HumanoidRobot,
        WorkManager,
        MaintenanceManager,
        EducationManager,
        SocialCoordinator,
        BiologicalCareProvider,
        demonstrateISPCorrect,
        followsContract,
        getFollowedContracts,
        // Contract definitions
        WorkableContract,
        BiologicalContract,
        CognitiveContract,
        SocialContract,
        PhysicalContract,
        MechanicalContract,
        RechargeableContract,
        ProgrammableContract,
        NetworkEnabledContract
    };
}

// Run demonstration if this file is executed directly
if (typeof require !== 'undefined' && require.main === module) {
    demonstrateISPCorrect();
}