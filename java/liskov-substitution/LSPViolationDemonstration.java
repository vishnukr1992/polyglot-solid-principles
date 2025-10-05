import java.util.*;

/**
 * VIOLATION OF LISKOV SUBSTITUTION PRINCIPLE
 * 
 * This demonstrates how Java's Stack class violates LSP by extending Vector.
 * Stack promises LIFO behavior, but since it extends Vector, it inherits
 * methods that can break this contract.
 * 
 * LSP Violation: Stack is-a Vector, but it doesn't behave like one
 * when it comes to the expected stack semantics.
 */
public class LSPViolationDemonstration {
    
    /**
     * This method expects Vector behavior (remove from front)
     * but doesn't know it might receive a Stack
     */
    public static Vector<Integer> takeFirstOut(Vector<Integer> vector) {
        System.out.println("Taking first element out: " + vector.remove(0)); // Removes from bottom!
        return vector;
    }
    
    /**
     * This method expects to work with any Collection
     * but Stack's inherited behavior can be confusing
     */
    public static void processAsCollection(Collection<Integer> collection) {
        System.out.println("Collection size: " + collection.size());
        System.out.println("Contains 20: " + collection.contains(20));
        
        // This works, but it's not stack-like behavior
        collection.remove(Integer.valueOf(20)); // Removes from middle!
        System.out.println("After removing 20, size: " + collection.size());
    }
    
    /**
     * Another method that expects List behavior
     */
    public static void accessByIndex(List<Integer> list) {
        System.out.println("Element at index 1: " + list.get(1));
        list.set(0, 999); // Direct index manipulation breaks stack semantics
        System.out.println("After setting index 0 to 999");
    }
    
    public static void main(String[] args) {
        System.out.println("=== LSP VIOLATION DEMONSTRATION ===");
        System.out.println("Stack extends Vector, but violates expected stack behavior");
        System.out.println();
        
        // Create a stack with elements
        Stack<Integer> stack = new Stack<>();
        stack.add(10); // Element at index 0 (Bottom of stack)
        stack.add(20); // Element at index 1
        stack.add(30); // Element at index 2 (Top of stack)
        
        System.out.println("--- Initial Stack State ---");
        System.out.println("Bottom of stack (index 0): " + stack.get(0)); // 10
        System.out.println("Top of stack (peek): " + stack.peek()); // 30
        System.out.println("Stack toString: " + stack);
        System.out.println();
        
        // Problem 1: Using Stack as Vector breaks stack semantics
        System.out.println("--- Problem 1: Vector method breaks stack contract ---");
        Vector<Integer> vectorView = stack; // Legal upcast
        var modifiedStack = takeFirstOut(vectorView); // Removes from BOTTOM!
        
        Stack<Integer> stackAgain = (Stack<Integer>) modifiedStack;
        System.out.println("After removing 'first' element:");
        System.out.println("Expected top (pop): should be 30, got " + stackAgain.pop()); // Still 30
        System.out.println("Next element: should be 20, got " + stackAgain.pop()); // 20
        System.out.println();
        
        // Reset stack
        stack.clear();
        stack.addAll(Arrays.asList(10, 20, 30));
        
        // Problem 2: Collection methods can break stack contract
        System.out.println("--- Problem 2: Collection methods break stack contract ---");
        processAsCollection(stack); // Removes from middle!
        System.out.println("Stack after collection processing: " + stack);
        System.out.println();
        
        // Reset stack
        stack.clear();
        stack.addAll(Arrays.asList(10, 20, 30));
        
        // Problem 3: List methods allow random access
        System.out.println("--- Problem 3: List methods allow random access ---");
        accessByIndex(stack); // Direct index manipulation!
        System.out.println("Stack after index manipulation: " + stack);
        System.out.println("Top element is now: " + stack.peek()); // Not what we expect!
        System.out.println();
        
        // Problem 4: Iterator doesn't follow stack order
        System.out.println("--- Problem 4: Iterator doesn't follow stack semantics ---");
        stack.clear();
        stack.addAll(Arrays.asList(10, 20, 30));
        System.out.print("Iterator order (should be LIFO for stack): ");
        for (Integer item : stack) {
            System.out.print(item + " "); // Prints in FIFO order: 10 20 30
        }
        System.out.println();
        
        System.out.print("Actual stack order (using pop): ");
        Stack<Integer> tempStack = (Stack<Integer>) stack.clone();
        while (!tempStack.isEmpty()) {
            System.out.print(tempStack.pop() + " "); // Prints in LIFO order: 30 20 10
        }
        System.out.println();
        System.out.println();
        
        System.out.println("=== WHY THIS VIOLATES LSP ===");
        System.out.println("1. Stack is-a Vector, but doesn't behave like one");
        System.out.println("2. Methods expecting Vector behavior get unexpected results");
        System.out.println("3. Stack's LIFO contract is broken by inherited Vector methods");
        System.out.println("4. Code written for Vector cannot safely use Stack as substitute");
        System.out.println("5. Behavioral substitutability is broken, not just structural");
    }
}