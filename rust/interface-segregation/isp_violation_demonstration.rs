/*
 * Interface Segregation Principle (ISP) - VIOLATION Example (Rust)
 * 
 * This example demonstrates how forcing a Robot to implement a Human trait
 * violates the Interface Segregation Principle.
 * 
 * Problem: Robot is forced to implement biological methods that make no sense
 * for a robot, leading to empty implementations or panics.
 */

use std::fmt;

// VIOLATION: Monolithic trait forces all implementations to have methods they don't need
trait Human {
    fn work(&mut self);
    fn eat(&mut self) -> Result<(), String>;
    fn sleep(&mut self) -> Result<(), String>;
    fn breathe(&mut self) -> Result<(), String>;
    fn think(&mut self);
    fn reproduce(&mut self) -> Result<(), String>;
    fn exercise(&mut self) -> Result<(), String>;
    fn socialize(&mut self);
    fn feel_emotions(&mut self) -> Result<(), String>;
    fn dream(&mut self) -> Result<(), String>;
}

// Person implementation - naturally implements all trait methods
#[derive(Debug)]
struct Person {
    name: String,
    energy: i32,
    is_awake: bool,
}

impl Person {
    fn new(name: String) -> Self {
        Person {
            name,
            energy: 100,
            is_awake: true,
        }
    }
}

impl Human for Person {
    fn work(&mut self) {
        if !self.is_awake {
            println!("{} cannot work while sleeping!", self.name);
            return;
        }
        println!("{} is working productively", self.name);
        self.energy -= 20;
    }
    
    fn eat(&mut self) -> Result<(), String> {
        println!("{} is eating delicious food", self.name);
        self.energy += 30;
        Ok(())
    }
    
    fn sleep(&mut self) -> Result<(), String> {
        println!("{} is sleeping peacefully", self.name);
        self.is_awake = false;
        self.energy = 100;
        Ok(())
    }
    
    fn breathe(&mut self) -> Result<(), String> {
        println!("{} is breathing fresh air", self.name);
        Ok(())
    }
    
    fn think(&mut self) {
        println!("{} is thinking creatively", self.name);
        self.energy -= 5;
    }
    
    fn reproduce(&mut self) -> Result<(), String> {
        println!("{} can participate in reproduction", self.name);
        Ok(())
    }
    
    fn exercise(&mut self) -> Result<(), String> {
        println!("{} is doing physical exercise", self.name);
        self.energy -= 15;
        Ok(())
    }
    
    fn socialize(&mut self) {
        println!("{} is socializing with friends", self.name);
        self.energy -= 10;
    }
    
    fn feel_emotions(&mut self) -> Result<(), String> {
        println!("{} is experiencing complex emotions", self.name);
        Ok(())
    }
    
    fn dream(&mut self) -> Result<(), String> {
        if !self.is_awake {
            println!("{} is dreaming while sleeping", self.name);
        }
        Ok(())
    }
}

// VIOLATION: Robot forced to implement Human trait
// This violates ISP because Robot doesn't need most of these methods
#[derive(Debug)]
struct IndustrialRobot {
    model: String,
    battery_level: i32,
    is_operational: bool,
}

impl IndustrialRobot {
    fn new(model: String) -> Self {
        IndustrialRobot {
            model,
            battery_level: 100,
            is_operational: true,
        }
    }
    
    // Robot-specific methods that don't fit in Human trait
    fn power_down(&mut self) {
        println!("{} is powering down systems", self.model);
        self.is_operational = false;
    }
    
    fn recharge(&mut self) {
        println!("{} is recharging battery", self.model);
        self.battery_level = 100;
    }
    
    fn run_diagnostics(&self) {
        println!("{} is running system diagnostics", self.model);
        let status = if self.is_operational { "Operational" } else { "Offline" };
        println!("Battery: {}%, Status: {}", self.battery_level, status);
    }
    
    fn execute_program(&self, program: &str) {
        println!("{} is executing program: {}", self.model, program);
    }
}

impl Human for IndustrialRobot {
    fn work(&mut self) {
        if !self.is_operational {
            println!("{} is not operational!", self.model);
            return;
        }
        println!("{} robot is performing industrial work", self.model);
        self.battery_level -= 10;
    }
    
    // VIOLATION: Forced to implement irrelevant methods
    fn eat(&mut self) -> Result<(), String> {
        // Makes no sense for a robot!
        Err("Robots don't eat food!".to_string())
    }
    
    fn sleep(&mut self) -> Result<(), String> {
        // Robots don't sleep, they power down
        Err("Robots don't sleep! Use power_down() instead".to_string())
    }
    
