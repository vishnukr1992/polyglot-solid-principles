/**
 * Interface Segregation Principle (ISP) - VIOLATION Example (JavaScript)
 * 
 * This example demonstrates how forcing a Robot to implement a Human interface
 * violates the Interface Segregation Principle in JavaScript.
 * 
 * JavaScript doesn't have formal interfaces, but we can simulate them
 * through duck typing and expected method contracts.
 */

/**
 * VIOLATION: Monolithic "interface" (expected contract) that forces
 * all implementations to have methods they don't need
 */

/**
 * Human class - naturally implements all expected methods
 */
class Person {
    constructor(name) {
        this.name = name;
        this.energy = 100;
        this.isAwake = true;
    }
    
    work() {
        if (!this.isAwake) {
            console.log(`${this.name} cannot work while sleeping!`);
            return;
        }
        console.log(`${this.name} is working productively`);
        this.energy -= 20;
    }
    
    eat() {
        console.log(`${this.name} is eating delicious food`);
        this.energy += 30;
    }
    
    sleep() {
        console.log(`${this.name} is sleeping peacefully`);
        this.isAwake = false;
        this.energy = 100;
    }
    
    breathe() {
        console.log(`${this.name} is breathing fresh air`);
        // Essential biological function
    }
    
    think() {
        console.log(`${this.name} is thinking creatively`);
        this.energy -= 5;
    }
    
    reproduce() {
        console.log(`${this.name} can participate in reproduction`);
    }
    
    exercise() {
        console.log(`${this.name} is doing physical exercise`);
        this.energy -= 15;
    }
    
    socialize() {
        console.log(`${this.name} is socializing with friends`);
        this.energy -= 10;
    }
    
    // Additional human methods
    feelEmotions() {
        console.log(`${this.name} is experiencing complex emotions`);
    }
    
    dream() {
        if (!this.isAwake) {
            console.log(`${this.name} is dreaming while sleeping`);
        }
    }
}

/**
 * VIOLATION: Robot forced to implement Human "interface"
 * This violates ISP because Robot doesn't need most of these methods
 */
class IndustrialRobot {
    constructor(model) {
        this.model = model;
        this.batteryLevel = 100;
        this.isOperational = true;
    }
    
    work() {
        if (!this.isOperational) {
            console.log(`${this.model} is not operational!`);
            return;
        }
        console.log(`${this.model} robot is performing industrial work`);
        this.batteryLevel -= 10;
    }
    
    // VIOLATION: Forced to implement irrelevant methods
    eat() {
        // Makes no sense for a robot!
        throw new Error("Robots don't eat food!");
    }
    
    sleep() {
        // Robots don't sleep, they power down
        throw new Error("Robots don't sleep! Use powerDown() instead");
    }
    
    breathe() {
        // Robots don't breathe
        throw new Error("Robots don't breathe air!");
    }
    
    think() {
        // Different kind of thinking (computational)
        console.log(`${this.model} is processing data algorithmically`);
        this.batteryLevel -= 2;
    }
    
    reproduce() {
        // Robots are manufactured, not reproduced
        throw new Error("Robots are manufactured, not reproduced!");
    }
    
    exercise() {
        // Robots don't need exercise
        throw new Error("Robots don't need exercise!");
    }
    
    socialize() {
        // Basic communication only
        console.log(`${this.model} is communicating via network protocols`);
    }
    
    feelEmotions() {
        // Robots don't feel emotions
        throw new Error("Robots don't have emotions!");
    }
    
    dream() {
        // Robots don't dream
        throw new Error("Robots don't dream!");
    }
    
    // Robot-specific methods that don't fit in Human "interface"
    powerDown() {
        console.log(`${this.model} is powering down systems`);
        this.isOperational = false;
    }
    
    recharge() {
        console.log(`${this.model} is recharging battery`);
        this.batteryLevel = 100;
    }
    
    runDiagnostics() {
        console.log(`${this.model} is running system diagnostics`);
        console.log(`Battery: ${this.batteryLevel}%, Status: ${this.isOperational ? 'Operational' : 'Offline'}`);
    }
    
    executeProgram(program) {
        console.log(`${this.model} is executing program: ${program}`);
    }
}

