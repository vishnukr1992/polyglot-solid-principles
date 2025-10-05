"""
VIOLATION OF LISKOV SUBSTITUTION PRINCIPLE

This demonstrates how LSP can be violated in Python when subclasses
don't properly honor the behavioral contract of their base classes.

Python's dynamic nature makes LSP violations particularly dangerous
because they're often only discovered at runtime.
"""

from abc import ABC, abstractmethod
from typing import List, Optional
import random
from enum import Enum


class DataStructure(ABC):
    """Abstract base class defining the expected contract"""
    
    @abstractmethod
    def add(self, element: int) -> None:
        """Add an element to the data structure"""
        pass
    
    @abstractmethod
    def remove(self) -> int:
        """Remove and return an element"""
        pass
    
    @abstractmethod
    def peek(self) -> int:
        """Look at the next element without removing it"""
        pass
    
    @abstractmethod
    def size(self) -> int:
        """Return the number of elements"""
        pass
    
    @abstractmethod
    def is_empty(self) -> bool:
        """Check if empty"""
        pass


class CorrectStack(DataStructure):
    """Baseline correct implementation"""
    
    def __init__(self):
        self._items: List[int] = []
    
    def add(self, element: int) -> None:
        self._items.append(element)
    
    def remove(self) -> int:
        if not self._items:
            raise IndexError("Stack is empty")
        return self._items.pop()  # LIFO
    
    def peek(self) -> int:
        if not self._items:
            raise IndexError("Stack is empty")
        return self._items[-1]
    
    def size(self) -> int:
        return len(self._items)
    
    def is_empty(self) -> bool:
        return len(self._items) == 0
    
    def __str__(self) -> str:
        return f"CorrectStack{self._items}"


# VIOLATION 1: Queue masquerading as a Stack
class MislabeledQueue(DataStructure):
    """LSP VIOLATION: Claims to be a stack but behaves like a queue"""
    
    def __init__(self):
        self._items: List[int] = []
    
    def add(self, element: int) -> None:
        self._items.append(element)
    
    def remove(self) -> int:
        if not self._items:
            raise IndexError("Queue is empty")
        # LSP VIOLATION: FIFO instead of expected LIFO!
        return self._items.pop(0)
    
    def peek(self) -> int:
        if not self._items:
            raise IndexError("Queue is empty")
        # LSP VIOLATION: Peeks at wrong end!
        return self._items[0]
    
    def size(self) -> int:
        return len(self._items)
    
    def is_empty(self) -> bool:
        return len(self._items) == 0
    
    def __str__(self) -> str:
        return f"MislabeledQueue{self._items}"


# VIOLATION 2: Random behavior
class RandomStructure(DataStructure):
    """LSP VIOLATION: Completely unpredictable behavior"""
    
    def __init__(self):
        self._items: List[int] = []
        self._random = random.Random()
    
    def add(self, element: int) -> None:
        self._items.append(element)
    
    def remove(self) -> int:
        if not self._items:
            raise IndexError("Structure is empty")
        # LSP VIOLATION: Removes random element!
        index = self._random.randint(0, len(self._items) - 1)
        return self._items.pop(index)
    
    def peek(self) -> int:
        if not self._items:
            raise IndexError("Structure is empty")
        # LSP VIOLATION: Random peek too!
        index = self._random.randint(0, len(self._items) - 1)
        return self._items[index]
    
    def size(self) -> int:
        return len(self._items)
    
    def is_empty(self) -> bool:
        return len(self._items) == 0
    
    def __str__(self) -> str:
        return f"RandomStructure{self._items}"


# VIOLATION 3: Behavior changes based on state
class InconsistentStructure(DataStructure):
    """LSP VIOLATION: Behavior changes based on internal state"""
    
    def __init__(self):
        self._items: List[int] = []
    
    def add(self, element: int) -> None:
        self._items.append(element)
    
    def remove(self) -> int:
        if not self._items:
            raise IndexError("Structure is empty")
        
        # LSP VIOLATION: Behavior depends on size!
        if len(self._items) <= 2:
            return self._items.pop(0)  # FIFO when small
        else:
            return self._items.pop()   # LIFO when large
    
    def peek(self) -> int:
        if not self._items:
            raise IndexError("Structure is empty")
        
        # LSP VIOLATION: Peek behavior also changes!
        if len(self._items) <= 2:
            return self._items[0]   # Peek front when small
        else:
            return self._items[-1]  # Peek back when large
    
    def size(self) -> int:
        return len(self._items)
    
    def is_empty(self) -> bool:
        return len(self._items) == 0
    
    def __str__(self) -> str:
        return f"InconsistentStructure{self._items}"


