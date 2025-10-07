/*
 * Interface Segregation Principle (ISP) - CORRECT Example (Rust)
 * 
 * This example demonstrates proper interface segregation where clients
 * only depend on the trait methods they actually need.
 * 
 * Solution: Segregated traits allow each type to implement only
 * relevant capabilities, following the Interface Segregation Principle.
 */

use std::fmt;
use std::collections::HashMap;

// CORRECT: Segregated traits - clients only depend on what they need

// Core work capability trait
trait Workable {
    fn work(&mut self);
    fn get_work_efficiency(&self) -> f32;
}

// Biological needs trait (only for living beings)
trait Biological {
    fn eat(&mut self) -> Result<(), String>;
    fn sleep(&mut self) -> Result<(), String>;
    fn breathe(&mut self) -> Result<(), String>;
    fn get_health_status(&self) -> String;
}

// Cognitive abilities trait
trait Cognitive {
    fn think(&mut self);
    fn learn(&mut self, skill: &str);
    fn solve_problem(&self, problem: &str) -> String;
    fn get_intelligence_level(&self) -> i32;
}

// Social capabilities trait
trait Social {
    fn socialize(&mut self);
    fn communicate(&self, message: &str) -> String;
    fn form_relationships(&mut self) -> Result<(), String>;
}

// Physical capabilities trait
trait Physical {
    fn exercise(&mut self) -> Result<(), String>;
    fn feel_emotions(&mut self) -> Result<(), String>;
    fn reproduce(&mut self) -> Result<(), String>;
    fn get_physical_condition(&self) -> String;
}

// Mechanical operations trait (for robots and machines)
trait Mechanical {
    fn power_down(&mut self);
    fn run_diagnostics(&self) -> String;
    fn perform_maintenance(&mut self) -> Result<(), String>;
    fn get_mechanical_status(&self) -> String;
}

// Power management trait (for electronic devices)
trait Rechargeable {
    fn recharge(&mut self);
    fn get_battery_level(&self) -> i32;
    fn is_charging(&self) -> bool;
}

// Programmable behavior trait
trait Programmable {
    fn execute_program(&self, program: &str) -> Result<(), String>;
    fn install_software(&mut self, software: &str) -> Result<(), String>;
    fn get_installed_programs(&self) -> Vec<String>;
}

// Network connectivity trait
trait NetworkEnabled {
    fn connect_to_network(&mut self, network: &str) -> Result<(), String>;
    fn send_data(&self, data: &str) -> Result<(), String>;
    fn receive_data(&mut self) -> Result<String, String>;
    fn get_network_status(&self) -> String;
}

// Human implementation - only implements relevant traits
#[derive(Debug)]
struct Person {
    name: String,
    energy: i32,
    is_awake: bool,
    skills: Vec<String>,
    intelligence: i32,
    health: String,
    relationships: Vec<String>,
}

impl Person {
    fn new(name: String) -> Self {
        Person {
            name,
            energy: 100,
            is_awake: true,
            skills: vec!["Basic Communication".to_string()],
            intelligence: 100,
            health: "Healthy".to_string(),
            relationships: vec![],
        }
    }
}

impl Workable for Person {
    fn work(&mut self) {
        if !self.is_awake {
            println!("{} cannot work while sleeping!", self.name);
            return;
        }
        println!("{} is working with human creativity", self.name);
        self.energy -= 20;
    }
    
    fn get_work_efficiency(&self) -> f32 {
        if self.is_awake { self.energy as f32 / 100.0 } else { 0.0 }
    }
}

impl Biological for Person {
    fn eat(&mut self) -> Result<(), String> {
        println!("{} is eating delicious food", self.name);
        self.energy = (self.energy + 30).min(100);
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
    
    fn get_health_status(&self) -> String {
        self.health.clone()
    }
}

impl Cognitive for Person {
    fn think(&mut self) {
        println!("{} is thinking creatively and emotionally", self.name);
        self.energy -= 5;
    }
    
    fn learn(&mut self, skill: &str) {
        println!("{} is learning: {}", self.name, skill);
        self.skills.push(skill.to_string());
        self.intelligence += 2;
    }
    
    fn solve_problem(&self, problem: &str) -> String {
        format!("{} is solving '{}' using human intuition and experience", self.name, problem)
    }
    
    fn get_intelligence_level(&self) -> i32 {
        self.intelligence
    }
}

impl Social for Person {
    fn socialize(&mut self) {
        println!("{} is socializing and building relationships", self.name);
        self.energy -= 10;
    }
    
