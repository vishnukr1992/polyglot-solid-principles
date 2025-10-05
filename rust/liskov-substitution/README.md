# Liskov Substitution Principle (LSP) - Rust Examples

The **Liskov Substitution Principle** states that objects of a superclass should be replaceable with objects of a subclass without altering the correctness of the program.

## LSP in Rust

Rust's **trait system** provides excellent LSP support through:
- **Compile-time guarantees**: Trait bounds verified at compile time
- **No inheritance**: Composition over inheritance prevents classical LSP violations
- **Explicit contracts**: Traits define clear behavioral contracts
- **Memory safety**: Ownership system prevents many runtime violations
- **Zero-cost abstractions**: LSP compliance with no runtime overhead

## Rust's Trait Advantage

```rust
// Trait defines the behavioral contract
trait DataStructure {
    fn add(&mut self, element: i32);
    fn remove(&mut self) -> Result<i32, String>;
    fn size(&self) -> usize;
}

// Any type implementing the trait satisfies the contract
impl DataStructure for Stack {
    fn remove(&mut self) -> Result<i32, String> {
        self.items.pop().ok_or_else(|| "Stack is empty".to_string())
    }
}

// Client code works with any implementation
fn process_data_structure(ds: &mut dyn DataStructure) {
    ds.add(42);
    match ds.remove() {
        Ok(value) => println!("Removed: {}", value),
        Err(e) => println!("Error: {}", e),
    }
}
```

## File Structure

### ✅ Correct Implementation

- **`lsp_correct_demonstration.rs`**: Shows proper LSP compliance with Rust traits

**Key features:**
- **Trait contracts**: Clear behavioral specifications with Result types
- **Multiple implementations**: Stack, Queue, PriorityQueue, Deque
- **Consistent error handling**: All use Result<T, String> pattern
- **Generic functions**: Work with any type implementing the trait
- **Trait objects**: Dynamic dispatch when needed

```rust
pub trait DataStructure: Debug {
    fn add(&mut self, element: i32);
    fn remove(&mut self) -> Result<i32, String>;
    fn peek(&self) -> Result<i32, String>;
    fn size(&self) -> usize;
    
    // Default implementation
    fn is_empty(&self) -> bool {
        self.size() == 0
    }
}

// All implementations honor the contract
impl DataStructure for Stack {
    fn remove(&mut self) -> Result<i32, String> {
        self.items.pop().ok_or_else(|| "Stack is empty".to_string())
    }
    
    fn peek(&self) -> Result<i32, String> {
        self.items.last().copied().ok_or_else(|| "Stack is empty".to_string())
    }
}
```

### ❌ Violation Examples

- **`lsp_violation_demonstration.rs`**: Shows various LSP violations in Rust

**Violations demonstrated:**
1. **Behavioral changes**: Queue with stack interface (FIFO vs LIFO)
2. **Random behavior**: Unpredictable element removal
3. **State-dependent behavior**: Behavior changes based on size
4. **Strengthened preconditions**: Additional input restrictions
5. **Hidden side effects**: Unexpected operations during method calls
6. **Weakened postconditions**: Wrong return values

```rust
// LSP Violation: Queue pretending to be stack
impl DataStructure for MislabeledQueue {
    fn remove(&mut self) -> Result<i32, String> {
        if self.items.is_empty() {
            Err("Queue is empty".to_string())
        } else {
            Ok(self.items.remove(0)) // FIFO instead of LIFO!
        }
    }
    
    fn peek(&self) -> Result<i32, String> {
        // VIOLATION: Peeks at wrong end!
        self.items.first().copied().ok_or_else(|| "Queue is empty".to_string())
    }
}
```

## Rust-Specific LSP Features

### 1. Trait Bounds
```rust
// Function works with any type implementing DataStructure
fn process_generic<T: DataStructure>(ds: &mut T) {
    ds.add(42);
    match ds.remove() {
        Ok(val) => println!("Removed: {}", val),
        Err(e) => println!("Error: {}", e),
    }
}

// Multiple trait bounds
fn process_with_debug<T: DataStructure + Debug>(ds: &mut T) {
    println!("Processing: {:?}", ds);
    ds.add(42);
}
```