# VIOLATION 4: Strengthened preconditions
class RestrictiveStructure(DataStructure):
    """LSP VIOLATION: Adds restrictions not in base contract"""
    
    def __init__(self):
        self._items: List[int] = []
    
    def add(self, element: int) -> None:
        # LSP VIOLATION: Strengthened precondition!
        if element < 0:
            raise ValueError("Negative numbers not allowed!")
        if element > 100:
            raise ValueError("Numbers greater than 100 not allowed!")
        self._items.append(element)
    
    def remove(self) -> int:
        if not self._items:
            raise IndexError("Structure is empty")
        
        # LSP VIOLATION: Additional restriction!
        if len(self._items) == 1:
            raise RuntimeError("Cannot remove last element!")
        
        return self._items.pop()
    
    def peek(self) -> int:
        if not self._items:
            raise IndexError("Structure is empty")
        
        # LSP VIOLATION: Peek also has restrictions!
        if len(self._items) == 1:
            raise RuntimeError("Cannot peek at last element!")
        
        return self._items[-1]
    
    def size(self) -> int:
        return len(self._items)
    
    def is_empty(self) -> bool:
        return len(self._items) == 0
    
    def __str__(self) -> str:
        return f"RestrictiveStructure{self._items}"


# VIOLATION 5: Hidden side effects
class SideEffectStructure(DataStructure):
    """LSP VIOLATION: Operations have unexpected side effects"""
    
    def __init__(self):
        self._items: List[int] = []
        self._operation_count = 0
        self._hidden_log: List[str] = []
    
    def add(self, element: int) -> None:
        self._items.append(element)
        self._operation_count += 1
        
        # LSP VIOLATION: Hidden side effect!
        if self._operation_count % 3 == 0:
            self._hidden_log.append(f"Secret: Added {element}")
            # Secretly double the element!
            self._items.append(element)
    
    def remove(self) -> int:
        if not self._items:
            raise IndexError("Structure is empty")
        
        element = self._items.pop()
        self._operation_count += 1
        
        # LSP VIOLATION: Hidden side effect!
        if self._operation_count % 5 == 0:
            self._hidden_log.append(f"Secret: Removed {element}")
            # Secretly remove another element!
            if self._items:
                self._items.pop()
        
        return element
    
    def peek(self) -> int:
        if not self._items:
            raise IndexError("Structure is empty")
        
        # LSP VIOLATION: Even peek has side effects!
        self._operation_count += 1
        self._hidden_log.append(f"Peeked at {self._items[-1]}")
        
        return self._items[-1]
    
    def size(self) -> int:
        # LSP VIOLATION: Size operation has side effects!
        self._operation_count += 1
        return len(self._items)
    
    def is_empty(self) -> bool:
        return len(self._items) == 0
    
    def __str__(self) -> str:
        return f"SideEffectStructure{self._items}"
    
    def get_hidden_log(self) -> List[str]:
        """This method reveals the hidden operations"""
        return self._hidden_log.copy()


# VIOLATION 6: Weakened postconditions  
class WeakStructure(DataStructure):
    """LSP VIOLATION: Doesn't guarantee what base class promises"""
    
    def __init__(self):
        self._items: List[int] = []
        self._corrupt_chance = 0.1
    
    def add(self, element: int) -> None:
        self._items.append(element)
    
    def remove(self) -> int:
        if not self._items:
            raise IndexError("Structure is empty")
        
        # LSP VIOLATION: Sometimes returns wrong value!
        if random.random() < self._corrupt_chance:
            return -999  # Corrupt value instead of actual element
        
        return self._items.pop()
    
    def peek(self) -> int:
        if not self._items:
            raise IndexError("Structure is empty")
        
        # LSP VIOLATION: Sometimes returns wrong value!
        if random.random() < self._corrupt_chance:
            return -999  # Corrupt value
        
        return self._items[-1]
    
    def size(self) -> int:
        # LSP VIOLATION: Sometimes returns wrong size!
        real_size = len(self._items)
        if random.random() < self._corrupt_chance:
            return real_size + 1  # Wrong size
        return real_size
    
    def is_empty(self) -> bool:
        return len(self._items) == 0
    
    def __str__(self) -> str:
        return f"WeakStructure{self._items}"


