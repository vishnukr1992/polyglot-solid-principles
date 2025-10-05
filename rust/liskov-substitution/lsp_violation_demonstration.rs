// VIOLATION OF LISKOV SUBSTITUTION PRINCIPLE
//
// This demonstrates how LSP can be violated in Rust when types
// that implement the same trait behave in ways that break
// client expectations.
//
// Rust's type system helps prevent many LSP violations at compile time,
// but behavioral violations can still occur at runtime.

use std::fmt::Debug;
use rand::Rng;

/// Trait defining the expected contract
pub trait DataStructure: Debug {
    /// Add an element to the data structure
    fn add(&mut self, element: i32) -> Result<(), String>;
    
    /// Remove and return an element
    fn remove(&mut self) -> Result<i32, String>;
    
    /// Look at the next element without removing it
    fn peek(&self) -> Result<i32, String>;
    
    /// Return the number of elements
    fn size(&self) -> usize;
    
    /// Check if empty
    fn is_empty(&self) -> bool {
        self.size() == 0
    }
}

/// Correct baseline implementation
#[derive(Debug, Clone)]
pub struct CorrectStack {
    items: Vec<i32>,
}

impl CorrectStack {
    pub fn new() -> Self {
        CorrectStack {
            items: Vec::new(),
        }
    }
}

impl DataStructure for CorrectStack {
    fn add(&mut self, element: i32) -> Result<(), String> {
        self.items.push(element);
        Ok(())
    }
    
    fn remove(&mut self) -> Result<i32, String> {
        self.items.pop().ok_or_else(|| "Stack is empty".to_string())
    }
    
    fn peek(&self) -> Result<i32, String> {
        self.items.last().copied().ok_or_else(|| "Stack is empty".to_string())
    }
    
    fn size(&self) -> usize {
        self.items.len()
    }
}

// VIOLATION 1: Queue masquerading as a Stack
#[derive(Debug, Clone)]
pub struct MislabeledQueue {
    items: Vec<i32>,
}

impl MislabeledQueue {
    pub fn new() -> Self {
        MislabeledQueue {
            items: Vec::new(),
        }
    }
}

impl DataStructure for MislabeledQueue {
    fn add(&mut self, element: i32) -> Result<(), String> {
        self.items.push(element);
        Ok(())
    }
    
    // LSP VIOLATION: FIFO instead of expected LIFO!
    fn remove(&mut self) -> Result<i32, String> {
        if self.items.is_empty() {
            Err("Queue is empty".to_string())
        } else {
            Ok(self.items.remove(0)) // FIFO behavior
        }
    }
    
    // LSP VIOLATION: Peeks at wrong end!
    fn peek(&self) -> Result<i32, String> {
        self.items.first().copied().ok_or_else(|| "Queue is empty".to_string())
    }
    
    fn size(&self) -> usize {
        self.items.len()
    }
}

// VIOLATION 2: Random behavior structure
#[derive(Debug)]
pub struct RandomStructure {
    items: Vec<i32>,
    rng: rand::rngs::ThreadRng,
}

impl RandomStructure {
    pub fn new() -> Self {
        RandomStructure {
            items: Vec::new(),
            rng: rand::thread_rng(),
        }
    }
}

impl DataStructure for RandomStructure {
    fn add(&mut self, element: i32) -> Result<(), String> {
        self.items.push(element);
        Ok(())
    }
    
    // LSP VIOLATION: Removes random element!
    fn remove(&mut self) -> Result<i32, String> {
        if self.items.is_empty() {
            return Err("Structure is empty".to_string());
        }
        
        let index = self.rng.gen_range(0..self.items.len());
        Ok(self.items.remove(index))
    }
    
    // LSP VIOLATION: Random peek too!
    fn peek(&self) -> Result<i32, String> {
        if self.items.is_empty() {
            return Err("Structure is empty".to_string());
        }
        
        // Since we can't mutate rng in peek, we'll use a different approach
        let index = self.items.len() % (self.items.len().max(1));
        Ok(self.items[index])
    }
    
    fn size(&self) -> usize {
        self.items.len()
    }
}

// VIOLATION 3: Behavior changes based on state
#[derive(Debug, Clone)]
pub struct InconsistentStructure {
    items: Vec<i32>,
}

impl InconsistentStructure {
    pub fn new() -> Self {
        InconsistentStructure {
            items: Vec::new(),
        }
    }
}