    fn communicate(&self, message: &str) -> String {
        format!("{} says: '{}'", self.name, message)
    }
    
    fn form_relationships(&mut self) -> Result<(), String> {
        println!("{} is forming meaningful relationships", self.name);
        self.relationships.push("New Friend".to_string());
        Ok(())
    }
}

impl Physical for Person {
    fn exercise(&mut self) -> Result<(), String> {
        println!("{} is doing physical exercise", self.name);
        self.energy -= 15;
        self.health = "Excellent".to_string();
        Ok(())
    }
    
    fn feel_emotions(&mut self) -> Result<(), String> {
        println!("{} is experiencing complex human emotions", self.name);
        Ok(())
    }
    
    fn reproduce(&mut self) -> Result<(), String> {
        println!("{} can participate in human reproduction", self.name);
        Ok(())
    }
    
    fn get_physical_condition(&self) -> String {
        format!("Energy: {}, Health: {}", self.energy, self.health)
    }
}

// Robot implementation - only implements relevant traits
#[derive(Debug)]
struct IndustrialRobot {
    model: String,
    battery_level: i32,
    is_operational: bool,
    programs: Vec<String>,
    network_status: String,
    is_charging: bool,
}

impl IndustrialRobot {
    fn new(model: String) -> Self {
        IndustrialRobot {
            model,
            battery_level: 100,
            is_operational: true,
            programs: vec!["Basic Operations".to_string()],
            network_status: "Disconnected".to_string(),
            is_charging: false,
        }
    }
}

impl Workable for IndustrialRobot {
    fn work(&mut self) {
        if !self.is_operational {
            println!("{} is not operational!", self.model);
            return;
        }
        println!("{} robot is performing precise industrial work", self.model);
        self.battery_level -= 10;
    }
    
    fn get_work_efficiency(&self) -> f32 {
        if self.is_operational { 
            (self.battery_level as f32 / 100.0) * 1.2  // Robots are more efficient
        } else { 
            0.0 
        }
    }
}

impl Cognitive for IndustrialRobot {
    fn think(&mut self) {
        println!("{} is processing data and running algorithms", self.model);
        self.battery_level -= 2;
    }
    
    fn learn(&mut self, skill: &str) {
        println!("{} is updating algorithms for: {}", self.model, skill);
        self.programs.push(format!("Algorithm: {}", skill));
    }
    
    fn solve_problem(&self, problem: &str) -> String {
        format!("{} is solving '{}' using computational analysis", self.model, problem)
    }
    
    fn get_intelligence_level(&self) -> i32 {
        self.programs.len() as i32 * 25  // Based on installed programs
    }
}

impl Mechanical for IndustrialRobot {
    fn power_down(&mut self) {
        println!("{} is safely powering down all systems", self.model);
        self.is_operational = false;
    }
    
    fn run_diagnostics(&self) -> String {
        let status = if self.is_operational { "Operational" } else { "Offline" };
        format!("Robot: {}, Battery: {}%, Status: {}, Programs: {}", 
                self.model, self.battery_level, status, self.programs.len())
    }
    
    fn perform_maintenance(&mut self) -> Result<(), String> {
        println!("{} is performing self-maintenance", self.model);
        self.battery_level = (self.battery_level + 10).min(100);
        Ok(())
    }
    
    fn get_mechanical_status(&self) -> String {
        if self.is_operational { "All systems operational".to_string() } 
        else { "Systems offline".to_string() }
    }
}

impl Rechargeable for IndustrialRobot {
    fn recharge(&mut self) {
        println!("{} is recharging battery", self.model);
        self.is_charging = true;
        self.battery_level = 100;
        self.is_charging = false;
    }
    
    fn get_battery_level(&self) -> i32 {
        self.battery_level
    }
    
    fn is_charging(&self) -> bool {
        self.is_charging
    }
}

impl Programmable for IndustrialRobot {
    fn execute_program(&self, program: &str) -> Result<(), String> {
        if !self.is_operational {
            return Err("Robot is not operational".to_string());
        }
        println!("{} is executing program: {}", self.model, program);
        Ok(())
    }
    
    fn install_software(&mut self, software: &str) -> Result<(), String> {
        println!("{} is installing software: {}", self.model, software);
        self.programs.push(software.to_string());
        Ok(())
    }
    
