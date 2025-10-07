# Interface Segregation Principle (ISP) Examples

## Overview

The **Interface Segregation Principle** is the fourth principle of SOLID design principles. It states:

> **"Clients should not be forced to depend upon interfaces that they do not use."**

## The Problem

When an interface is too large and monolithic, it forces implementing classes to provide implementations for methods they don't need. This leads to:

- **Fat interfaces** with too many responsibilities
- **Forced implementations** of irrelevant methods
- **Empty or exception-throwing methods** in concrete classes
- **Tight coupling** between unrelated functionalities
- **Violation of Single Responsibility Principle**

## Our Example: Robot vs Human Interface

### The Scenario

We demonstrate ISP using a Robot/Human concept where:

- **Problem**: Robot is forced to implement a Human interface only because it has a `work()` method
- **Issue**: Other Human interface methods (`eat()`, `sleep()`, `breathe()`, `reproduce()`, etc.) are not helpful and not useful for robots
- **Solution**: Proper interface segregation with a Humanoid interface that extends Robot capabilities with necessary Human functions

### The Violation

In the violation examples, we have a monolithic `Human` interface that includes:

```
Interface Human {
    work()           // ✓ Useful for both humans and robots
    eat()            // ✗ Not applicable to robots
    sleep()          // ✗ Robots power down, don't sleep
    breathe()        // ✗ Robots don't breathe
    think()          // ✓ Different implementation for robots
    reproduce()      // ✗ Robots are manufactured
    exercise()       // ✗ Robots don't need physical exercise
    socialize()      // ✓ Some robots have social capabilities
    feel_emotions()  // ✗ Traditional robots don't have emotions
    dream()          // ✗ Robots don't dream
}
```

**Problems with this approach:**
- Robot is forced to implement methods it doesn't need
- Many methods must return errors or throw exceptions
- Clients cannot rely on all interface methods working
- Violates ISP by forcing dependency on unused methods

### The Solution

In the correct examples, we segregate the interface into smaller, focused interfaces:

- **Workable**: Core work capabilities
- **Biological**: Living being needs (eat, sleep, breathe)
- **Cognitive**: Thinking and learning abilities
- **Social**: Social interaction capabilities
- **Physical**: Physical/emotional human aspects
- **Mechanical**: Robot-specific operations
- **Rechargeable**: Power management for electronic devices
- **Programmable**: Software execution capabilities
- **NetworkEnabled**: Connectivity features

**Benefits of segregation:**
- Each entity implements only relevant interfaces
- No forced empty implementations
- Clients depend only on methods they actually use
- Easy to add new entity types with different interface combinations
- Clear separation of concerns

### Entity Implementations

1. **Person**: Implements Workable, Biological, Cognitive, Social, Physical
2. **IndustrialRobot**: Implements Workable, Cognitive, Mechanical, Rechargeable, Programmable, NetworkEnabled
3. **HumanoidRobot**: Implements Workable, Cognitive, Social, Mechanical, Rechargeable, Programmable, NetworkEnabled

The **HumanoidRobot** demonstrates how we can extend Robot concepts with additional human-like capabilities without forcing all robots to have these features.

## Language-Specific Implementations

### Java (`java/interface-segregation/`)
- **Files**: `ISPViolationDemonstration.java`, `ISPCorrectDemonstration.java`
- **Features**: Abstract interfaces, concrete classes, interface composition
- **Key Concepts**: Interface inheritance, multiple interface implementation, compile-time safety

### JavaScript (`javascript/interface-segregation/`)
- **Files**: `ispViolationDemonstration.js`, `ispCorrectDemonstration.js`
- **Features**: Duck typing, contract-based programming, runtime errors
- **Key Concepts**: Dynamic typing, method presence checking, flexible contracts

### Go (`go/interface-segregation/`)
- **Files**: `ispViolationDemonstration.go`, `ispCorrectDemonstration.go`
- **Features**: Interface composition, implicit satisfaction, type assertions
- **Key Concepts**: Structural typing, embedded interfaces, composition over inheritance