    fn breathe(&mut self) -> Result<(), String> {
        // Robots don't breathe
        Err("Robots don't breathe air!".to_string())
    }
    
    fn think(&mut self) {
        // Different kind of thinking (computational)
        println!("{} is processing data algorithmically", self.model);
        self.battery_level -= 2;
    }
    
    fn reproduce(&mut self) -> Result<(), String> {
        // Robots are manufactured, not reproduced
        Err("Robots are manufactured, not reproduced!".to_string())
    }
    
    fn exercise(&mut self) -> Result<(), String> {
        // Robots don't need exercise
        Err("Robots don't need exercise!".to_string())
    }
    
    fn socialize(&mut self) {
        // Basic communication only
        println!("{} is communicating via network protocols", self.model);
    }
    
    fn feel_emotions(&mut self) -> Result<(), String> {
        // Robots don't feel emotions
        Err("Robots don't have emotions!".to_string())
    }
    
    fn dream(&mut self) -> Result<(), String> {
        // Robots don't dream
        Err("Robots don't dream!".to_string())
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Person({})", self.name)
    }
}

impl fmt::Display for IndustrialRobot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IndustrialRobot({})", self.model)
    }
}

// Client code that expects Human trait
struct WorkManager;

impl WorkManager {
    // This method expects all Human trait methods to work
    fn manage_worker(worker: &mut dyn Human) {
        println!("Managing worker...");
        
        // This works for both
        worker.work();
        
        // These fail for Robot!
        if let Err(e) = worker.eat() {
            println!("Error: {}", e);
        }
        
        if let Err(e) = worker.sleep() {
            println!("Error: {}", e);
        }
        
        worker.think(); // Different behavior for robot
        
        if let Err(e) = worker.exercise() {
            println!("Error: {}", e);
        }
        
        if let Err(e) = worker.feel_emotions() {
            println!("Error: {}", e);
        }
    }
    
    // This method only needs work capability but must accept full Human trait
    fn assign_work(worker: &mut dyn Human, task: &str) {
        println!("Assigning task: {}", task);
        // We only need work() method, but must accept entire Human trait
        worker.work();
    }
    
    // Expects Human biological needs
    fn provide_care(human: &mut dyn Human) -> Result<(), String> {
        println!("Providing human care...");
        
        human.eat().map_err(|e| format!("Failed to provide food: {}", e))?;
        human.sleep().map_err(|e| format!("Failed to provide rest: {}", e))?;
        human.breathe().map_err(|e| format!("Failed to provide air: {}", e))?;
        
        Ok(())
    }
}

// Function that checks if object follows Human trait contract
fn check_human_compliance() -> bool {
    // In Rust, if a type implements a trait, it automatically follows the contract
    // This is enforced at compile time
    true
}

// Demonstrates the ISP violation
fn main() {
    println!("=== Interface Segregation Principle - VIOLATION (Rust) ===");
    println!();
    
    println!("1. Creating human worker:");
    let mut person = Person::new("Alice".to_string());
    
    println!("\n2. Creating robot worker:");
    let mut robot = IndustrialRobot::new("R2D2-Industrial".to_string());
    
    println!("\n3. Checking Human trait compliance:");
    println!("Person follows Human trait: {}", check_human_compliance());
    println!("Robot follows Human trait: {}", check_human_compliance());
    
    println!("\n4. Managing human worker (works fine):");
    WorkManager::manage_worker(&mut person);
    
    println!("\n5. Managing robot worker (many failures!):");
    WorkManager::manage_worker(&mut robot);
    
    println!("\n6. Attempting to provide human care to robot:");
    if let Err(e) = WorkManager::provide_care(&mut robot) {
        println!("Failed to provide care: {}", e);
    }
    
    println!("\n7. Problems with this design:");
    println!("   - Robot forced to implement methods it doesn't need");
    println!("   - Many methods return Err instead of working");
    println!("   - Clients can't rely on trait methods working");
    println!("   - Robot-specific methods don't fit Human trait");
    println!("   - Violates ISP: clients forced to depend on methods they don't use");
    
    println!("\n8. Robot-specific operations (not part of Human trait):");
    robot.power_down();
    robot.recharge();
    robot.run_diagnostics();
    robot.execute_program("Industrial Assembly v2.1");
    
    println!("\n=== Conclusion ===");
    println!("The monolithic Human trait forces Robot to implement");
    println!("irrelevant methods, violating the Interface Segregation Principle.");
    println!("Clients depending on Human trait cannot rely on all methods working.");
}