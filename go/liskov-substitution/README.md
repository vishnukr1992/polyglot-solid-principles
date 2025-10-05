# Liskov Substitution Principle (LSP) - Go Examples

The **Liskov Substitution Principle** states that objects of a superclass should be replaceable with objects of a subclass without altering the correctness of the program.

## LSP in Go

Go's **interface system** naturally enforces LSP through:
- **Implicit interface satisfaction**: Types implement interfaces automatically
- **Compile-time checking**: Interface compliance verified at compile time
- **Structural typing**: "If it walks like a duck and quacks like a duck..."
- **No inheritance**: Composition over inheritance prevents many LSP violations

## Go's Interface Advantage

```go
// Interface defines the contract
type DataStructure interface {
    Add(element int)
    Remove() (int, error)
    Size() int
}

// Any type implementing these methods satisfies the interface
type Stack struct { items []int }
type Queue struct { items []int }

// Client code works with any implementation
func ProcessDataStructure(ds DataStructure) {
    ds.Add(42)
    element, _ := ds.Remove()
    // Works with Stack, Queue, or any other implementation
}
```

## File Structure

### ✅ Correct Implementation

- **`lspCorrectDemonstration.go`**: Shows proper LSP compliance with Go interfaces

**Key features:**
- **Interface contracts**: Clear behavioral specifications
- **Multiple implementations**: Stack, Queue, PriorityQueue, Deque
- **Consistent error handling**: All use same error patterns
- **Polymorphic functions**: Work with any DataStructure

```go
type DataStructure interface {
    Add(element int)
    Remove() (int, error)    // Consistent error handling
    Peek() (int, error)      // Consistent return types
    Size() int
    IsEmpty() bool
}

// All implementations honor the contract
func (s *Stack) Remove() (int, error) {
    if len(s.items) == 0 {
        return 0, fmt.Errorf("stack is empty")
    }
    // LIFO behavior as expected
    return s.items[len(s.items)-1], nil
}
```

### ❌ Violation Examples

- **`lspViolationDemonstration.go`**: Shows various LSP violations in Go

**Violations demonstrated:**
1. **Behavior changes**: Queue with stack interface (FIFO vs LIFO)
2. **Random behavior**: Unpredictable element removal
3. **State-dependent behavior**: Different behavior based on size
4. **Strengthened preconditions**: Additional input restrictions
5. **Hidden side effects**: Unexpected operations during method calls

```go
// LSP Violation: Queue pretending to be stack
type MislabeledQueue struct {
    items []int
}

func (q *MislabeledQueue) Remove() (int, error) {
    // VIOLATION: FIFO instead of expected LIFO!
    return q.items[0], nil // Wrong end!
}
```

## Go-Specific LSP Features

### 1. Interface Composition
```go
// Compose interfaces for complex contracts
type Reader interface {
    Read([]byte) (int, error)
}

type Writer interface {
    Write([]byte) (int, error)
}

type ReadWriter interface {
    Reader
    Writer
}
```

### 2. Empty Interface
```go
// interface{} accepts any type
func ProcessAny(data interface{}) {
    // Type assertion to verify contract
    if ds, ok := data.(DataStructure); ok {
        ds.Add(42)
    }
}
```

### 3. Interface Assertions
```go
// Check if type implements additional interfaces
func OptionalFeature(ds DataStructure) {
    if stringer, ok := ds.(fmt.Stringer); ok {
        fmt.Println(stringer.String())
    }
}
```

### 4. Error Interface
```go
// Go's error interface is a great LSP example
type error interface {
    Error() string
}

// Any type implementing Error() satisfies error
type CustomError struct {
    message string
}

func (e CustomError) Error() string {
    return e.message
}
```

## Common Go LSP Violations

### 1. Panic Instead of Error
```go
// VIOLATION: Panics instead of returning error
func (bad *BadStack) Remove() (int, error) {
    if len(bad.items) == 0 {
        panic("stack is empty") // Should return error!
    }
    return bad.items[len(bad.items)-1], nil
}
```

### 2. Inconsistent Nil Handling
```go
// VIOLATION: Some implementations handle nil, others don't
func (inconsistent *InconsistentType) Process(data *Data) error {
    if data == nil {
        return fmt.Errorf("data cannot be nil") // Strengthened precondition!
    }
    // Process data
}
```

### 3. Method Receiver Inconsistency
```go
// VIOLATION: Mixed pointer and value receivers
type BadType struct{}

func (b BadType) Read() {}    // Value receiver
func (b *BadType) Write() {}  // Pointer receiver - inconsistent!
```

## Running the Examples

```bash
# Run correct implementation
go run lspCorrectDemonstration.go

# Run violation examples (note: has renamed main function)
go run lspViolationDemonstration.go
# Or call mainViolation() from another file

# Build and run
go build -o lsp_demo lspCorrectDemonstration.go
./lsp_demo
```

## Interface Design Best Practices

### 1. Keep Interfaces Small
```go
// Good: Small, focused interface
type Writer interface {
    Write([]byte) (int, error)
}

// Avoid: Large, monolithic interfaces
type BadInterface interface {
    Read([]byte) (int, error)
    Write([]byte) (int, error)
    Seek(int64, int) (int64, error)
    Close() error
    // ... many more methods
}
```

### 2. Interface Segregation
```go
// Separate concerns into different interfaces
type Readable interface {
    Read([]byte) (int, error)
}

type Closable interface {
    Close() error
}

// Compose as needed
type ReadCloser interface {
    Readable
    Closable
}
```

### 3. Error Handling Consistency
```go
// Consistent error patterns across implementations
func (s *Stack) Remove() (int, error) {
    if s.IsEmpty() {
        return 0, fmt.Errorf("stack is empty")
    }
    // Implementation
}

func (q *Queue) Remove() (int, error) {
    if q.IsEmpty() {
        return 0, fmt.Errorf("queue is empty") // Same pattern
    }
    // Implementation
}
```

## Testing LSP Compliance

```go
// Generic test function for any DataStructure
func TestDataStructureContract(t *testing.T, createDS func() DataStructure) {
    ds := createDS()
    
    // Test basic operations
    ds.Add(1)
    ds.Add(2)
    
    if ds.Size() != 2 {
        t.Errorf("Expected size 2, got %d", ds.Size())
    }
    
    element, err := ds.Remove()
    if err != nil {
        t.Errorf("Unexpected error: %v", err)
    }
    
    // Contract should hold for all implementations
}

// Test all implementations
func TestAllImplementations(t *testing.T) {
    TestDataStructureContract(t, func() DataStructure { return NewStack() })
    TestDataStructureContract(t, func() DataStructure { return NewQueue() })
    TestDataStructureContract(t, func() DataStructure { return NewPriorityQueue() })
}
```

## Real-world Go Examples

### ✅ Good LSP Examples
- **io.Reader/Writer**: Consistent interfaces across file, network, buffer types
- **database/sql**: Different database drivers implement same interface
- **net.Conn**: Various connection types (TCP, UDP, Unix) implement same interface
- **sort.Interface**: Any type can be sorted by implementing Less, Len, Swap

### ⚠️ Potential Issues
- **context.Context**: Some implementations handle cancellation differently
- **http.Handler**: Varying error handling approaches

## Go LSP Advantages

1. **Compile-time verification**: Interface satisfaction checked at compile time
2. **No inheritance**: Composition prevents classical inheritance LSP violations
3. **Structural typing**: Focus on behavior, not implementation
4. **Explicit error handling**: Forces consistent error patterns
5. **Interface composition**: Build complex contracts from simple ones

Go's interface system makes LSP violations less common and easier to detect, promoting robust, substitutable code design.