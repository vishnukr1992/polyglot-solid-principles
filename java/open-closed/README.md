# Open-Closed Principle (OCP) - Java Examples

The **Open-Closed Principle** states that software entities should be **open for extension but closed for modification**.

## Java Implementation Approach

Java uses **interfaces** and **abstract classes** to implement OCP effectively through polymorphism and inheritance.

## File Structure

### ✅ Correct Implementation (Follows OCP)

- **`Transistor.java`**: Interface defining the contract for all transistor types
- **`BjtTransistor.java`**: Contains BJT, FET, MOSFET implementations and AmplifierCircuit

```java
public interface Transistor {
    void base(double signal);
    void collector(double input);
    double output();
}
```

**Key Java Features Used:**
- **Interfaces**: Define contracts without implementation
- **Polymorphism**: `List<Transistor>` can hold any implementation
- **Method overriding**: Each class implements behavior differently

**Benefits:**
```java
// Adding new transistor types requires no changes to existing code
class NewTransistorType implements Transistor {
    // Just implement the interface methods
}

// Amplifier works with any Transistor implementation
public double amplifySignal(double signal, double input) {
    for (Transistor transistor : transistors) {
        transistor.base(signal);
        transistor.collector(input);
        totalOutput += transistor.output();
    }
}
```

### ❌ Violation Example

- **`TransistorOCPViolation.java`**: Shows how NOT to implement OCP

**Problems:**
```java
// This switch statement violates OCP
switch (transistor.transistorType) {
    case BJT_TYPE:
        // BJT logic
        break;
    case FET_TYPE:
        // FET logic - requires modifying this method
        break;
    // Adding new types requires more cases here
}
```

## Java-Specific OCP Benefits

1. **Compile-time safety**: Interfaces ensure all required methods are implemented
2. **IDE support**: IntelliJ/Eclipse can generate implementations automatically
3. **Reflection**: Can dynamically load implementations at runtime
4. **Annotations**: Can mark extension points with custom annotations

## Running the Examples

```bash
# Compile and run correct implementation
javac *.java
java BjtTransistor

# Run violation example
java TransistorOCPViolation
```

## Advanced Java OCP Patterns

### 1. Factory Pattern
```java
public class TransistorFactory {
    public static Transistor create(String type) {
        switch(type) {
            case "BJT": return new BJTTransistor();
            case "FET": return new FETTransistor();
            default: throw new IllegalArgumentException();
        }
    }
}
```

### 2. Plugin Architecture
```java
// Use ServiceLoader for dynamic plugin loading
ServiceLoader<Transistor> transistors = ServiceLoader.load(Transistor.class);
```

### 3. Spring Framework Integration
```java
@Component
public class BJTTransistor implements Transistor {
    // Spring will automatically discover and inject implementations
}
```

## Real-world Java Applications

- **Spring Framework**: Dependency injection and AOP
- **JDBC Drivers**: Database-specific implementations
- **Servlet API**: Web server implementations
- **Collections Framework**: Different data structure implementations
- **Logging Frameworks**: SLF4J with various backends

Java's strong type system and extensive ecosystem make it excellent for implementing OCP in enterprise applications.