# Open-Closed Principle (OCP) - Go Examples

The **Open-Closed Principle** states that software entities (classes, modules, functions, etc.) should be **open for extension but closed for modification**.

## What does this mean?

- **Open for Extension**: You should be able to add new functionality to existing code
- **Closed for Modification**: You should not need to modify existing code when adding new features

## The Transistor Example

We use electronic transistors as our domain example because they perfectly illustrate OCP:
- Different transistor types (BJT, FET, MOSFET) have similar interfaces but different behaviors
- New transistor types are frequently developed in electronics
- An amplifier circuit should work with any transistor type

## File Structure

### ✅ Correct Implementation (Follows OCP)

- **`Transistor.go`**: Interface defining the contract for all transistor types
- **`BjtTransistor.go`**: BJT implementation of the Transistor interface

```go
// Interface allows extension without modification
type Transistor interface {
    Base(signal float64)
    Collector(input float64)
    Output() float64
}
```

**Benefits:**
- Adding new transistor types (FET, MOSFET) requires no changes to existing code
- Each transistor type handles its own behavior
- Amplifier circuits work with any Transistor implementation
- Easy to test and maintain

### ❌ Violation Example

- **`BjtOcpViolation.go`**: Shows how NOT to implement OCP

**Problems:**
- Hard-coded switch statements for different transistor types
- Adding new types requires modifying existing methods
- Violates Single Responsibility Principle too
- Difficult to test and maintain

## Key Takeaways

1. **Use interfaces/abstractions** to define contracts
2. **Avoid switch/if-else chains** based on types
3. **Prefer composition over modification** when adding features
4. **Think about future extensions** when designing interfaces

## Running the Examples

```bash
# Run the correct implementation
go run Transistor.go BjtTransistor.go

# Run the violation example
go run BjtOcpViolation.go
```

## Real-world Applications

- Plugin architectures
- Strategy pattern implementations
- Device drivers
- Payment processing systems
- Notification systems

The OCP is fundamental to creating flexible, maintainable software that can evolve without breaking existing functionality.