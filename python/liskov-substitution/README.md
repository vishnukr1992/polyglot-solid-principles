# Liskov Substitution Principle (LSP) - Python Examples

The **Liskov Substitution Principle** states that objects of a superclass should be replaceable with objects of a subclass without altering the correctness of the program.

## LSP in Python

Python's **dynamic typing** and **duck typing** make LSP both powerful and dangerous:
- ✅ **Powerful**: Rich inheritance and protocol support
- ⚠️ **Dangerous**: Violations often only discovered at runtime
- 🦆 **Duck typing**: "If it walks like a duck and quacks like a duck, it's a duck"

## Python's LSP Features

```python
# Abstract Base Classes enforce contracts
from abc import ABC, abstractmethod

class DataStructure(ABC):
    @abstractmethod
    def add(self, element: int) -> None:
        pass
    
    @abstractmethod
    def remove(self) -> int:
        pass

# Duck typing - no explicit inheritance needed
def process_stack_like(obj):
    if hasattr(obj, 'push') and hasattr(obj, 'pop'):
        obj.push(42)
        return obj.pop()
```

## File Structure

### ✅ Correct Implementation

- **`lsp_correct_demonstration.py`**: Shows proper LSP compliance with Python

**Key features:**
- **Abstract Base Classes**: Clear contract definition
- **Type hints**: Improved clarity and IDE support
- **Consistent behavior**: All implementations honor the same contract
- **Proper exception handling**: Consistent error patterns

```python
class DataStructure(ABC):
    @abstractmethod
    def add(self, element: int) -> None:
        """Add an element to the data structure"""
        pass
    
    @abstractmethod
    def remove(self) -> int:
        """Remove and return an element"""
        pass

class Stack(DataStructure):
    def remove(self) -> int:
        if not self._items:
            raise IndexError("Stack is empty")
        return self._items.pop()  # LIFO as expected

class Queue(DataStructure):
    def remove(self) -> int:
        if not self._items:
            raise IndexError("Queue is empty")
        return self._items.pop(0)  # FIFO as documented
```

### ❌ Violation Examples

- **`lsp_violation_demonstration.py`**: Shows various LSP violations

**Violations demonstrated:**
1. **Behavioral changes**: Queue with stack interface
2. **Random behavior**: Unpredictable operations
3. **State-dependent behavior**: Behavior changes based on internal state
4. **Strengthened preconditions**: Additional restrictions in subclasses
5. **Hidden side effects**: Unexpected operations
6. **Weakened postconditions**: Wrong return values

```python
# LSP Violation: Different behavior contract
class MislabeledQueue(DataStructure):
    def remove(self) -> int:
        # VIOLATION: FIFO instead of expected LIFO!
        return self._items.pop(0)
    
    def peek(self) -> int:
        # VIOLATION: Peeks at wrong end!
        return self._items[0]  # Should be self._items[-1]
```

## Python-Specific LSP Features

### 1. Abstract Base Classes (ABC)
```python
from abc import ABC, abstractmethod

class Shape(ABC):
    @abstractmethod
    def area(self) -> float:
        pass
    
    @abstractmethod
    def perimeter(self) -> float:
        pass

# Cannot instantiate abstract class
# shape = Shape()  # TypeError!
```

### 2. Protocols (Python 3.8+)
```python
from typing import Protocol

class StackProtocol(Protocol):
    def push(self, item: int) -> None: ...
    def pop(self) -> int: ...

# Any class with these methods satisfies the protocol
def process_stack(stack: StackProtocol) -> int:
    stack.push(42)
    return stack.pop()
```

### 3. Duck Typing
```python
# No explicit inheritance needed
def process_file_like(obj):
    if hasattr(obj, 'read') and hasattr(obj, 'write'):
        data = obj.read()
        obj.write(data.upper())

# Works with files, StringIO, BytesIO, etc.
process_file_like(open('file.txt'))
process_file_like(io.StringIO())
```

### 4. Multiple Inheritance
```python
class Readable:
    def read(self) -> str:
        return "reading"

class Writable:
    def write(self, data: str) -> None:
        print(f"writing: {data}")

class ReadWritable(Readable, Writable):
    pass  # Inherits both interfaces
```

### 5. Mixins
```python
class TimestampMixin:
    def get_timestamp(self):
        return datetime.now()

class LoggingMixin:
    def log(self, message):
        print(f"[{self.get_timestamp()}] {message}")

class TimestampedStack(Stack, TimestampMixin, LoggingMixin):
    def add(self, element):
        self.log(f"Adding {element}")
        super().add(element)
```

