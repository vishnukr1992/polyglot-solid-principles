# Open-Closed Principle (OCP) - Python Examples

The **Open-Closed Principle** states that software entities should be **open for extension but closed for modification**.

## Python Implementation Approach

Python uses **Abstract Base Classes (ABC)**, **duck typing**, and **multiple inheritance** to implement OCP effectively.

## File Structure

### ✅ Correct Implementation (Follows OCP)

- **`transistor.py`**: Contains ABC and all correct implementations

```python
from abc import ABC, abstractmethod

class Transistor(ABC):
    @abstractmethod
    def base(self, signal: float) -> None:
        pass
    
    @abstractmethod
    def collector(self, input_value: float) -> None:
        pass
    
    @abstractmethod
    def output(self) -> float:
        pass
```

**Key Python Features Used:**
- **Abstract Base Classes**: Enforce interface contracts
- **Type hints**: Improve code clarity and IDE support
- **Duck typing**: "If it looks like a duck and quacks like a duck..."
- **Multiple inheritance**: Mix functionality from multiple sources

**Benefits:**
```python
# Adding new transistor types requires no changes to existing code
class JFETTransistor(Transistor):
    def base(self, signal: float) -> None:
        # JFET-specific logic
        pass
    
    def collector(self, input_value: float) -> None:
        # JFET-specific logic
        pass
    
    def output(self) -> float:
        # JFET-specific logic
        return 0.0

# Amplifier works with any Transistor implementation
def amplify_signal(self, signal: float, input_value: float) -> float:
    for transistor in self.transistors:
        transistor.base(signal)
        transistor.collector(input_value)
        total_output += transistor.output()
```

### ❌ Violation Example

- **`transistor_ocp_violation.py`**: Shows how NOT to implement OCP

**Problems:**
```python
# This if-elif chain violates OCP
if transistor.transistor_type == TransistorType.BJT:
    # BJT logic
    output = transistor.process_signal(signal, input_value)
elif transistor.transistor_type == TransistorType.FET:
    # FET logic - requires modifying this method
    output = transistor.process_signal(signal, input_value) * 1.1
# Adding new types requires more elif statements here
```

## Python-Specific OCP Features

### 1. Duck Typing
```python
# No need for explicit interface if objects have required methods
def amplify_signal(transistors, signal, input_value):
    for transistor in transistors:
        if hasattr(transistor, 'base') and hasattr(transistor, 'output'):
            transistor.base(signal)
            transistor.collector(input_value)
            yield transistor.output()
```

### 2. Mixins
```python
class LoggingMixin:
    def log_output(self):
        result = self.output()
        print(f"Output: {result}")
        return result

class LoggingBJT(BJTTransistor, LoggingMixin):
    pass  # Inherits both BJT behavior and logging
```

### 3. Decorators
```python
def with_caching(cls):
    """Decorator to add caching to transistor output"""
    original_output = cls.output
    
    def cached_output(self):
        if not hasattr(self, '_cache'):
            self._cache = original_output(self)
        return self._cache
    
    cls.output = cached_output
    return cls

@with_caching
class CachedBJT(BJTTransistor):
    pass
```

### 4. Metaclasses
```python
class TransistorMeta(type):
    def __new__(mcs, name, bases, attrs):
        # Automatically register new transistor types
        cls = super().__new__(mcs, name, bases, attrs)
        if hasattr(cls, 'register'):
            cls.register()
        return cls

class AutoRegisteringTransistor(Transistor, metaclass=TransistorMeta):
    pass
```

## Running the Examples

```bash
# Run correct implementation
python transistor.py

# Run violation example
python transistor_ocp_violation.py

# Run with type checking
mypy transistor.py
```

## Advanced Python OCP Patterns

### 1. Factory with Registration
```python
class TransistorFactory:
    _transistors = {}
    
    @classmethod
    def register(cls, name, transistor_class):
        cls._transistors[name] = transistor_class
    
    @classmethod
    def create(cls, name, *args, **kwargs):
        transistor_class = cls._transistors.get(name)
        if transistor_class:
            return transistor_class(*args, **kwargs)
        raise ValueError(f"Unknown transistor type: {name}")

# Auto-registration decorator
def register_transistor(name):
    def decorator(cls):
        TransistorFactory.register(name, cls)
        return cls
    return decorator

@register_transistor('BJT')
class BJTTransistor(Transistor):
    pass
```

### 2. Plugin System
```python
import importlib
import pkgutil

def load_transistor_plugins(package_name):
    """Dynamically load transistor plugins"""
    package = importlib.import_module(package_name)
    for _, module_name, _ in pkgutil.iter_modules(package.__path__):
        module = importlib.import_module(f"{package_name}.{module_name}")
        # Plugins auto-register themselves when imported
```

### 3. Context Managers
```python
class TransistorContext:
    def __init__(self, transistor):
        self.transistor = transistor
    
    def __enter__(self):
        self.transistor.power_on()
        return self.transistor
    
    def __exit__(self, exc_type, exc_val, exc_tb):
        self.transistor.power_off()

# Usage
with TransistorContext(BJTTransistor()) as transistor:
    transistor.base(2.0)
    result = transistor.output()
```

### 4. Protocol (Python 3.8+)
```python
from typing import Protocol

class TransistorProtocol(Protocol):
    def base(self, signal: float) -> None: ...
    def collector(self, input_value: float) -> None: ...
    def output(self) -> float: ...

# Any class implementing these methods satisfies the protocol
def process_transistor(transistor: TransistorProtocol) -> float:
    transistor.base(1.0)
    transistor.collector(5.0)
    return transistor.output()
```

## Real-world Python Applications

- **Django**: Model inheritance and custom field types
- **SQLAlchemy**: Database dialect implementations
- **Pytest**: Plugin architecture with fixtures
- **Flask**: Blueprint system for modular applications
- **Pandas**: Custom data types and operations
- **Scikit-learn**: Estimator interface for ML algorithms

Python's dynamic nature and rich metaprogramming capabilities make it excellent for implementing flexible, extensible architectures following OCP.