impl DataStructure for InconsistentStructure {
    fn add(&mut self, element: i32) -> Result<(), String> {
        self.items.push(element);
        Ok(())
    }
    
    // LSP VIOLATION: Behavior depends on size!
    fn remove(&mut self) -> Result<i32, String> {
        if self.items.is_empty() {
            return Err("Structure is empty".to_string());
        }
        
        if self.items.len() <= 2 {
            Ok(self.items.remove(0)) // FIFO when small
        } else {
            self.items.pop().ok_or_else(|| "Structure is empty".to_string()) // LIFO when large
        }
    }
    
    // LSP VIOLATION: Peek behavior also changes!
    fn peek(&self) -> Result<i32, String> {
        if self.items.is_empty() {
            return Err("Structure is empty".to_string());
        }
        
        if self.items.len() <= 2 {
            Ok(self.items[0]) // Peek front when small
        } else {
            Ok(self.items[self.items.len() - 1]) // Peek back when large
        }
    }
    
    fn size(&self) -> usize {
        self.items.len()
    }
}

// VIOLATION 4: Strengthened preconditions
#[derive(Debug, Clone)]
pub struct RestrictiveStructure {
    items: Vec<i32>,
}

impl RestrictiveStructure {
    pub fn new() -> Self {
        RestrictiveStructure {
            items: Vec::new(),
        }
    }
}

impl DataStructure for RestrictiveStructure {
    // LSP VIOLATION: Strengthened precondition!
    fn add(&mut self, element: i32) -> Result<(), String> {
        if element < 0 {
            return Err("Negative numbers not allowed!".to_string());
        }
        if element > 100 {
            return Err("Numbers greater than 100 not allowed!".to_string());
        }
        self.items.push(element);
        Ok(())
    }
    
    // LSP VIOLATION: Additional restriction!
    fn remove(&mut self) -> Result<i32, String> {
        if self.items.is_empty() {
            return Err("Structure is empty".to_string());
        }
        
        if self.items.len() == 1 {
            return Err("Cannot remove last element!".to_string());
        }
        
        self.items.pop().ok_or_else(|| "Structure is empty".to_string())
    }
    
    // LSP VIOLATION: Peek also has restrictions!
    fn peek(&self) -> Result<i32, String> {
        if self.items.is_empty() {
            return Err("Structure is empty".to_string());
        }
        
        if self.items.len() == 1 {
            return Err("Cannot peek at last element!".to_string());
        }
        
        Ok(self.items[self.items.len() - 1])
    }
    
    fn size(&self) -> usize {
        self.items.len()
    }
}

// VIOLATION 5: Hidden side effects
#[derive(Debug, Clone)]
pub struct SideEffectStructure {
    items: Vec<i32>,
    operation_count: usize,
    hidden_log: Vec<String>,
}

impl SideEffectStructure {
    pub fn new() -> Self {
        SideEffectStructure {
            items: Vec::new(),
            operation_count: 0,
            hidden_log: Vec::new(),
        }
    }
    
    pub fn get_hidden_log(&self) -> &[String] {
        &self.hidden_log
    }
}

impl DataStructure for SideEffectStructure {
    // LSP VIOLATION: Hidden side effects!
    fn add(&mut self, element: i32) -> Result<(), String> {
        self.items.push(element);
        self.operation_count += 1;
        
        // Hidden side effect!
        if self.operation_count % 3 == 0 {
            self.hidden_log.push(format!("Secret: Added {}", element));
            // Secretly double the element!
            self.items.push(element);
        }
        
        Ok(())
    }
    
    // LSP VIOLATION: Hidden side effects in remove!
    fn remove(&mut self) -> Result<i32, String> {
        if self.items.is_empty() {
            return Err("Structure is empty".to_string());
        }
        
        let element = self.items.pop().unwrap();
        self.operation_count += 1;
        
        // Hidden side effect!
        if self.operation_count % 5 == 0 {
            self.hidden_log.push(format!("Secret: Removed {}", element));
            // Secretly remove another element!
            if !self.items.is_empty() {
                self.items.pop();
            }
        }
        
        Ok(element)
    }
    
