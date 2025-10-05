# Liskov Substitution Principle (LSP) - JavaScript Examples

The **Liskov Substitution Principle** states that objects of a superclass should be replaceable with objects of a subclass without altering the correctness of the program.

## LSP in JavaScript Context

JavaScript's **duck typing** and **dynamic nature** make LSP both easier and more dangerous:
- ✅ **Easier**: "If it walks like a duck and quacks like a duck, it's a duck"
- ⚠️ **More dangerous**: Violations only discovered at runtime

## Duck Typing and LSP

```javascript
// If objects have the same methods and behave consistently,
// they can be substituted for each other
function processStack(stack) {
    stack.push(1);
    stack.push(2);
    console.log(stack.pop()); // Should always be 2 (LIFO)
}

// All these work because they follow the same contract
processStack([]); // Native array
processStack(new CustomStack()); // Custom implementation
processStack(objectStack); // Object literal with stack methods
```

## File Structure

### ✅ Correct Implementation

- **`lspCorrectDemonstration.js`**: Shows proper LSP compliance through duck typing

**Key features:**
- **Consistent interfaces**: All stack implementations have same methods
- **Behavioral consistency**: All behave like stacks (LIFO)
- **Polymorphic usage**: Client code works with any implementation
- **No unexpected behavior**: Substitution maintains correctness

```javascript
// Different implementations, same contract
const implementations = [
    [], // Native array
    new CustomStack(), // ES6 class
    createStack(), // Factory function
    objectStack, // Object literal
];

// All work identically
implementations.forEach(stack => {
    processStack(stack); // Same behavior guaranteed
});
```

### ❌ Violation Examples

- **`lspViolationDemonstration.js`**: Shows various LSP violations

**Violations demonstrated:**
1. **Different behavior contracts**: Queue pretending to be stack
2. **Unpredictable behavior**: Random element removal
3. **State-dependent behavior**: Behavior changes based on size
4. **Unexpected exceptions**: Strengthened preconditions
5. **Side effects**: Hidden operations during method calls

```javascript
// LSP Violation: Queue with stack interface
class FakeStack {
    push(x) { this.items.push(x); }
    pop() { return this.items.shift(); } // FIFO instead of LIFO!
}

// Breaks client expectations
function reverseStack(stack) {
    // Expects LIFO behavior, gets FIFO
    // Results in wrong order!
}
```

## JavaScript-Specific LSP Features

### 1. Prototype-based Extension
```javascript
// Extend existing objects
Array.prototype.peek = function() {
    return this[this.length - 1];
};

// Now all arrays support peek - LSP maintained
```

### 2. Mixins for Behavior
```javascript
const StackMixin = {
    push(x) { this.items.push(x); },
    pop() { return this.items.pop(); },
    peek() { return this.items[this.items.length - 1]; }
};

// Apply to any object
Object.assign(myObject, StackMixin);
```

### 3. Higher-Order Functions
```javascript
// Decorator that adds logging without breaking LSP
function withLogging(stackClass) {
    return class extends stackClass {
        push(x) {
            console.log(`Pushing ${x}`);
            return super.push(x);
        }
        pop() {
            const result = super.pop();
            console.log(`Popped ${result}`);
            return result;
        }
    };
}

const LoggingStack = withLogging(CustomStack);
```

### 4. Dynamic Method Addition
```javascript
// Add methods at runtime while maintaining contract
function enhanceStack(stack) {
    if (!stack.size) {
        stack.size = function() {
            return this.items ? this.items.length : this.length;
        };
    }
    return stack;
}
```

## Common JavaScript LSP Violations

### 1. Array-like Objects That Aren't
```javascript
// Violation: Looks like array but doesn't behave like one
const fakeArray = {
    length: 2,
    0: 'a',
    1: 'b',
    push: function(item) {
        // Doesn't update length! Breaks contract
        this[this.length] = item;
    }
};
```

### 2. Promise-like Objects
```javascript
// Violation: Synchronous "then" breaks Promise contract
class FakePromise {
    then(onResolve) {
        // Executes immediately instead of async!
        return onResolve(this.value);
    }
}
```

### 3. Event Emitter Violations
```javascript
// Violation: Different event behavior
class BrokenEmitter extends EventEmitter {
    emit(event, data) {
        // Modifies data before emitting - breaks contract!
        data.timestamp = Date.now();
        super.emit(event, data);
    }
}
```

## Running the Examples

```bash
# Node.js
node lspCorrectDemonstration.js
node lspViolationDemonstration.js

# Browser (include in HTML)
<script src="lspCorrectDemonstration.js"></script>
<script>
    // Call demonstration functions
</script>
```

## Module System Integration

### ES6 Modules
```javascript
// Export implementations
export { CustomStack, LinkedStack, createStack };

// Import and use
import { CustomStack } from './stack-implementations.js';
```

### CommonJS
```javascript
// Export
module.exports = { CustomStack, processStack };

// Import
const { CustomStack } = require('./stack-implementations');
```

## Testing LSP Compliance

```javascript
// Generic test suite for any stack implementation
function testStackContract(createStack) {
    const stack = createStack();
    
    // Test LIFO behavior
    stack.push(1);
    stack.push(2);
    assert(stack.pop() === 2, 'Should remove last added');
    assert(stack.pop() === 1, 'Should remove in LIFO order');
    
    // Test empty behavior
    assert(stack.length === 0 || stack.size() === 0, 'Should be empty');
}

// Test all implementations
testStackContract(() => []);
testStackContract(() => new CustomStack());
testStackContract(() => createStack());
```

## Real-world Applications

- **Express.js middleware**: Functions with same signature
- **React components**: Components with consistent props interface
- **Node.js streams**: Different stream types with same interface
- **Database adapters**: Different databases, same query interface
- **Plugin systems**: Plugins with consistent API

## Best Practices

1. **Define clear contracts** through documentation and examples
2. **Use TypeScript** for compile-time contract checking
3. **Test behavioral contracts** not just method signatures
4. **Favor composition** over complex inheritance
5. **Use duck typing wisely** - ensure behavioral consistency
6. **Document side effects** explicitly

JavaScript's flexibility makes LSP violations easy to introduce but also provides powerful mechanisms for creating compliant, substitutable objects.