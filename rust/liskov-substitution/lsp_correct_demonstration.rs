// CORRECT IMPLEMENTATION - FOLLOWS LISKOV SUBSTITUTION PRINCIPLE
//
// This demonstrates proper trait design where all implementations
// can be substituted without breaking the program's correctness.
//
// Rust's trait system naturally enforces behavioral contracts through
// compile-time guarantees and explicit error handling.

use std::fmt::Debug;

/// Trait defining the contract for data structures
pub trait DataStructure: Debug {
    /// Add an element to the data structure
    fn add(&mut self, element: i32);
    
    /// Remove and return an element from the data structure
    fn remove(&mut self) -> Result<i32, String>;
    
    /// Look at the next element without removing it
    fn peek(&self) -> Result<i32, String>;
    
    /// Return the number of elements
    fn size(&self) -> usize;
    
    /// Check if the data structure is empty
    fn is_empty(&self) -> bool {
        self.size() == 0
    }
}

/// Stack implementation (LIFO - Last In, First Out)
#[derive(Debug, Clone)]
pub struct Stack {
    items: Vec<i32>,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            items: Vec::new(),
        }
    }
}

impl DataStructure for Stack {
    fn add(&mut self, element: i32) {
        self.items.push(element);
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

/// Queue implementation (FIFO - First In, First Out)
#[derive(Debug, Clone)]
pub struct Queue {
    items: Vec<i32>,
}

impl Queue {
    pub fn new() -> Self {
        Queue {
            items: Vec::new(),
        }
    }
}

impl DataStructure for Queue {
    fn add(&mut self, element: i32) {
        self.items.push(element);
    }
    
    fn remove(&mut self) -> Result<i32, String> {
        if self.items.is_empty() {
            Err("Queue is empty".to_string())
        } else {
            Ok(self.items.remove(0)) // Remove from front (FIFO)
        }
    }
    
    fn peek(&self) -> Result<i32, String> {
        self.items.first().copied().ok_or_else(|| "Queue is empty".to_string())
    }
    
    fn size(&self) -> usize {
        self.items.len()
    }
}

/// Priority Queue implementation (highest value first)
#[derive(Debug, Clone)]
pub struct PriorityQueue {
    items: Vec<i32>,
}

impl PriorityQueue {
    pub fn new() -> Self {
        PriorityQueue {
            items: Vec::new(),
        }
    }
}

impl DataStructure for PriorityQueue {
    fn add(&mut self, element: i32) {
        self.items.push(element);
        self.items.sort(); // Keep sorted for priority
    }
    
    fn remove(&mut self) -> Result<i32, String> {
        self.items.pop().ok_or_else(|| "Priority queue is empty".to_string())
    }
    
    fn peek(&self) -> Result<i32, String> {
        self.items.last().copied().ok_or_else(|| "Priority queue is empty".to_string())
    }
    
    fn size(&self) -> usize {
        self.items.len()
    }
}

/// Deque implementation that can operate in different modes
#[derive(Debug, Clone)]
pub struct Deque {
    items: Vec<i32>,
    mode: DequeMode,
}

#[derive(Debug, Clone, Copy)]
pub enum DequeMode {
    Front, // Remove from front
    Back,  // Remove from back
}

impl Deque {
    pub fn new(mode: DequeMode) -> Self {
        Deque {
            items: Vec::new(),
            mode,
        }
    }
}

impl DataStructure for Deque {
    fn add(&mut self, element: i32) {
        self.items.push(element); // Always add to back
    }
    
    fn remove(&mut self) -> Result<i32, String> {
        if self.items.is_empty() {
            return Err("Deque is empty".to_string());
        }
        
        match self.mode {
            DequeMode::Front => Ok(self.items.remove(0)),
            DequeMode::Back => self.items.pop().ok_or_else(|| "Deque is empty".to_string()),
        }
    }
    
    fn peek(&self) -> Result<i32, String> {
        if self.items.is_empty() {
            return Err("Deque is empty".to_string());
        }
        
        match self.mode {
            DequeMode::Front => Ok(self.items[0]),
            DequeMode::Back => Ok(self.items[self.items.len() - 1]),
        }
    }
    
    fn size(&self) -> usize {
        self.items.len()
    }
}

/// Client function that works with any DataStructure implementation
pub fn process_data_structure(ds: &mut dyn DataStructure, name: &str) {
    println!("--- Processing {} ---", name);
    
    // Add elements
    ds.add(10);
    ds.add(20);
    ds.add(30);
    println!("After adding 10, 20, 30 - Size: {}", ds.size());
    
    // Peek at next element
    match ds.peek() {
        Ok(element) => println!("Next element (peek): {}", element),
        Err(e) => println!("Peek error: {}", e),
    }
    
    // Remove elements
    match ds.remove() {
        Ok(element) => println!("Removed: {}", element),
        Err(e) => println!("Remove error: {}", e),
    }
    
    println!("Size after removal: {}", ds.size());
    println!("Is empty: {}", ds.is_empty());
    println!();
}

/// Transfer elements from source to target
pub fn transfer_elements(
    source: &mut dyn DataStructure,
    target: &mut dyn DataStructure,
    count: usize,
) -> Result<(), String> {
    for _ in 0..count {
        if source.is_empty() {
            break;
        }
        match source.remove() {
            Ok(element) => target.add(element),
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

/// Count elements by removing them (destructive)
pub fn count_elements(ds: &mut dyn DataStructure) -> usize {
    let mut count = 0;
    while !ds.is_empty() {
        if ds.remove().is_ok() {
            count += 1;
        } else {
            break;
        }
    }
    count
}

/// Sum all elements by removing them (destructive)
pub fn sum_elements(ds: &mut dyn DataStructure) -> i32 {
    let mut total = 0;
    while !ds.is_empty() {
        match ds.remove() {
            Ok(element) => total += element,
            Err(_) => break,
        }
    }
    total
}

/// Generic function that works with any data structure type
pub fn process_generic<T: DataStructure>(ds: &mut T, name: &str) {
    println!("--- Processing {} (Generic) ---", name);
    
    ds.add(100);
    ds.add(200);
    
    match ds.peek() {
        Ok(element) => println!("Peek: {}", element),
        Err(e) => println!("Peek error: {}", e),
    }
    
    println!("Size: {}", ds.size());
    println!();
}

fn demonstrate_polymorphism() {
    println!("=== POLYMORPHIC BEHAVIOR DEMONSTRATION ===");
    
    // Create different data structures
    let mut structures: Vec<Box<dyn DataStructure>> = vec![
        Box::new(Stack::new()),
        Box::new(Queue::new()),
        Box::new(PriorityQueue::new()),
        Box::new(Deque::new(DequeMode::Back)),
        Box::new(Deque::new(DequeMode::Front)),
    ];
    
    let names = [
        "Stack",
        "Queue",
        "PriorityQueue",
        "Deque(Back)",
        "Deque(Front)",
    ];
    
    // All can be treated the same way
    for (ds, name) in structures.iter_mut().zip(names.iter()) {
        ds.add(5);
        ds.add(1);
        ds.add(3);
        
        match ds.peek() {
            Ok(element) => println!("{} peek: {}", name, element),
            Err(e) => println!("{} peek error: {}", name, e),
        }
    }
    println!();
}

fn demonstrate_transfer() {
    println!("=== TRANSFER OPERATION DEMONSTRATION ===");
    
    let mut source = Stack::new();
    let mut target = Queue::new();
    
    // Populate source
    for i in 1..=5 {
        source.add(i);
    }
    
    println!("Before transfer - Source: {:?}, Target: {:?}", source, target);
    
    // Transfer 3 elements
    match transfer_elements(&mut source, &mut target, 3) {
        Ok(()) => println!("Transfer successful"),
        Err(e) => println!("Transfer error: {}", e),
    }
    
    println!("After transfer - Source: {:?}, Target: {:?}", source, target);
    println!();
}

fn demonstrate_processors() {
    println!("=== PROCESSOR DEMONSTRATION ===");
    
    // Create and populate different structures
    let mut stack = Stack::new();
    let mut queue = Queue::new();
    
    for i in [10, 20, 30, 40, 50] {
        stack.add(i);
        queue.add(i);
    }
    
    // Clone for different operations
    let mut stack_for_count = stack.clone();
    let mut queue_for_sum = queue.clone();
    
    let stack_count = count_elements(&mut stack_for_count);
    let queue_sum = sum_elements(&mut queue_for_sum);
    
    println!("Stack element count: {}", stack_count);
    println!("Queue element sum: {}", queue_sum);
    println!();
}

fn demonstrate_generic_functions() {
    println!("=== GENERIC FUNCTION DEMONSTRATION ===");
    
    let mut stack = Stack::new();
    let mut queue = Queue::new();
    let mut priority_queue = PriorityQueue::new();
    
    // Same function works with all types
    process_generic(&mut stack, "Stack");
    process_generic(&mut queue, "Queue");
    process_generic(&mut priority_queue, "PriorityQueue");
}

pub fn main() {
    println!("=== LSP CORRECT DEMONSTRATION ===");
    println!("All implementations can be substituted without breaking correctness");
    println!();
    
    // Test all implementations with the same client code
    let mut stack = Stack::new();
    let mut queue = Queue::new();
    let mut priority_queue = PriorityQueue::new();
    let mut deque_back = Deque::new(DequeMode::Back);
    let mut deque_front = Deque::new(DequeMode::Front);
    
    // All these calls work correctly because LSP is followed
    process_data_structure(&mut stack, "Stack (LIFO)");
    process_data_structure(&mut queue, "Queue (FIFO)");
    process_data_structure(&mut priority_queue, "Priority Queue (Highest First)");
    process_data_structure(&mut deque_back, "Deque (Back Mode)");
    process_data_structure(&mut deque_front, "Deque (Front Mode)");
    
    // Demonstrate polymorphic behavior
    demonstrate_polymorphism();
    
    // Demonstrate transfer between different types
    demonstrate_transfer();
    
    // Demonstrate processors working with any data structure
    demonstrate_processors();
    
    // Demonstrate generic functions
    demonstrate_generic_functions();
    
    println!("=== WHY THIS FOLLOWS LSP ===");
    println!("1. All implementations honor the DataStructure trait contract");
    println!("2. Client code works correctly with any implementation");
    println!("3. Behavioral substitutability is maintained");
    println!("4. Error handling is consistent across implementations");
    println!("5. Trait bounds ensure compile-time contract verification");
    println!("6. No unexpected panics or undefined behavior");
    println!("7. Generic functions work with any conforming type");
}