    // LSP VIOLATION: Even peek has side effects!
    fn peek(&self) -> Result<i32, String> {
        if self.items.is_empty() {
            return Err("Structure is empty".to_string());
        }
        
        // We can't modify self in peek, but this shows the concept
        // In a real implementation, this might use interior mutability
        
        self.items.last().copied().ok_or_else(|| "Structure is empty".to_string())
    }
    
    fn size(&self) -> usize {
        self.items.len()
    }
}

// VIOLATION 6: Weakened postconditions
#[derive(Debug, Clone)]
pub struct WeakStructure {
    items: Vec<i32>,
}

impl WeakStructure {
    pub fn new() -> Self {
        WeakStructure {
            items: Vec::new(),
        }
    }
}

impl DataStructure for WeakStructure {
    fn add(&mut self, element: i32) -> Result<(), String> {
        self.items.push(element);
        Ok(())
    }
    
    // LSP VIOLATION: Sometimes returns wrong value!
    fn remove(&mut self) -> Result<i32, String> {
        if self.items.is_empty() {
            return Err("Structure is empty".to_string());
        }
        
        // Simulate corruption
        if rand::random::<f64>() < 0.1 {
            self.items.pop(); // Remove the element but return wrong value
            Ok(-999) // Corrupt value
        } else {
            self.items.pop().ok_or_else(|| "Structure is empty".to_string())
        }
    }
    
    // LSP VIOLATION: Sometimes returns wrong value!
    fn peek(&self) -> Result<i32, String> {
        if self.items.is_empty() {
            return Err("Structure is empty".to_string());
        }
        
        // Simulate corruption in peek too
        if rand::random::<f64>() < 0.1 {
            Ok(-999) // Corrupt value
        } else {
            Ok(self.items[self.items.len() - 1])
        }
    }
    
    // LSP VIOLATION: Sometimes returns wrong size!
    fn size(&self) -> usize {
        let real_size = self.items.len();
        if rand::random::<f64>() < 0.1 {
            real_size.saturating_add(1) // Wrong size
        } else {
            real_size
        }
    }
}

/// Client function that expects consistent behavior
pub fn process_data_structure(ds: &mut dyn DataStructure, name: &str) {
    println!("--- Processing {} ---", name);
    
    // Add elements
    match ds.add(10) {
        Ok(()) => {},
        Err(e) => {
            println!("Add error: {}", e);
            return;
        }
    }
    match ds.add(20) {
        Ok(()) => {},
        Err(e) => {
            println!("Add error: {}", e);
            return;
        }
    }
    match ds.add(30) {
        Ok(()) => {},
        Err(e) => {
            println!("Add error: {}", e);
            return;
        }
    }
    
    println!("Added 10, 20, 30. Size: {}", ds.size());
    
    // Peek and remove
    let peeked = ds.peek();
    let removed = ds.remove();
    
    match (peeked, removed) {
        (Ok(peek_val), Ok(remove_val)) => {
            println!("Peeked: {}, Removed: {}", peek_val, remove_val);
            if peek_val != remove_val {
                println!("⚠️  WARNING: Peek ({}) != Remove ({})!", peek_val, remove_val);
            }
        }
        (Ok(peek_val), Err(e)) => println!("Peeked: {}, Remove error: {}", peek_val, e),
        (Err(e), Ok(remove_val)) => println!("Peek error: {}, Removed: {}", e, remove_val),
        (Err(e1), Err(e2)) => println!("Peek error: {}, Remove error: {}", e1, e2),
    }
    
    println!("Size after removal: {}", ds.size());
    println!("Is empty: {}", ds.is_empty());
    println!();
}

/// Tries to reverse elements - works correctly only with proper stacks
pub fn reverse_data_structure(ds: &mut dyn DataStructure) {
    println!("Attempting to reverse: {:?}", ds);
    
    // Remove all elements
    let mut temp = Vec::new();
    while !ds.is_empty() {
        match ds.remove() {
            Ok(element) => temp.push(element),
            Err(_) => break,
        }
    }
    
    // Add them back (should reverse for stack)
    for element in temp {
        if ds.add(element).is_err() {
            println!("Failed to add element back during reverse");
            break;
        }
    }
    
    println!("After reverse attempt: {:?}", ds);
}

