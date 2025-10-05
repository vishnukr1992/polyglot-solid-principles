/**
 * VIOLATION OF LISKOV SUBSTITUTION PRINCIPLE
 * 
 * This demonstrates how LSP can be violated in JavaScript when objects
 * that appear to have the same interface behave differently in ways
 * that break client expectations.
 */

console.log("=== LSP VIOLATION DEMONSTRATION ===");
console.log("Objects with same interface but different behavior contracts");
console.log();

// Base class that defines expected behavior
class DataStructure {
    constructor() {
        this.items = [];
    }
    
    add(element) {
        this.items.push(element);
    }
    
    remove() {
        return this.items.pop();
    }
    
    size() {
        return this.items.length;
    }
    
    isEmpty() {
        return this.items.length === 0;
    }
}

// Stack implementation (correct)
class Stack extends DataStructure {
    remove() {
        return this.items.pop(); // LIFO - correct stack behavior
    }
}

// Queue implementation (violates LSP when used as DataStructure)
class Queue extends DataStructure {
    remove() {
        return this.items.shift(); // FIFO - breaks DataStructure contract!
    }
}

// Random access structure (major LSP violation)
class RandomStructure extends DataStructure {
    remove() {
        if (this.items.length === 0) return undefined;
        // Removes random element - completely unpredictable!
        const randomIndex = Math.floor(Math.random() * this.items.length);
        return this.items.splice(randomIndex, 1)[0];
    }
}

// Broken structure that changes behavior based on size
class InconsistentStructure extends DataStructure {
    remove() {
        // Behavior changes based on size - violates LSP!
        if (this.items.length <= 2) {
            return this.items.shift(); // FIFO when small
        } else {
            return this.items.pop(); // LIFO when large
        }
    }
}

// Structure that throws errors unexpectedly
class ExceptionThrowingStructure extends DataStructure {
    add(element) {
        if (typeof element === 'string') {
            throw new Error("Strings not allowed!"); // Strengthened precondition!
        }
        super.add(element);
    }
    
    remove() {
        if (this.items.length === 1) {
            throw new Error("Cannot remove last element!"); // Unexpected behavior!
        }
        return super.remove();
    }
}

// Client function that expects consistent DataStructure behavior
function processDataStructure(ds, name) {
    console.log(`--- Processing ${name} ---`);
    
    try {
        // Add elements
        ds.add(10);
        ds.add(20);
        ds.add(30);
        
        console.log(`Added 10, 20, 30. Size: ${ds.size()}`);
        
        // Remove elements and expect consistent order
        const first = ds.remove();
        const second = ds.remove();
        
        console.log(`First removed: ${first}`);
        console.log(`Second removed: ${second}`);
        console.log(`Remaining size: ${ds.size()}`);
        
        // This expectation might be broken by LSP violations
        console.log(`Expected last element: 10, Actual: ${ds.items[0] || 'none'}`);
        
    } catch (error) {
        console.log(`ERROR: ${error.message}`);
    }
    
    console.log();
}

// Test all implementations
const structures = [
    new Stack(),
    new Queue(), // LSP violation: different removal order
    new RandomStructure(), // LSP violation: unpredictable behavior
    new InconsistentStructure(), // LSP violation: behavior changes
    new ExceptionThrowingStructure() // LSP violation: unexpected exceptions
];

const names = [
    "Stack (Correct)",
    "Queue (LSP Violation - FIFO instead of LIFO)",
    "Random Structure (LSP Violation - Unpredictable)",
    "Inconsistent Structure (LSP Violation - Behavior Changes)",
    "Exception Throwing (LSP Violation - Unexpected Errors)"
];

structures.forEach((structure, index) => {
    processDataStructure(structure, names[index]);
});

console.log("=== DEMONSTRATION OF BROKEN CLIENT CODE ===");

// Client code that works with Stack but breaks with others
function reverseUsingDataStructure(ds) {
    const original = [...ds.items];
    const temp = [];
    
    // Move all elements to temporary storage
    while (!ds.isEmpty()) {
        temp.push(ds.remove());
    }
    
    // Put them back (should reverse for stack)
    temp.forEach(item => ds.add(item));
    
    console.log(`Original: [${original}]`);
    console.log(`After reverse operation: [${ds.items}]`);
    
    return ds.items;
}

console.log("Reverse operation with different structures:");

const testStack = new Stack();
testStack.add(1);
testStack.add(2);
testStack.add(3);
console.log("With Stack:");
reverseUsingDataStructure(testStack);
console.log();

const testQueue = new Queue();
testQueue.add(1);
testQueue.add(2);
testQueue.add(3);
console.log("With Queue (LSP Violation):");
reverseUsingDataStructure(testQueue);
console.log();

console.log("=== REAL-WORLD JAVASCRIPT LSP VIOLATIONS ===");

// Example 1: Array-like objects that don't behave like arrays
const fakeArray = {
    length: 3,
    0: 'a',
    1: 'b',
    2: 'c',
    push: function(item) {
        // Doesn't actually increase length!
        this[this.length] = item;
        // Missing: this.length++; 
    },
    pop: function() {
        // Returns wrong element!
        return this[0]; // Should return this[--this.length]
    }
};

console.log("Fake Array object:");
console.log("Initial:", fakeArray);
fakeArray.push('d');
console.log("After push('d'):", fakeArray);
console.log("Pop result:", fakeArray.pop()); // Should be 'd', but returns 'a'
console.log();

// Example 2: Promise-like objects that don't follow Promise semantics
class FakePromise {
    constructor(executor) {
        this.value = null;
        this.resolved = false;
        try {
            executor(
                (value) => {
                    this.value = value;
                    this.resolved = true;
                },
                (error) => {
                    this.value = error;
                    this.resolved = false;
                }
            );
        } catch (e) {
            this.value = e;
            this.resolved = false;
        }
    }
    
    then(onResolve, onReject) {
        // LSP Violation: Executes synchronously instead of async!
        if (this.resolved) {
            return onResolve ? onResolve(this.value) : this.value;
        } else {
            return onReject ? onReject(this.value) : this.value;
        }
    }
}

console.log("Promise-like object violation:");
const fakePromise = new FakePromise(resolve => resolve("test"));
console.log("Before then()");
fakePromise.then(value => console.log("Value:", value)); // Executes immediately!
console.log("After then()"); // This should print first with real Promise
console.log();

console.log("=== WHY THESE VIOLATE LSP ===");
console.log("1. Subtypes don't honor the behavioral contract of their supertypes");
console.log("2. Client code written for the base type fails with substituted types");
console.log("3. Preconditions are strengthened (throwing unexpected errors)");
console.log("4. Postconditions are weakened (unpredictable return values)");
console.log("5. Invariants are not maintained (behavior changes based on state)");
console.log("6. History constraints are violated (inconsistent behavior over time)");

// Export for module usage
if (typeof module !== 'undefined' && module.exports) {
    module.exports = {
        DataStructure,
        Stack,
        Queue,
        RandomStructure,
        InconsistentStructure,
        ExceptionThrowingStructure,
        processDataStructure,
        reverseUsingDataStructure
    };
}