"""
CORRECT IMPLEMENTATION - FOLLOWS LISKOV SUBSTITUTION PRINCIPLE

This demonstrates proper inheritance where subtypes can be substituted
for their base types without breaking the program's correctness.

In Python, LSP is enforced through proper inheritance design and
behavioral contracts, even though the language is dynamically typed.
"""

from abc import ABC, abstractmethod
from typing import List, Optional
import random


class DataStructure(ABC):
    """Abstract base class defining the contract for data structures"""
    
    @abstractmethod
    def add(self, element: int) -> None:
        """Add an element to the data structure"""
        pass
    
    @abstractmethod
    def remove(self) -> int:
        """Remove and return an element from the data structure"""
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
        """Check if the data structure is empty"""
        pass


class Stack(DataStructure):
    """Stack implementation (LIFO - Last In, First Out)"""
    
    def __init__(self):
        self._items: List[int] = []
    
    def add(self, element: int) -> None:
        self._items.append(element)
    
    def remove(self) -> int:
        if not self._items:
            raise IndexError("Stack is empty")
        return self._items.pop()  # Remove from end (LIFO)
    
    def peek(self) -> int:
        if not self._items:
            raise IndexError("Stack is empty")
        return self._items[-1]  # Peek at end
    
    def size(self) -> int:
        return len(self._items)
    
    def is_empty(self) -> bool:
        return len(self._items) == 0
    
    def __str__(self) -> str:
        return f"Stack{self._items}"


class Queue(DataStructure):
    """Queue implementation (FIFO - First In, First Out)"""
    
    def __init__(self):
        self._items: List[int] = []
    
    def add(self, element: int) -> None:
        self._items.append(element)  # Add to end
    
    def remove(self) -> int:
        if not self._items:
            raise IndexError("Queue is empty")
        return self._items.pop(0)  # Remove from beginning (FIFO)
    
    def peek(self) -> int:
        if not self._items:
            raise IndexError("Queue is empty")
        return self._items[0]  # Peek at beginning
    
    def size(self) -> int:
        return len(self._items)
    
    def is_empty(self) -> bool:
        return len(self._items) == 0
    
    def __str__(self) -> str:
        return f"Queue{self._items}"


class PriorityQueue(DataStructure):
    """Priority Queue implementation (highest value first)"""
    
    def __init__(self):
        self._items: List[int] = []
    
    def add(self, element: int) -> None:
        self._items.append(element)
        self._items.sort()  # Keep sorted for priority
    
    def remove(self) -> int:
        if not self._items:
            raise IndexError("Priority queue is empty")
        return self._items.pop()  # Remove highest (last after sorting)
    
    def peek(self) -> int:
        if not self._items:
            raise IndexError("Priority queue is empty")
        return self._items[-1]  # Peek at highest
    
    def size(self) -> int:
        return len(self._items)
    
    def is_empty(self) -> bool:
        return len(self._items) == 0
    
    def __str__(self) -> str:
        return f"PriorityQueue{self._items}"


class Deque(DataStructure):
    """Double-ended queue that can add/remove from both ends"""
    
    def __init__(self, mode='back'):
        """Mode can be 'front' or 'back' to specify default removal end"""
        self._items: List[int] = []
        self._mode = mode
    
    def add(self, element: int) -> None:
        self._items.append(element)  # Always add to back
    
    def remove(self) -> int:
        if not self._items:
            raise IndexError("Deque is empty")
        
        if self._mode == 'front':
            return self._items.pop(0)  # Remove from front
        else:
            return self._items.pop()   # Remove from back
    
    def peek(self) -> int:
        if not self._items:
            raise IndexError("Deque is empty")
        
        if self._mode == 'front':
            return self._items[0]   # Peek at front
        else:
            return self._items[-1]  # Peek at back
    
    def size(self) -> int:
        return len(self._items)
    
    def is_empty(self) -> bool:
        return len(self._items) == 0
    
    def __str__(self) -> str:
        return f"Deque({self._mode}){self._items}"


# Client functions that work with any DataStructure implementation
def process_data_structure(ds: DataStructure, name: str) -> None:
    """Process any data structure following the contract"""
    print(f"--- Processing {name} ---")
    
    # Add elements
    ds.add(10)
    ds.add(20)
    ds.add(30)
    print(f"After adding 10, 20, 30 - Size: {ds.size()}")
    
    # Peek at next element
    try:
        next_element = ds.peek()
        print(f"Next element (peek): {next_element}")
    except IndexError as e:
        print(f"Peek error: {e}")
    
    # Remove elements
    try:
        element = ds.remove()
        print(f"Removed: {element}")
    except IndexError as e:
        print(f"Remove error: {e}")
    
    print(f"Size after removal: {ds.size()}")
    print(f"Is empty: {ds.is_empty()}")
    print()