# Client function that expects consistent behavior
def process_data_structure(ds: DataStructure, name: str) -> None:
    """Process any data structure - expects consistent behavior"""
    print(f"--- Processing {name} ---")
    
    try:
        # Add elements
        ds.add(10)
        ds.add(20)
        ds.add(30)
        print(f"Added 10, 20, 30. Size: {ds.size()}")
        
        # Peek and remove
        peeked = ds.peek()
        print(f"Peeked: {peeked}")
        
        removed = ds.remove()
        print(f"Removed: {removed}")
        
        # Expect peek and remove to match for stack behavior
        if peeked != removed:
            print(f"⚠️  WARNING: Peek ({peeked}) != Remove ({removed})!")
        
        print(f"Size after removal: {ds.size()}")
        print(f"Is empty: {ds.is_empty()}")
        
    except Exception as e:
        print(f"ERROR: {e}")
    
    print()


def reverse_data_structure(ds: DataStructure) -> None:
    """Tries to reverse elements - works correctly only with proper stacks"""
    print(f"Attempting to reverse: {ds}")
    
    try:
        # Remove all elements
        temp = []
        while not ds.is_empty():
            temp.append(ds.remove())
        
        # Add them back (should reverse for stack)
        for element in temp:
            ds.add(element)
        
        print(f"After reverse attempt: {ds}")
    
    except Exception as e:
        print(f"Reverse failed: {e}")


def demonstrate_violations():
    """Show how different violations break client expectations"""
    print("=== TESTING WITH PROBLEMATIC IMPLEMENTATIONS ===")
    
    structures = [
        CorrectStack(),
        MislabeledQueue(),
        RandomStructure(),
        InconsistentStructure(),
        RestrictiveStructure(),
        SideEffectStructure(),
        WeakStructure()
    ]
    
    names = [
        "CorrectStack (Baseline)",
        "MislabeledQueue (FIFO Violation)",
        "RandomStructure (Unpredictable)",
        "InconsistentStructure (State-dependent)",
        "RestrictiveStructure (Precondition Violation)",
        "SideEffectStructure (Hidden Effects)",
        "WeakStructure (Postcondition Violation)"
    ]
    
    for ds, name in zip(structures, names):
        process_data_structure(ds, name)


def demonstrate_broken_client_code():
    """Show how LSP violations break client code"""
    print("=== DEMONSTRATING BROKEN CLIENT CODE ===")
    
    test_structures = [
        (CorrectStack(), "CorrectStack (works)"),
        (MislabeledQueue(), "MislabeledQueue (broken)"),
        (InconsistentStructure(), "InconsistentStructure (unpredictable)")
    ]
    
    for ds, name in test_structures:
        # Add test elements
        ds.add(1)
        ds.add(2)
        ds.add(3)
        
        print(f"\nReverse test with {name}:")
        reverse_data_structure(ds)


def demonstrate_precondition_violations():
    """Show how strengthened preconditions break LSP"""
    print("\n=== PRECONDITION VIOLATION DEMONSTRATION ===")
    
    restrictive = RestrictiveStructure()
    
    print("Testing RestrictiveStructure:")
    try:
        restrictive.add(5)
        restrictive.add(10)
        print("Added 5, 10 successfully")
        
        print("Trying to add -5 (should fail):")
        restrictive.add(-5)
    except Exception as e:
        print(f"Failed as expected: {e}")
    
    try:
        print("Trying to add 150 (should fail):")
        restrictive.add(150)
    except Exception as e:
        print(f"Failed as expected: {e}")


def demonstrate_side_effects():
    """Show how side effects violate LSP"""
    print("\n=== SIDE EFFECT DEMONSTRATION ===")
    
    side_effect_ds = SideEffectStructure()
    
    print("Testing SideEffectStructure:")
    print("Adding elements 1, 2, 3, 4, 5:")
    
    for i in range(1, 6):
        side_effect_ds.add(i)
        print(f"After adding {i}: size = {side_effect_ds.size()}")
    
    print(f"Final structure: {side_effect_ds}")
    print(f"Hidden log: {side_effect_ds.get_hidden_log()}")


def main():
    print("=== LSP VIOLATION DEMONSTRATION ===")
    print("Objects implementing same interface but violating behavioral contracts")
    print()
    
    demonstrate_violations()
    demonstrate_broken_client_code()
    demonstrate_precondition_violations()
    demonstrate_side_effects()
    
    print("\n=== WHY THESE VIOLATE LSP ===")
    print("1. MislabeledQueue: Changes expected removal order (FIFO vs LIFO)")
    print("2. RandomStructure: Unpredictable behavior breaks client expectations")
    print("3. InconsistentStructure: Behavior changes based on internal state")
    print("4. RestrictiveStructure: Strengthens preconditions with new restrictions")
    print("5. SideEffectStructure: Hidden side effects violate behavioral contract")
    print("6. WeakStructure: Weakens postconditions by returning wrong values")
    print("7. Client code written for base class fails with these implementations")
    print("8. Substitutability is broken - cannot replace base with derived safely")


if __name__ == "__main__":
    main()