### 6. Decorators for Behavior Extension
```python
def logged(cls):
    """Decorator to add logging to all methods"""
    for name, method in cls.__dict__.items():
        if callable(method) and not name.startswith('_'):
            setattr(cls, name, log_calls(method))
    return cls

def log_calls(func):
    def wrapper(*args, **kwargs):
        print(f"Calling {func.__name__}")
        return func(*args, **kwargs)
    return wrapper

@logged
class LoggedStack(Stack):
    pass  # All methods automatically logged
```

## Common Python LSP Violations

### 1. Square/Rectangle Problem
```python
# Classic LSP violation
class Rectangle:
    def __init__(self, width, height):
        self._width = width
        self._height = height
    
    def set_width(self, width):
        self._width = width
    
    def set_height(self, height):
        self._height = height

class Square(Rectangle):
    def set_width(self, width):
        # VIOLATION: Changes both dimensions
        self._width = self._height = width
    
    def set_height(self, height):
        # VIOLATION: Changes both dimensions
        self._width = self._height = height

# Breaks client expectations
def test_rectangle(rect):
    rect.set_width(5)
    rect.set_height(4)
    assert rect.area() == 20  # Fails with Square!
```

### 2. List Subclass Violations
```python
# VIOLATION: Immutable list breaks list contract
class ImmutableList(list):
    def append(self, item):
        raise TypeError("Cannot modify immutable list")
    
    def extend(self, items):
        raise TypeError("Cannot modify immutable list")

# Breaks code expecting mutable list
def add_items(lst):
    lst.append(1)  # Fails with ImmutableList!
    lst.extend([2, 3])
```

### 3. Exception Hierarchy Violations
```python
# VIOLATION: Subclass throws different exceptions
class BaseProcessor:
    def process(self, data):
        if not data:
            raise ValueError("Data cannot be empty")
        return data.upper()

class StrictProcessor(BaseProcessor):
    def process(self, data):
        if not data:
            # VIOLATION: Different exception type!
            raise TypeError("Data must be non-empty string")
        if not isinstance(data, str):
            # VIOLATION: Additional restriction!
            raise TypeError("Data must be string")
        return super().process(data)
```

## Running the Examples

```bash
# Run correct implementation
python lsp_correct_demonstration.py

# Run violation examples
python lsp_violation_demonstration.py

# With type checking (requires mypy)
pip install mypy
mypy lsp_correct_demonstration.py
```

## Type Checking and LSP

```python
# Use mypy for static type checking
from typing import List, Protocol

class Sortable(Protocol):
    def __lt__(self, other) -> bool: ...

def sort_items(items: List[Sortable]) -> List[Sortable]:
    return sorted(items)

# mypy will check LSP compliance at static analysis time
```

## Testing LSP Compliance

```python
import pytest
from abc import ABC

def test_data_structure_contract(ds_factory):
    """Generic test for any DataStructure implementation"""
    ds = ds_factory()
    
    # Test basic operations
    ds.add(1)
    ds.add(2)
    assert ds.size() == 2
    
    # Test remove operation
    element = ds.remove()
    assert isinstance(element, int)
    assert ds.size() == 1
    
    # Test empty state
    ds.remove()  # Remove last element
    assert ds.is_empty()
    
    # Test error handling
    with pytest.raises(IndexError):
        ds.remove()  # Should raise on empty

# Parametrized test for all implementations
@pytest.mark.parametrize("ds_factory", [
    lambda: Stack(),
    lambda: Queue(),
    lambda: PriorityQueue(),
])
def test_all_implementations(ds_factory):
    test_data_structure_contract(ds_factory)
```

## Real-world Python Examples

### ✅ Good LSP Examples
- **Collections**: `list`, `tuple`, `deque` all implement sequence protocol
- **File objects**: `TextIOWrapper`, `StringIO`, `BytesIO` implement file protocol
- **Iterators**: Any object implementing `__iter__` and `__next__`
- **Context managers**: Any object implementing `__enter__` and `__exit__`

### ❌ Common Violations
- **Django model inheritance**: Child models with different validation rules
- **Custom list subclasses**: Adding restrictions that break list semantics
- **Exception handling**: Subclasses throwing different exception types

## Best Practices

1. **Use Abstract Base Classes** for clear contracts
2. **Leverage type hints** for better documentation
3. **Test behavioral contracts**, not just method signatures
4. **Use Protocols** for structural subtyping
5. **Avoid strengthening preconditions** in subclasses
6. **Maintain consistent exception handling**
7. **Document behavioral requirements** clearly

```python
# Good: Clear contract documentation
class DataStructure(ABC):
    @abstractmethod
    def add(self, element: int) -> None:
        """Add element to the structure.
        
        Args:
            element: Integer to add
            
        Raises:
            MemoryError: If out of memory
        """
        pass
```

Python's rich type system and dynamic features provide powerful tools for LSP compliance, but require careful design to avoid runtime violations.