    fn get_installed_programs(&self) -> Vec<String> {
        self.programs.clone()
    }
}

impl NetworkEnabled for IndustrialRobot {
    fn connect_to_network(&mut self, network: &str) -> Result<(), String> {
        println!("{} is connecting to network: {}", self.model, network);
        self.network_status = format!("Connected to {}", network);
        Ok(())
    }
    
    fn send_data(&self, data: &str) -> Result<(), String> {
        if self.network_status == "Disconnected" {
            return Err("Not connected to network".to_string());
        }
        println!("{} is sending data: {}", self.model, data);
        Ok(())
    }
    
    fn receive_data(&mut self) -> Result<String, String> {
        if self.network_status == "Disconnected" {
            return Err("Not connected to network".to_string());
        }
        let data = "Sensor data received";
        println!("{} received: {}", self.model, data);
        Ok(data.to_string())
    }
    
    fn get_network_status(&self) -> String {
        self.network_status.clone()
    }
}

// Humanoid Robot - extends Robot concept with additional human-like capabilities
#[derive(Debug)]
struct HumanoidRobot {
    model: String,
    battery_level: i32,
    is_operational: bool,
    programs: Vec<String>,
    network_status: String,
    is_charging: bool,
    social_protocols: Vec<String>,
}

impl HumanoidRobot {
    fn new(model: String) -> Self {
        HumanoidRobot {
            model,
            battery_level: 100,
            is_operational: true,
            programs: vec!["Basic Operations".to_string(), "Social Interaction".to_string()],
            network_status: "Disconnected".to_string(),
            is_charging: false,
            social_protocols: vec!["Polite Communication".to_string()],
        }
    }
}

impl Workable for HumanoidRobot {
    fn work(&mut self) {
        if !self.is_operational {
            println!("{} is not operational!", self.model);
            return;
        }
        println!("{} humanoid robot is performing human-like work tasks", self.model);
        self.battery_level -= 8;
    }
    
    fn get_work_efficiency(&self) -> f32 {
        if self.is_operational { 
            (self.battery_level as f32 / 100.0) * 1.1  // Efficient but human-like
        } else { 
            0.0 
        }
    }
}

impl Cognitive for HumanoidRobot {
    fn think(&mut self) {
        println!("{} is processing with advanced AI and learning algorithms", self.model);
        self.battery_level -= 3;
    }
    
    fn learn(&mut self, skill: &str) {
        println!("{} is learning social skill: {}", self.model, skill);
        self.programs.push(format!("Social Algorithm: {}", skill));
    }
    
    fn solve_problem(&self, problem: &str) -> String {
        format!("{} is solving '{}' using AI with human-like reasoning", self.model, problem)
    }
    
    fn get_intelligence_level(&self) -> i32 {
        (self.programs.len() as i32 * 30) + (self.social_protocols.len() as i32 * 10)
    }
}

impl Social for HumanoidRobot {
    fn socialize(&mut self) {
        println!("{} is engaging in programmed social interactions", self.model);
        self.battery_level -= 5;
    }
    
    fn communicate(&self, message: &str) -> String {
        format!("{} communicates: '{}'", self.model, message)
    }
    
    fn form_relationships(&mut self) -> Result<(), String> {
        println!("{} is forming programmed social bonds", self.model);
        self.social_protocols.push("New Social Protocol".to_string());
        Ok(())
    }
}

impl Mechanical for HumanoidRobot {
    fn power_down(&mut self) {
        println!("{} is gracefully shutting down with social protocols", self.model);
        self.is_operational = false;
    }
    
    fn run_diagnostics(&self) -> String {
        let status = if self.is_operational { "Operational" } else { "Offline" };
        format!("Humanoid: {}, Battery: {}%, Status: {}, Programs: {}, Social: {}", 
                self.model, self.battery_level, status, self.programs.len(), self.social_protocols.len())
    }
    
    fn perform_maintenance(&mut self) -> Result<(), String> {
        println!("{} is performing maintenance on social and mechanical systems", self.model);
        self.battery_level = (self.battery_level + 15).min(100);
        Ok(())
    }
    
    fn get_mechanical_status(&self) -> String {
        if self.is_operational { "All humanoid systems operational".to_string() } 
        else { "Systems offline".to_string() }
    }
}

impl Rechargeable for HumanoidRobot {
    fn recharge(&mut self) {
        println!("{} is recharging with energy-efficient protocols", self.model);
        self.is_charging = true;
        self.battery_level = 100;
        self.is_charging = false;
    }
    
    fn get_battery_level(&self) -> i32 {
        self.battery_level
    }
    
