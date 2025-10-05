import java.util.*;

/**
 * CORRECT IMPLEMENTATION - FOLLOWS LISKOV SUBSTITUTION PRINCIPLE
 * 
 * This demonstrates proper inheritance where subtypes can be substituted
 * for their base types without breaking the program's correctness.
 * 
 * Key LSP principle: Objects of a superclass should be replaceable with
 * objects of a subclass without altering the correctness of the program.
 */
public class LSPCorrectDemonstration {
    
    // Abstract base class defining the contract
    static abstract class DataStructure<T> {
        protected List<T> elements = new ArrayList<>();
        
        public abstract void add(T element);
        public abstract T remove();
        public abstract T peek();
        public abstract boolean isEmpty();
        public abstract int size();
        
        // This method should work with any proper implementation
        public void transferTo(DataStructure<T> other, int count) {
            for (int i = 0; i < count && !this.isEmpty(); i++) {
                T element = this.remove();
                other.add(element);
            }
        }
        
        @Override
        public String toString() {
            return getClass().getSimpleName() + ": " + elements.toString();
        }
    }
    
    // Stack implementation that follows LSP
    static class ProperStack<T> extends DataStructure<T> {
        @Override
        public void add(T element) {
            elements.add(element); // Add to end (top of stack)
        }
        
        @Override
        public T remove() {
            if (isEmpty()) throw new EmptyStackException();
            return elements.remove(elements.size() - 1); // Remove from end (LIFO)
        }
        
        @Override
        public T peek() {
            if (isEmpty()) throw new EmptyStackException();
            return elements.get(elements.size() - 1); // Peek at end
        }
        
        @Override
        public boolean isEmpty() {
            return elements.isEmpty();
        }
        
        @Override
        public int size() {
            return elements.size();
        }
    }
    
    // Queue implementation that follows LSP
    static class ProperQueue<T> extends DataStructure<T> {
        @Override
        public void add(T element) {
            elements.add(element); // Add to end
        }
        
        @Override
        public T remove() {
            if (isEmpty()) throw new NoSuchElementException("Queue is empty");
            return elements.remove(0); // Remove from beginning (FIFO)
        }
        
        @Override
        public T peek() {
            if (isEmpty()) throw new NoSuchElementException("Queue is empty");
            return elements.get(0); // Peek at beginning
        }
        
        @Override
        public boolean isEmpty() {
            return elements.isEmpty();
        }
        
        @Override
        public int size() {
            return elements.size();
        }
    }
    
    // Priority Queue implementation that follows LSP
    static class ProperPriorityQueue<T extends Comparable<T>> extends DataStructure<T> {
        @Override
        public void add(T element) {
            elements.add(element);
            Collections.sort(elements); // Keep sorted for priority
        }
        
        @Override
        public T remove() {
            if (isEmpty()) throw new NoSuchElementException("Priority queue is empty");
            return elements.remove(elements.size() - 1); // Remove highest priority (last after sorting)
        }
        
        @Override
        public T peek() {
            if (isEmpty()) throw new NoSuchElementException("Priority queue is empty");
            return elements.get(elements.size() - 1); // Peek at highest priority
        }
        
        @Override
        public boolean isEmpty() {
            return elements.isEmpty();
        }
        
        @Override
        public int size() {
            return elements.size();
        }
    }
    
    // Client code that works with any DataStructure implementation
    public static void processDataStructure(DataStructure<Integer> ds, String name) {
        System.out.println("--- Processing " + name + " ---");
        
        // Add elements
        ds.add(10);
        ds.add(20);
        ds.add(30);
        System.out.println("After adding 10, 20, 30: " + ds);
        
        // Peek at next element
        System.out.println("Next element (peek): " + ds.peek());
        System.out.println("Size: " + ds.size());
        
        // Remove elements
        System.out.println("Removing: " + ds.remove());
        System.out.println("After removal: " + ds);
        
        // Check if operations maintain the expected contract
        System.out.println("Is empty: " + ds.isEmpty());
        System.out.println("Size after removal: " + ds.size());
        System.out.println();
    }
    
    public static void demonstrateTransfer() {
        System.out.println("=== TRANSFER OPERATION DEMONSTRATION ===");
        
        DataStructure<Integer> stack = new ProperStack<>();
        DataStructure<Integer> queue = new ProperQueue<>();
        
        // Populate stack
        stack.add(1);
        stack.add(2);
        stack.add(3);
        
        System.out.println("Before transfer:");
        System.out.println("Stack: " + stack);
        System.out.println("Queue: " + queue);
        
        // Transfer 2 elements from stack to queue
        stack.transferTo(queue, 2);
        
        System.out.println("After transferring 2 elements from stack to queue:");
        System.out.println("Stack: " + stack);
        System.out.println("Queue: " + queue);
        
        // This works because both implementations properly follow the contract
        System.out.println();
    }
    
    public static void main(String[] args) {
        System.out.println("=== LSP CORRECT DEMONSTRATION ===");
        System.out.println("All implementations can be substituted without breaking correctness");
        System.out.println();
        
        // Test all implementations with the same client code
        DataStructure<Integer> stack = new ProperStack<>();
        DataStructure<Integer> queue = new ProperQueue<>();
        DataStructure<Integer> priorityQueue = new ProperPriorityQueue<>();
        
        // All these calls work correctly because LSP is followed
        processDataStructure(stack, "Stack (LIFO)");
        processDataStructure(queue, "Queue (FIFO)");
        processDataStructure(priorityQueue, "Priority Queue");
        
        // Demonstrate polymorphic behavior
        demonstrateTransfer();
        
        // Show that behavior is predictable and consistent
        System.out.println("=== POLYMORPHIC ARRAY DEMONSTRATION ===");
        DataStructure<Integer>[] structures = {
            new ProperStack<>(),
            new ProperQueue<>(),
            new ProperPriorityQueue<>()
        };
        
        for (DataStructure<Integer> ds : structures) {
            ds.add(5);
            ds.add(1);
            ds.add(3);
            System.out.println(ds.getClass().getSimpleName() + " peek: " + ds.peek());
        }
        
        System.out.println();
        System.out.println("=== WHY THIS FOLLOWS LSP ===");
        System.out.println("1. All subtypes honor the base class contract");
        System.out.println("2. Method preconditions are not strengthened in subtypes");
        System.out.println("3. Method postconditions are not weakened in subtypes");
        System.out.println("4. Base class invariants are preserved in subtypes");
        System.out.println("5. Client code works correctly with any implementation");
        System.out.println("6. Behavioral substitutability is maintained");
    }
}