### Python (`python/interface-segregation/`)
- **Files**: `isp_violation_demonstration.py`, `isp_correct_demonstration.py`
- **Features**: Abstract Base Classes (ABC), Protocols, type hints
- **Key Concepts**: Multiple inheritance, protocol-based programming, runtime type checking

### Rust (`rust/interface-segregation/`)
- **Files**: `isp_violation_demonstration.rs`, `isp_correct_demonstration.rs`
- **Features**: Trait system, trait objects, compile-time guarantees
- **Key Concepts**: Trait composition, zero-cost abstractions, memory safety

## Key Differences Across Languages

| Language   | Mechanism | Enforcement | Key Features |
|------------|-----------|-------------|--------------|
| Java       | Interfaces | Compile-time | Multiple inheritance of interfaces |
| JavaScript | Duck typing | Runtime | Dynamic method checking |
| Go         | Interfaces | Compile-time | Implicit satisfaction, composition |
| Python     | ABC/Protocols | Runtime | Multiple inheritance, type hints |
| Rust       | Traits | Compile-time | Zero-cost abstractions, memory safety |

## Running the Examples

### Java
```bash
cd java/interface-segregation
javac *.java
java ISPViolationDemonstration
java ISPCorrectDemonstration
```

### JavaScript
```bash
cd javascript/interface-segregation
node ispViolationDemonstration.js
node ispCorrectDemonstration.js
```

### Go
```bash
cd go/interface-segregation
go run ispViolationDemonstration.go
go run ispCorrectDemonstration.go
```

### Python
```bash
cd python/interface-segregation
python isp_violation_demonstration.py
python isp_correct_demonstration.py
```

### Rust
```bash
cd rust/interface-segregation
rustc isp_violation_demonstration.rs && ./isp_violation_demonstration
rustc isp_correct_demonstration.rs && ./isp_correct_demonstration
```

## ISP Best Practices

1. **Keep interfaces small and focused** - Each interface should have a single responsibility
2. **Use interface composition** - Combine small interfaces rather than creating large ones
3. **Segregate by client needs** - Design interfaces based on how clients will use them
4. **Avoid fat interfaces** - Don't add methods to interfaces unless all implementers need them
5. **Consider role-based interfaces** - Create interfaces for specific roles or capabilities
6. **Use dependency inversion** - Depend on abstractions, not concretions

## Benefits of ISP

- **Reduced coupling**: Classes only depend on methods they actually use
- **Improved maintainability**: Changes to unused methods don't affect clients
- **Better testability**: Smaller interfaces are easier to mock and test
- **Enhanced flexibility**: New implementations can pick and choose relevant interfaces
- **Clearer contracts**: Interface purpose is more obvious when focused
- **Easier evolution**: Adding new capabilities doesn't break existing code

## Common ISP Violations

1. **God interfaces**: Interfaces with too many unrelated methods
2. **Convenience interfaces**: Adding methods for convenience rather than necessity
3. **Framework-driven design**: Implementing large framework interfaces when only small parts are needed
4. **Copy-paste interfaces**: Copying existing large interfaces without considering actual needs

## Relationship to Other SOLID Principles

- **SRP**: ISP supports SRP by keeping interfaces focused on single responsibilities
- **OCP**: Well-segregated interfaces make it easier to extend behavior without modification
- **LSP**: Smaller interfaces reduce the chance of substitution violations
- **DIP**: ISP enables better dependency inversion by providing focused abstractions

## Conclusion

The Interface Segregation Principle helps create more maintainable, flexible, and robust software by ensuring that clients only depend on the functionality they actually need. By breaking large interfaces into smaller, focused ones, we reduce coupling, improve testability, and make our code more adaptable to change.

The Robot/Human example clearly demonstrates how forcing a Robot to implement a Human interface violates ISP, and how proper segregation with specialized interfaces (and a Humanoid interface for hybrid capabilities) solves these problems while maintaining clean, understandable code.