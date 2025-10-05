# Liskov Substitution Principle (LSP) - Java Examples

The **Liskov Substitution Principle** states that objects of a superclass should be replaceable with objects of a subclass without altering the correctness of the program.

## What does LSP mean?

**Formal definition**: If S is a subtype of T, then objects of type T may be replaced with objects of type S without altering any of the desirable properties of the program.

**Simple version**: Subtypes must be substitutable for their base types.

## Key LSP Rules

1. **Preconditions cannot be strengthened** in subclasses
2. **Postconditions cannot be weakened** in subclasses  
3. **Invariants must be preserved** in subclasses
4. **No new exceptions** should be thrown by subclass methods
5. **Behavioral substitutability** must be maintained

## The Stack/Vector Problem

Java's `Stack` class is a classic example of LSP violation:

```java
Stack<Integer> stack = new Stack<>();
Vector<Integer> vector = stack; // Legal upcast

// This breaks stack semantics!
vector.remove(0); // Removes from bottom, not top!
```

## File Structure

### ❌ Violation Example

- **`LSPViolationDemonstration.java`**: Shows how Java's Stack violates LSP

**Problems demonstrated:**
1. **Stack extends Vector** but doesn't behave like one
2. **Vector methods break stack contract** (LIFO behavior)
3. **Random access** via List interface breaks stack semantics
4. **Iterator order** doesn't match stack order

```java
// Client expects Vector behavior
public static Vector<Integer> takeFirstOut(Vector<Integer> vector) {
    vector.remove(0); // Removes from BOTTOM of stack!
    return vector;
}

// This breaks when stack is passed as vector
Stack<Integer> stack = new Stack<>();
Vector<Integer> result = takeFirstOut(stack); // Violates LSP!
```

### ✅ Correct Implementation

- **`LSPCorrectDemonstration.java`**: Shows proper LSP-compliant inheritance

**Key features:**
- **Abstract base class** defines clear contract
- **All implementations** honor the same behavioral contract
- **Polymorphic usage** works correctly
- **No unexpected behavior** when substituting types

```java
abstract class DataStructure<T> {
    public abstract void add(T element);
    public abstract T remove();
    public abstract T peek();
    // ... contract methods
}

class ProperStack<T> extends DataStructure<T> {
    // Implements contract correctly - LIFO behavior
}

class ProperQueue<T> extends DataStructure<T> {
    // Implements contract correctly - FIFO behavior
}
```

## Java-Specific LSP Considerations

### 1. Interface vs Implementation Inheritance
```java
// Better: Use interfaces for contracts
interface DataStructure<T> {
    void add(T element);
    T remove();
}

// Avoid: Deep inheritance hierarchies
class BadStack extends Vector<Integer> { // LSP violation risk
```

### 2. Exception Handling
```java
// Base class
public void process() throws IOException {
    // Base implementation
}

// LSP Violation: Subclass throws more exceptions
public void process() throws IOException, SQLException { // Strengthens contract!
    // Subclass implementation
}

// LSP Compliant: Same or fewer exceptions
public void process() { // Weakens contract - OK
    // Subclass implementation
}
```

### 3. Generic Type Safety
```java
// LSP violation with generics
List<Dog> dogs = new ArrayList<>();
List<Animal> animals = dogs; // Compilation error prevents LSP violation

// Correct approach
List<? extends Animal> animals = dogs; // Bounded wildcards
```

## Running the Examples

```bash
# Compile all files
javac *.java

# Run violation demonstration
java LSPViolationDemonstration

# Run correct implementation
java LSPCorrectDemonstration
```

## Real-world Java LSP Examples

### ✅ Good Examples
- **Collections Framework**: `ArrayList`, `LinkedList` implement `List`
- **IO Streams**: `FileInputStream`, `ByteArrayInputStream` extend `InputStream`
- **JDBC**: Different database drivers implement same interfaces

### ❌ Problematic Examples
- **Stack extends Vector**: Random access breaks stack semantics
- **Properties extends Hashtable**: String-only constraint not enforced
- **Rectangle/Square problem**: Square constrains width=height

## Best Practices

1. **Favor composition over inheritance**
2. **Use interfaces to define contracts**
3. **Test substitutability explicitly**
4. **Document behavioral contracts clearly**
5. **Avoid strengthening preconditions**
6. **Avoid weakening postconditions**

## Testing LSP Compliance

```java
// Generic test that should work with any DataStructure
public static void testDataStructure(DataStructure<Integer> ds) {
    ds.add(1);
    ds.add(2);
    
    Integer first = ds.remove();
    Integer second = ds.remove();
    
    // Test should pass for all valid implementations
    assert ds.isEmpty();
}
```

The LSP is crucial for polymorphism to work correctly in Java. Violating it leads to fragile code that breaks when different implementations are substituted.