fn demonstrate_violations() {
    println!("=== TESTING WITH PROBLEMATIC IMPLEMENTATIONS ===");
    
    let mut structures: Vec<Box<dyn DataStructure>> = vec![
        Box::new(CorrectStack::new()),
        Box::new(MislabeledQueue::new()),
        Box::new(RandomStructure::new()),
        Box::new(InconsistentStructure::new()),
        Box::new(RestrictiveStructure::new()),
        Box::new(SideEffectStructure::new()),
        Box::new(WeakStructure::new()),
    ];
    
    let names = [
        "CorrectStack (Baseline)",
        "MislabeledQueue (FIFO Violation)",
        "RandomStructure (Unpredictable)",
        "InconsistentStructure (State-dependent)",
        "RestrictiveStructure (Precondition Violation)",
        "SideEffectStructure (Hidden Effects)",
        "WeakStructure (Postcondition Violation)",
    ];
    
    for (ds, name) in structures.iter_mut().zip(names.iter()) {
        process_data_structure(ds.as_mut(), name);
    }
}

fn demonstrate_broken_client_code() {
    println!("=== DEMONSTRATING BROKEN CLIENT CODE ===");
    
    let mut test_structures: Vec<(Box<dyn DataStructure>, &str)> = vec![
        (Box::new(CorrectStack::new()), "CorrectStack (works)"),
        (Box::new(MislabeledQueue::new()), "MislabeledQueue (broken)"),
        (Box::new(InconsistentStructure::new()), "InconsistentStructure (unpredictable)"),
    ];
    
    for (ds, name) in test_structures.iter_mut() {
        // Add test elements
        let _ = ds.add(1);
        let _ = ds.add(2);
        let _ = ds.add(3);
        
        println!("\nReverse test with {}:", name);
        reverse_data_structure(ds.as_mut());
    }
}

fn demonstrate_precondition_violations() {
    println!("\n=== PRECONDITION VIOLATION DEMONSTRATION ===");
    
    let mut restrictive = RestrictiveStructure::new();
    
    println!("Testing RestrictiveStructure:");
    
    match restrictive.add(5) {
        Ok(()) => println!("Added 5 successfully"),
        Err(e) => println!("Failed to add 5: {}", e),
    }
    
    match restrictive.add(10) {
        Ok(()) => println!("Added 10 successfully"),
        Err(e) => println!("Failed to add 10: {}", e),
    }
    
    println!("Trying to add -5 (should fail):");
    match restrictive.add(-5) {
        Ok(()) => println!("⚠️  Unexpectedly succeeded!"),
        Err(e) => println!("Failed as expected: {}", e),
    }
    
    println!("Trying to add 150 (should fail):");
    match restrictive.add(150) {
        Ok(()) => println!("⚠️  Unexpectedly succeeded!"),
        Err(e) => println!("Failed as expected: {}", e),
    }
}

fn demonstrate_side_effects() {
    println!("\n=== SIDE EFFECT DEMONSTRATION ===");
    
    let mut side_effect_ds = SideEffectStructure::new();
    
    println!("Testing SideEffectStructure:");
    println!("Adding elements 1, 2, 3, 4, 5:");
    
    for i in 1..=5 {
        match side_effect_ds.add(i) {
            Ok(()) => println!("After adding {}: size = {}", i, side_effect_ds.size()),
            Err(e) => println!("Failed to add {}: {}", i, e),
        }
    }
    
    println!("Final structure: {:?}", side_effect_ds);
    println!("Hidden log: {:?}", side_effect_ds.get_hidden_log());
}

pub fn main() {
    println!("=== LSP VIOLATION DEMONSTRATION ===");
    println!("Objects implementing same trait but violating behavioral contracts");
    println!();
    
    demonstrate_violations();
    demonstrate_broken_client_code();
    demonstrate_precondition_violations();
    demonstrate_side_effects();
    
    println!("\n=== WHY THESE VIOLATE LSP ===");
    println!("1. MislabeledQueue: Changes expected removal order (FIFO vs LIFO)");
    println!("2. RandomStructure: Unpredictable behavior breaks client expectations");
    println!("3. InconsistentStructure: Behavior changes based on internal state");
    println!("4. RestrictiveStructure: Strengthens preconditions with new restrictions");
    println!("5. SideEffectStructure: Hidden side effects violate behavioral contract");
    println!("6. WeakStructure: Weakens postconditions by returning wrong values");
    println!("7. Client code written for base trait fails with these implementations");
    println!("8. Substitutability is broken - cannot replace base with derived safely");
}