    fn is_charging(&self) -> bool {
        self.is_charging
    }
}

impl Programmable for HumanoidRobot {
    fn execute_program(&self, program: &str) -> Result<(), String> {
        if !self.is_operational {
            return Err("Humanoid robot is not operational".to_string());
        }
        println!("{} is executing human-interaction program: {}", self.model, program);
        Ok(())
    }
    
    fn install_software(&mut self, software: &str) -> Result<(), String> {
        println!("{} is installing human-interaction software: {}", self.model, software);
        self.programs.push(software.to_string());
        Ok(())
    }
    
    fn get_installed_programs(&self) -> Vec<String> {
        self.programs.clone()
    }
}

impl NetworkEnabled for HumanoidRobot {
    fn connect_to_network(&mut self, network: &str) -> Result<(), String> {
        println!("{} is connecting to social network: {}", self.model, network);
        self.network_status = format!("Connected to {}", network);
        Ok(())
    }
    
    fn send_data(&self, data: &str) -> Result<(), String> {
        if self.network_status == "Disconnected" {
            return Err("Not connected to network".to_string());
        }
        println!("{} is sharing social data: {}", self.model, data);
        Ok(())
    }
    
    fn receive_data(&mut self) -> Result<String, String> {
        if self.network_status == "Disconnected" {
            return Err("Not connected to network".to_string());
        }
        let data = "Social interaction data received";
        println!("{} received: {}", self.model, data);
        Ok(data.to_string())
    }
    
    fn get_network_status(&self) -> String {
        self.network_status.clone()
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

impl fmt::Display for HumanoidRobot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HumanoidRobot({})", self.model)
    }
}

// CORRECT: Segregated managers that depend only on needed traits

struct WorkManager;

impl WorkManager {
    // Only depends on Workable trait
    fn assign_work(worker: &mut dyn Workable, task: &str) {
        println!("Assigning work task: {}", task);
        worker.work();
        println!("Work efficiency: {:.2}", worker.get_work_efficiency());
    }
    
    // Can work with any Workable + Cognitive combination
    fn assign_complex_work(worker: &mut (dyn Workable + Cognitive), task: &str) {
        println!("Assigning complex task: {}", task);
        worker.think();
        let solution = worker.solve_problem(task);
        println!("Solution approach: {}", solution);
        worker.work();
    }
}

struct BiologicalCareProvider;

impl BiologicalCareProvider {
    // Only depends on Biological trait
    fn provide_care(being: &mut dyn Biological) -> Result<(), String> {
        println!("Providing biological care...");
        being.eat()?;
        being.sleep()?;
        being.breathe()?;
        println!("Health status: {}", being.get_health_status());
        Ok(())
    }
    
    // Combines biological and physical care
    fn provide_wellness_care(being: &mut (dyn Biological + Physical)) -> Result<(), String> {
        println!("Providing comprehensive wellness care...");
        being.eat()?;
        being.exercise()?;
        println!("Physical condition: {}", being.get_physical_condition());
        Ok(())
    }
}

struct TechnicalSupportManager;

impl TechnicalSupportManager {
    // Only depends on Mechanical trait
    fn perform_maintenance(device: &mut dyn Mechanical) -> Result<(), String> {
        println!("Performing technical maintenance...");
        println!("Diagnostics: {}", device.run_diagnostics());
        device.perform_maintenance()?;
        println!("Status: {}", device.get_mechanical_status());
        Ok(())
    }
    
    // Depends on both Mechanical and Rechargeable
    fn service_electronic_device(device: &mut (dyn Mechanical + Rechargeable)) -> Result<(), String> {
        println!("Servicing electronic device...");
        println!("Battery level: {}%", device.get_battery_level());
        device.recharge();
        device.perform_maintenance()?;
        Ok(())
    }
}

struct SocialCoordinator;

impl SocialCoordinator {
    // Only depends on Social trait
    fn facilitate_interaction(participant: &mut dyn Social) {
        println!("Facilitating social interaction...");
        participant.socialize();
        let response = participant.communicate("Hello, how are you?");
        println!("Response: {}", response);
    }
    
    // Combines social and cognitive capabilities
    fn organize_collaborative_work(participant: &mut (dyn Social + Cognitive)) {
        println!("Organizing collaborative work session...");
        participant.think();
        participant.socialize();
        participant.learn("Team Collaboration");
    }
}

struct ITManager;

impl ITManager {
    // Only depends on Programmable trait
    fn deploy_software(device: &mut dyn Programmable, software: &str) -> Result<(), String> {
        println!("Deploying software...");
        device.install_software(software)?;
        device.execute_program(software)?;
        println!("Installed programs: {:?}", device.get_installed_programs());
        Ok(())
    }
    