### 2. Trait Objects (Dynamic Dispatch)
```rust
// Heterogeneous collections
let mut structures: Vec<Box<dyn DataStructure>> = vec![
    Box::new(Stack::new()),
    Box::new(Queue::new()),
    Box::new(PriorityQueue::new()),
];

// All can be processed uniformly
for ds in structures.iter_mut() {
    ds.add(42);
}
```

### 3. Associated Types
```rust
trait Iterator {
    type Item;
    
    fn next(&mut self) -> Option<Self::Item>;
}

// Implementation specifies the associated type
impl Iterator for Counter {
    type Item = usize;
    
    fn next(&mut self) -> Option<Self::Item> {
        // Implementation
    }
}
```

### 4. Default Implementations
```rust
trait DataStructure {
    fn add(&mut self, element: i32);
    fn remove(&mut self) -> Result<i32, String>;
    fn size(&self) -> usize;
    
    // Default implementations
    fn is_empty(&self) -> bool {
        self.size() == 0
    }
    
    fn add_multiple(&mut self, elements: &[i32]) {
        for &element in elements {
            self.add(element);
        }
    }
}
```

### 5. Const Generics
```rust
// Compile-time parameterization
trait FixedSizeStack<const N: usize> {
    fn push(&mut self, element: i32) -> Result<(), String>;
    fn pop(&mut self) -> Result<i32, String>;
    fn capacity(&self) -> usize { N }
}

struct ArrayStack<const N: usize> {
    items: [Option<i32>; N],
    len: usize,
}
```

### 6. Async Traits
```rust
use async_trait::async_trait;

#[async_trait]
trait AsyncDataStructure {
    async fn add(&mut self, element: i32) -> Result<(), String>;
    async fn remove(&mut self) -> Result<i32, String>;
}

#[async_trait]
impl AsyncDataStructure for AsyncStack {
    async fn add(&mut self, element: i32) -> Result<(), String> {
        // Async implementation
        tokio::time::sleep(Duration::from_millis(1)).await;
        self.items.push(element);
        Ok(())
    }
}
```

## Common Rust LSP Violations

### 1. Panic Instead of Result
```rust
// VIOLATION: Panics instead of returning Result
impl DataStructure for BadStack {
    fn remove(&mut self) -> Result<i32, String> {
        if self.items.is_empty() {
            panic!("Stack is empty"); // Should return Err!
        }
        Ok(self.items.pop().unwrap())
    }
}
```

### 2. Inconsistent Error Types
```rust
// VIOLATION: Different error types break contract
impl DataStructure for InconsistentStack {
    fn remove(&mut self) -> Result<i32, String> {
        self.items.pop().ok_or_else(|| {
            format!("Stack empty at {}", std::file!()) // Different error format
        })
    }
}
```

### 3. Hidden Side Effects
```rust
// VIOLATION: Unexpected side effects
impl DataStructure for SideEffectStack {
    fn remove(&mut self) -> Result<i32, String> {
        let result = self.items.pop().ok_or_else(|| "Empty".to_string());
        
        // VIOLATION: Hidden logging side effect
        if self.operation_count % 3 == 0 {
            println!("Secret log: removing element");
        }
        
        result
    }
}
```

## Running the Examples

```bash
# Add to Cargo.toml for violation examples
[dependencies]
rand = "0.8"

# Run correct implementation
cargo run --bin lsp_correct_demonstration

# Run violation examples
cargo run --bin lsp_violation_demonstration

# Run with optimizations
cargo run --release --bin lsp_correct_demonstration

# Check for issues
cargo check
cargo clippy
```

## Advanced Rust LSP Patterns

