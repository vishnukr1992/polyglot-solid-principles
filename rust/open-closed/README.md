# Open-Closed Principle (OCP) - Rust Examples

The **Open-Closed Principle** states that software entities should be **open for extension but closed for modification**.

## Rust Implementation Approach

Rust uses **traits**, **trait objects**, and **generics** to implement OCP while maintaining memory safety and zero-cost abstractions.

## File Structure

### ✅ Correct Implementation (Follows OCP)

- **`transistor.rs`**: Contains trait definition and all correct implementations

```rust
/// Trait defining the interface for all transistor types
pub trait Transistor: Debug {
    fn base(&mut self, signal: f64);
    fn collector(&mut self, input: f64);
    fn output(&self) -> f64;
}
```

**Key Rust Features Used:**
- **Traits**: Define shared behavior (similar to interfaces)
- **Trait objects**: `Box<dyn Transistor>` for runtime polymorphism
- **Generics**: Compile-time polymorphism with zero cost
- **Memory safety**: Ownership system prevents memory errors

**Benefits:**
```rust
// Adding new transistor types requires no changes to existing code
#[derive(Debug)]
pub struct JFETTransistor {
    gate_voltage: f64,
    drain_current: f64,
}

impl Transistor for JFETTransistor {
    fn base(&mut self, signal: f64) {
        self.gate_voltage = signal;
    }
    
    fn collector(&mut self, input: f64) {
        self.drain_current = input;
    }
    
    fn output(&self) -> f64 {
        // JFET-specific logic
        self.drain_current * (self.gate_voltage * 0.12)
    }
}

// Amplifier works with any Transistor implementation
pub fn amplify_signal(&mut self, signal: f64, input: f64) -> f64 {
    for transistor in &mut self.transistors {
        transistor.base(signal);
        transistor.collector(input);
        total_output += transistor.output();
    }
}
```

### ❌ Violation Example

- **`transistor_ocp_violation.rs`**: Shows how NOT to implement OCP

**Problems:**
```rust
// This match statement violates OCP
match transistor.transistor_type {
    TransistorType::BJT => {
        // BJT logic
        transistor.process_signal(signal, input)
    }
    TransistorType::FET => {
        // FET logic - requires modifying this method
        transistor.process_signal(signal, input) * 1.1
    }
    // Adding new types requires more match arms here
}
```

## Rust-Specific OCP Features

### 1. Static vs Dynamic Dispatch

```rust
// Static dispatch (compile-time) - zero cost
fn amplify_static<T: Transistor>(transistor: &mut T, signal: f64) -> f64 {
    transistor.base(signal);
    transistor.output()
}

// Dynamic dispatch (runtime) - flexible but small cost
fn amplify_dynamic(transistor: &mut dyn Transistor, signal: f64) -> f64 {
    transistor.base(signal);
    transistor.output()
}
```

### 2. Generic Collections

```rust
// Homogeneous collection with static dispatch
struct StaticAmplifier<T: Transistor> {
    transistors: Vec<T>,
}

// Heterogeneous collection with dynamic dispatch
struct DynamicAmplifier {
    transistors: Vec<Box<dyn Transistor>>,
}
```

### 3. Associated Types

```rust
trait Transistor {
    type Output;
    type Config;
    
    fn configure(&mut self, config: Self::Config);
    fn process(&self, signal: f64) -> Self::Output;
}

impl Transistor for BJTTransistor {
    type Output = f64;
    type Config = BJTConfig;
    
    fn configure(&mut self, config: Self::Config) {
        // BJT-specific configuration
    }
}
```

### 4. Default Implementations

```rust
trait Transistor {
    fn base(&mut self, signal: f64);
    fn collector(&mut self, input: f64);
    fn output(&self) -> f64;
    
    // Default implementation that can be overridden
    fn amplify(&self, signal: f64) -> f64 {
        self.output() * signal
    }
    
    // Default implementation using other trait methods
    fn process_with_gain(&mut self, signal: f64, input: f64, gain: f64) -> f64 {
        self.base(signal);
        self.collector(input);
        self.output() * gain
    }
}
```

## Running the Examples

```bash
# Run correct implementation
cargo run --bin transistor

# Run violation example
cargo run --bin transistor_ocp_violation

# Run with optimizations
cargo run --release --bin transistor

# Check code
cargo check
cargo clippy
```

## Advanced Rust OCP Patterns

### 1. Trait Objects with Lifetimes

```rust
struct AmplifierCircuit<'a> {
    transistors: Vec<&'a mut dyn Transistor>,
}

impl<'a> AmplifierCircuit<'a> {
    fn add_transistor(&mut self, transistor: &'a mut dyn Transistor) {
        self.transistors.push(transistor);
    }
}
```

### 2. Enum Dispatch (Type-Safe Alternative)

```rust
#[derive(Debug)]
enum TransistorEnum {
    BJT(BJTTransistor),
    FET(FETTransistor),
    MOSFET(MOSFETTransistor),
}

impl Transistor for TransistorEnum {
    fn base(&mut self, signal: f64) {
        match self {
            TransistorEnum::BJT(bjt) => bjt.base(signal),
            TransistorEnum::FET(fet) => fet.base(signal),
            TransistorEnum::MOSFET(mosfet) => mosfet.base(signal),
        }
    }
    
    // This approach trades OCP compliance for performance and type safety
    // New types require match arm updates, but it's compile-time verified
}
```

### 3. Plugin System with Dynamic Loading

```rust
use libloading::{Library, Symbol};

trait TransistorPlugin {
    fn create_transistor(&self) -> Box<dyn Transistor>;
}

struct PluginManager {
    libraries: Vec<Library>,
}

impl PluginManager {
    fn load_plugin(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            let lib = Library::new(path)?;
            let create_plugin: Symbol<fn() -> Box<dyn TransistorPlugin>> = 
                lib.get(b"create_plugin")?;
            let plugin = create_plugin();
            self.libraries.push(lib);
            Ok(())
        }
    }
}
```

### 4. Const Generics for Compile-Time Configuration

```rust
trait Transistor<const CHANNELS: usize> {
    fn base(&mut self, signals: [f64; CHANNELS]);
    fn output(&self) -> [f64; CHANNELS];
}

#[derive(Debug)]
struct MultiChannelBJT<const N: usize> {
    channels: [f64; N],
}

impl<const N: usize> Transistor<N> for MultiChannelBJT<N> {
    fn base(&mut self, signals: [f64; N]) {
        self.channels = signals;
    }
    
    fn output(&self) -> [f64; N] {
        self.channels.map(|ch| ch * 0.1)
    }
}
```

### 5. Async Traits

```rust
use async_trait::async_trait;

#[async_trait]
trait AsyncTransistor {
    async fn initialize(&mut self) -> Result<(), String>;
    async fn process_signal(&self, signal: f64) -> f64;
}

#[async_trait]
impl AsyncTransistor for BJTTransistor {
    async fn initialize(&mut self) -> Result<(), String> {
        // Async initialization (e.g., hardware calibration)
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        Ok(())
    }
    
    async fn process_signal(&self, signal: f64) -> f64 {
        // Async signal processing
        self.output()
    }
}
```

## Real-world Rust Applications

- **Serde**: Serialization trait with many format implementations
- **Tokio**: Runtime trait for different async executors
- **Bevy ECS**: Component and System traits for game engine
- **Diesel**: Database backend traits for different SQL databases
- **Warp/Axum**: HTTP service traits for web frameworks
- **embedded-hal**: Hardware abstraction layer for embedded systems

Rust's trait system provides powerful abstractions while maintaining zero-cost principles, making it excellent for implementing OCP in performance-critical applications.