    // Combines programmable and network capabilities
    fn setup_networked_system(device: &mut (dyn Programmable + NetworkEnabled), network: &str) -> Result<(), String> {
        println!("Setting up networked system...");
        device.connect_to_network(network)?;
        device.install_software("Network Manager")?;
        device.send_data("System ready")?;
        println!("Network status: {}", device.get_network_status());
        Ok(())
    }
}

fn main() {
    println!("=== Interface Segregation Principle - CORRECT (Rust) ===");
    println!();
    
    println!("1. Creating entities with segregated traits:");
    let mut person = Person::new("Alice".to_string());
    let mut robot = IndustrialRobot::new("R2D2-Industrial".to_string());
    let mut humanoid = HumanoidRobot::new("ASIMO-Advanced".to_string());
    
    println!("\n2. Work management (only needs Workable trait):");
    WorkManager::assign_work(&mut person, "Write documentation");
    WorkManager::assign_work(&mut robot, "Assemble components");
    WorkManager::assign_work(&mut humanoid, "Assist customers");
    
    println!("\n3. Complex work (needs Workable + Cognitive):");
    WorkManager::assign_complex_work(&mut person, "Design new product");
    WorkManager::assign_complex_work(&mut robot, "Optimize assembly line");
    WorkManager::assign_complex_work(&mut humanoid, "Develop social protocols");
    
    println!("\n4. Biological care (only for entities with Biological trait):");
    // Only Person implements Biological - no errors!
    if let Ok(()) = BiologicalCareProvider::provide_care(&mut person) {
        println!("Biological care provided successfully");
    }
    
    println!("\n5. Technical support (only for entities with Mechanical trait):");
    // Only robots implement Mechanical
    if let Ok(()) = TechnicalSupportManager::perform_maintenance(&mut robot) {
        println!("Robot maintenance completed");
    }
    if let Ok(()) = TechnicalSupportManager::perform_maintenance(&mut humanoid) {
        println!("Humanoid maintenance completed");
    }
    
    println!("\n6. Electronic device servicing (needs Mechanical + Rechargeable):");
    if let Ok(()) = TechnicalSupportManager::service_electronic_device(&mut robot) {
        println!("Robot servicing completed");
    }
    if let Ok(()) = TechnicalSupportManager::service_electronic_device(&mut humanoid) {
        println!("Humanoid servicing completed");
    }
    
    println!("\n7. Social interaction (only for entities with Social trait):");
    SocialCoordinator::facilitate_interaction(&mut person);
    SocialCoordinator::facilitate_interaction(&mut humanoid);  // Humanoid has social capabilities
    
    println!("\n8. IT management (for programmable devices):");
    if let Ok(()) = ITManager::deploy_software(&mut robot, "Quality Control v3.0") {
        println!("Software deployed to robot");
    }
    if let Ok(()) = ITManager::deploy_software(&mut humanoid, "Advanced Social AI v2.1") {
        println!("Software deployed to humanoid");
    }
    
    println!("\n9. Network setup (for networked programmable devices):");
    if let Ok(()) = ITManager::setup_networked_system(&mut robot, "Factory Network") {
        println!("Robot network setup completed");
    }
    if let Ok(()) = ITManager::setup_networked_system(&mut humanoid, "Social Network") {
        println!("Humanoid network setup completed");
    }
    
    println!("\n10. Demonstrating trait specialization:");
    println!("Person traits: Workable, Biological, Cognitive, Social, Physical");
    println!("Robot traits: Workable, Cognitive, Mechanical, Rechargeable, Programmable, NetworkEnabled");
    println!("Humanoid traits: Workable, Cognitive, Social, Mechanical, Rechargeable, Programmable, NetworkEnabled");
    
    println!("\n=== Benefits of Segregated Traits ===");
    println!("✓ Each entity only implements traits it actually needs");
    println!("✓ Clients depend only on the methods they use");
    println!("✓ Easy to add new entities with different trait combinations");
    println!("✓ No empty/error implementations required");
    println!("✓ Compile-time safety ensures trait contracts are met");
    println!("✓ Clear separation of concerns");
    println!("✓ Follows Interface Segregation Principle perfectly");
}