/**
 * Client code that expects Human "interface" contract
 */
class WorkManager {
    // This method expects all Human interface methods to work
    static manageWorker(worker) {
        console.log("Managing worker...");
        
        // This works for both
        worker.work();
        
        // These fail for Robot!
        try {
            worker.eat();  // Robot will throw exception
        } catch (error) {
            console.log(`Error: ${error.message}`);
        }
        
        try {
            worker.sleep(); // Robot will throw exception
        } catch (error) {
            console.log(`Error: ${error.message}`);
        }
        
        worker.think(); // Different behavior for robot
        
        try {
            worker.exercise(); // Robot will throw exception
        } catch (error) {
            console.log(`Error: ${error.message}`);
        }
        
        try {
            worker.feelEmotions(); // Robot will throw exception
        } catch (error) {
            console.log(`Error: ${error.message}`);
        }
    }
    
    // This method only needs work capability but must accept full Human "interface"
    static assignWork(worker, task) {
        console.log(`Assigning task: ${task}`);
        // We only need work() method, but must accept entire Human "interface"
        worker.work();
    }
    
    // Expects Human biological needs
    static provideCare(human) {
        console.log("Providing human care...");
        human.eat();
        human.sleep();
        human.breathe();
    }
}

/**
 * Function that checks if object follows Human contract
 */
function isHumanLike(obj) {
    const requiredMethods = [
        'work', 'eat', 'sleep', 'breathe', 'think', 
        'reproduce', 'exercise', 'socialize', 'feelEmotions', 'dream'
    ];
    
    return requiredMethods.every(method => typeof obj[method] === 'function');
}

/**
 * Demonstrates the ISP violation in JavaScript
 */
function demonstrateISPViolation() {
    console.log("=== Interface Segregation Principle - VIOLATION (JavaScript) ===");
    console.log("");
    
    console.log("1. Creating human worker:");
    const person = new Person("Alice");
    
    console.log("");
    console.log("2. Creating robot worker:");
    const robot = new IndustrialRobot("R2D2-Industrial");
    
    console.log("");
    console.log("3. Checking Human contract compliance:");
    console.log(`Person follows Human contract: ${isHumanLike(person)}`);
    console.log(`Robot follows Human contract: ${isHumanLike(robot)}`);
    
    console.log("");
    console.log("4. Managing human worker (works fine):");
    WorkManager.manageWorker(person);
    
    console.log("");
    console.log("5. Managing robot worker (many failures!):");
    WorkManager.manageWorker(robot);
    
    console.log("");
    console.log("6. Attempting to provide human care to robot:");
    try {
        WorkManager.provideCare(robot); // This will fail!
    } catch (error) {
        console.log(`Failed to provide care: ${error.message}`);
    }
    
    console.log("");
    console.log("7. Problems with this design:");
    console.log("   - Robot forced to implement methods it doesn't need");
    console.log("   - Many methods throw errors instead of working");
    console.log("   - Clients can't rely on interface methods working");
    console.log("   - Robot-specific methods don't fit Human contract");
    console.log("   - Violates ISP: clients forced to depend on methods they don't use");
    
    console.log("");
    console.log("8. Robot-specific operations (not part of Human contract):");
    robot.powerDown();
    robot.recharge();
    robot.runDiagnostics();
    robot.executeProgram("Industrial Assembly v2.1");
    
    console.log("");
    console.log("=== Conclusion ===");
    console.log("The monolithic Human contract forces Robot to implement");
    console.log("irrelevant methods, violating the Interface Segregation Principle.");
    console.log("Clients depending on Human contract cannot rely on all methods working.");
}

// Export for use in other modules
if (typeof module !== 'undefined' && module.exports) {
    module.exports = {
        Person,
        IndustrialRobot,
        WorkManager,
        demonstrateISPViolation,
        isHumanLike
    };
}

// Export for use in other modules
if (typeof module !== 'undefined' && module.exports) {
    module.exports = {
        Person,
        IndustrialRobot,
        WorkManager,
        demonstrateISPViolation,
        isHumanLike
    };
}

// Run demonstration if this file is executed directly
if (typeof require !== 'undefined' && require.main === module) {
    demonstrateISPViolation();
}