def transfer_elements(source: DataStructure, target: DataStructure, count: int) -> None:
    """Transfer elements from source to target"""
    for _ in range(count):
        if source.is_empty():
            break
        try:
            element = source.remove()
            target.add(element)
        except IndexError:
            break


def count_elements(ds: DataStructure) -> int:
    """Count elements by removing them (destructive)"""
    count = 0
    while not ds.is_empty():
        try:
            ds.remove()
            count += 1
        except IndexError:
            break
    return count


def sum_elements(ds: DataStructure) -> int:
    """Sum all elements by removing them (destructive)"""
    total = 0
    while not ds.is_empty():
        try:
            element = ds.remove()
            total += element
        except IndexError:
            break
    return total


def demonstrate_polymorphism():
    """Show how different implementations can be used polymorphically"""
    print("=== POLYMORPHIC BEHAVIOR DEMONSTRATION ===")
    
    # Create different data structures
    structures = [
        Stack(),
        Queue(),
        PriorityQueue(),
        Deque('back'),
        Deque('front')
    ]
    
    names = [
        "Stack",
        "Queue", 
        "PriorityQueue",
        "Deque(back)",
        "Deque(front)"
    ]
    
    # All can be treated the same way
    for ds, name in zip(structures, names):
        ds.add(5)
        ds.add(1)
        ds.add(3)
        
        try:
            next_element = ds.peek()
            print(f"{name} peek: {next_element}")
        except IndexError:
            print(f"{name} peek: empty")
    
    print()


def demonstrate_transfer():
    """Demonstrate transfer between different data structure types"""
    print("=== TRANSFER OPERATION DEMONSTRATION ===")
    
    source = Stack()
    target = Queue()
    
    # Populate source
    for i in [1, 2, 3, 4, 5]:
        source.add(i)
    
    print(f"Before transfer - Source: {source}, Target: {target}")
    
    # Transfer 3 elements
    transfer_elements(source, target, 3)
    
    print(f"After transfer - Source: {source}, Target: {target}")
    print()


def demonstrate_processors():
    """Demonstrate functions that process any data structure"""
    print("=== PROCESSOR DEMONSTRATION ===")
    
    # Create and populate different structures
    stack = Stack()
    queue = Queue()
    
    for i in [10, 20, 30, 40, 50]:
        stack.add(i)
        queue.add(i)
    
    # Process with different functions
    print(f"Stack before counting: {stack}")
    count = count_elements(Stack())  # Use fresh stack
    for i in [10, 20, 30, 40, 50]:
        Stack().add(i)  # Can't modify original, so create new one
    
    # Better approach: clone the structures
    stack_copy = Stack()
    queue_copy = Queue()
    
    for i in [10, 20, 30, 40, 50]:
        stack_copy.add(i)
        queue_copy.add(i)
    
    stack_count = count_elements(stack_copy)
    queue_sum = sum_elements(queue_copy)
    
    print(f"Stack element count: {stack_count}")
    print(f"Queue element sum: {queue_sum}")
    print()


def main():
    print("=== LSP CORRECT DEMONSTRATION ===")
    print("All implementations can be substituted without breaking correctness")
    print()
    
    # Test all implementations with the same client code
    stack = Stack()
    queue = Queue()
    priority_queue = PriorityQueue()
    deque_back = Deque('back')
    deque_front = Deque('front')
    
    # All these calls work correctly because LSP is followed
    process_data_structure(stack, "Stack (LIFO)")
    process_data_structure(queue, "Queue (FIFO)")
    process_data_structure(priority_queue, "Priority Queue (Highest First)")
    process_data_structure(deque_back, "Deque (Back Mode)")
    process_data_structure(deque_front, "Deque (Front Mode)")
    
    # Demonstrate polymorphic behavior
    demonstrate_polymorphism()
    
    # Demonstrate transfer between different types
    demonstrate_transfer()
    
    # Demonstrate processors working with any data structure
    demonstrate_processors()
    
    print("=== WHY THIS FOLLOWS LSP ===")
    print("1. All implementations honor the DataStructure abstract contract")
    print("2. Client code works correctly with any implementation")
    print("3. Behavioral substitutability is maintained")
    print("4. Exception handling is consistent across implementations")
    print("5. No unexpected side effects or behavior changes")
    print("6. Preconditions are not strengthened in subclasses")
    print("7. Postconditions are not weakened in subclasses")


if __name__ == "__main__":
    main()