### 1. Enum Dispatch (Type-Safe Alternative)
```rust
// Sometimes LSP can be traded for performance and exhaustiveness
#[derive(Debug)]
enum DataStructureEnum {
    Stack(Stack),
    Queue(Queue),
    PriorityQueue(PriorityQueue),
}

impl DataStructure for DataStructureEnum {
    fn add(&mut self, element: i32) {
        match self {
            DataStructureEnum::Stack(s) => s.add(element),
            DataStructureEnum::Queue(q) => q.add(element),
            DataStructureEnum::PriorityQueue(pq) => pq.add(element),
        }
    }
    
    // Compiler ensures all variants are handled
    fn remove(&mut self) -> Result<i32, String> {
        match self {
            DataStructureEnum::Stack(s) => s.remove(),
            DataStructureEnum::Queue(q) => q.remove(),
            DataStructureEnum::PriorityQueue(pq) => pq.remove(),
        }
    }
}
```

### 2. Generic Associated Types (GATs)
```rust
// Advanced trait with generic associated types
trait DataStructureGAT {
    type Item<T>;
    type Error;
    
    fn add<T>(&mut self, element: T) -> Result<(), Self::Error>;
    fn remove<T>(&mut self) -> Result<Self::Item<T>, Self::Error>;
}
```

### 3. Higher-Ranked Trait Bounds
```rust
// Function that works with any lifetime
fn process_with_lifetime<F>(f: F) 
where
    F: for<'a> Fn(&'a mut dyn DataStructure) -> Result<i32, String>,
{
    let mut stack = Stack::new();
    let result = f(&mut stack);
    println!("Result: {:?}", result);
}
```

## Testing LSP Compliance

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    // Generic test for any DataStructure implementation
    fn test_data_structure_contract<T: DataStructure>(mut ds: T) {
        // Test basic operations
        ds.add(1);
        ds.add(2);
        assert_eq!(ds.size(), 2);
        
        // Test remove operation
        let element = ds.remove().expect("Should not be empty");
        assert_eq!(ds.size(), 1);
        
        // Test empty state
        ds.remove().expect("Should not be empty");
        assert!(ds.is_empty());
        
        // Test error handling
        assert!(ds.remove().is_err());
    }
    
    #[test]
    fn test_all_implementations() {
        test_data_structure_contract(Stack::new());
        test_data_structure_contract(Queue::new());
        test_data_structure_contract(PriorityQueue::new());
    }
    
    // Property-based testing with quickcheck
    #[quickcheck]
    fn prop_add_remove_size(elements: Vec<i32>) -> bool {
        let mut stack = Stack::new();
        
        for element in &elements {
            stack.add(*element);
        }
        
        assert_eq!(stack.size(), elements.len());
        
        for _ in 0..elements.len() {
            stack.remove().is_ok()
        }
    }
}
```

## Real-world Rust Examples

### ✅ Excellent LSP Examples
- **Iterator trait**: Consistent interface across all iterator types
- **Read/Write traits**: File, network, buffer types implement same interface
- **Serde traits**: Serialize/Deserialize work with any format
- **Error trait**: All error types implement same interface
- **Future trait**: All async types implement same interface

### ⚠️ Potential Issues
- **Custom Drop implementations**: Can introduce surprising behavior
- **Panic in trait methods**: Violates error handling contracts
- **Platform-specific behavior**: Different behavior on different systems

## Best Practices

1. **Use Result types** for fallible operations
2. **Avoid panics** in trait implementations
3. **Keep traits focused** and cohesive
4. **Provide default implementations** where sensible
5. **Use associated types** for type relationships
6. **Document behavioral contracts** thoroughly
7. **Test trait implementations** generically
8. **Consider enum dispatch** for closed sets of types

```rust
// Good: Clear contract with Result type
trait DataStructure: Debug {
    /// Add an element to the data structure.
    /// 
    /// # Arguments
    /// * `element` - The element to add
    /// 
    /// # Returns
    /// `Ok(())` on success, `Err(String)` if operation fails
    fn add(&mut self, element: i32) -> Result<(), String>;
    
    /// Remove an element from the data structure.
    /// 
    /// # Returns
    /// `Ok(element)` if successful, `Err(String)` if empty
    /// 
    /// # Behavior
    /// The specific removal strategy (LIFO, FIFO, etc.) depends on
    /// the implementation type.
    fn remove(&mut self) -> Result<i32, String>;
}
```

Rust's trait system provides powerful compile-time guarantees for LSP compliance while maintaining zero-cost abstractions and memory safety.