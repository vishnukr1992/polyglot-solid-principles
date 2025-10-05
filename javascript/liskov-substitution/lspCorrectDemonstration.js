/**
 * CORRECT IMPLEMENTATION - FOLLOWS LISKOV SUBSTITUTION PRINCIPLE
 * 
 * JavaScript demonstrates LSP through duck typing and consistent interfaces.
 * If objects have the same methods and behave consistently, they can be
 * substituted for each other.
 * 
 * Key principle: "If it walks like a duck and quacks like a duck, it's a duck"
 */

// Client function that works with any stack-like object
function processStack(stack) {
    console.log(`Processing ${stack.constructor?.name || 'stack-like object'}`);
    
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    console.log(`After pushing 1, 2, 3:`);
    console.log(`Size: ${stack.size ? stack.size() : stack.length || 'unknown'}`);
    
    // Always behaves like a stack (LIFO)
    console.log(`Pop: ${stack.pop()}`); // Should be 3
    console.log(`Pop: ${stack.pop()}`); // Should be 2
    console.log(`Peek: ${stack.peek ? stack.peek() : stack[stack.length - 1]}`); // Should be 1
    
    console.log(`Final size: ${stack.size ? stack.size() : stack.length || 'unknown'}`);
    console.log('---');
}

// Array as a stack (built-in JavaScript behavior)
console.log("=== LSP SUCCESS DEMONSTRATION ===");
console.log("Different stack implementations that follow the same contract");
console.log();

console.log("1. Using native Array as stack:");
const arrayStack = [];
processStack(arrayStack);

// Custom stack implementation with explicit methods
console.log("2. Using custom Stack class:");
class CustomStack {
    constructor() {
        this.items = [];
    }
    
    push(element) {
        this.items.push(element);
    }
    
    pop() {
        return this.items.pop();
    }
    
    peek() {
        return this.items[this.items.length - 1];
    }
    
    size() {
        return this.items.length;
    }
    
    isEmpty() {
        return this.items.length === 0;
    }
}

const customStack = new CustomStack();
processStack(customStack);

// Another custom implementation using different internal structure
console.log("3. Using LinkedStack implementation:");
class LinkedStack {
    constructor() {
        this.top = null;
        this._size = 0;
    }
    
    push(element) {
        const node = {
            data: element,
            next: this.top
        };
        this.top = node;
        this._size++;
    }
    
    pop() {
        if (!this.top) return undefined;
        
        const data = this.top.data;
        this.top = this.top.next;
        this._size--;
        return data;
    }
    
    peek() {
        return this.top ? this.top.data : undefined;
    }
    
    size() {
        return this._size;
    }
    
    isEmpty() {
        return this._size === 0;
    }
}

const linkedStack = new LinkedStack();
processStack(linkedStack);

// Object literal that implements stack interface
console.log("4. Using object literal with stack methods:");
const objectStack = {
    items: [],
    push(x) { 
        this.items.push(x); 
    },
    pop() { 
        return this.items.pop(); 
    },
    peek() { 
        return this.items[this.items.length - 1]; 
    },
    size() { 
        return this.items.length; 
    }
};

processStack(objectStack);

// Function-based stack (closure)
console.log("5. Using closure-based stack:");
function createStack() {
    let items = [];
    
    return {
        push(element) {
            items.push(element);
        },
        pop() {
            return items.pop();
        },
        peek() {
            return items[items.length - 1];
        },
        size() {
            return items.length;
        }
    };
}

const closureStack = createStack();
processStack(closureStack);

// Demonstrating polymorphic behavior
console.log("=== POLYMORPHIC BEHAVIOR DEMONSTRATION ===");

const stacks = [
    [],
    new CustomStack(),
    new LinkedStack(),
    objectStack,
    createStack()
];

console.log("Processing array of different stack implementations:");
stacks.forEach((stack, index) => {
    console.log(`Stack ${index + 1}:`);
    
    // All behave the same way
    stack.push(99);
    const popped = stack.pop();
    console.log(`Pushed 99, then popped: ${popped}`);
});

console.log();

// Advanced example: Stack with middleware
console.log("=== ADVANCED: STACK WITH MIDDLEWARE ===");

class LoggingStack {
    constructor(baseStack) {
        this.base = baseStack;
    }
    
    push(element) {
        console.log(`  Logging: Pushing ${element}`);
        this.base.push(element);
    }
    
    pop() {
        const result = this.base.pop();
        console.log(`  Logging: Popped ${result}`);
        return result;
    }
    
    peek() {
        const result = this.base.peek();
        console.log(`  Logging: Peeked ${result}`);
        return result;
    }
    
    size() {
        return this.base.size();
    }
}

// Wrap any stack implementation with logging
const loggingStack = new LoggingStack(new CustomStack());
console.log("Stack with logging middleware:");
processStack(loggingStack);

// Generic utility functions that work with any stack
console.log("=== UTILITY FUNCTIONS ===");

function reverseStack(stack) {
    const temp = [];
    while (stack.size && stack.size() > 0 || stack.length > 0) {
        temp.push(stack.pop());
    }
    temp.forEach(item => stack.push(item));
}

function transferElements(fromStack, toStack, count) {
    for (let i = 0; i < count; i++) {
        const element = fromStack.pop();
        if (element !== undefined) {
            toStack.push(element);
        }
    }
}

const sourceStack = new CustomStack();
const targetStack = new LinkedStack();

sourceStack.push(10);
sourceStack.push(20);
sourceStack.push(30);

console.log("Before transfer:");
console.log(`Source size: ${sourceStack.size()}`);
console.log(`Target size: ${targetStack.size()}`);

transferElements(sourceStack, targetStack, 2);

console.log("After transferring 2 elements:");
console.log(`Source size: ${sourceStack.size()}`);
console.log(`Target size: ${targetStack.size()}`);
console.log(`Target top: ${targetStack.peek()}`);

console.log();
console.log("=== WHY THIS FOLLOWS LSP ===");
console.log("1. All implementations honor the same behavioral contract");
console.log("2. Duck typing ensures interface compatibility");
console.log("3. Client code works with any conforming implementation");
console.log("4. Substitution doesn't break program correctness");
console.log("5. Behavioral expectations are consistently met");
console.log("6. No strengthened preconditions or weakened postconditions");

// Export for module usage
if (typeof module !== 'undefined' && module.exports) {
    module.exports = {
        CustomStack,
        LinkedStack,
        createStack,
        LoggingStack,
        processStack,
        transferElements,
        reverseStack
    };
}