# Open-Closed Principle (OCP) - JavaScript Examples

The **Open-Closed Principle** states that software entities should be **open for extension but closed for modification**.

## JavaScript Implementation Approach

JavaScript uses **ES6 classes**, **prototypal inheritance**, and **duck typing** to implement OCP through polymorphism.

## File Structure

### ✅ Correct Implementation (Follows OCP)

- **`transistor.js`**: Contains base Transistor class and all implementations

```javascript
// Base class acts as interface
class Transistor {
    constructor() {
        if (this.constructor === Transistor) {
            throw new Error("Cannot instantiate abstract class");
        }
    }
    
    base(signal) { throw new Error("Must implement"); }
    collector(input) { throw new Error("Must implement"); }
    output() { throw new Error("Must implement"); }
}
```

**Key JavaScript Features Used:**
- **ES6 Classes**: Modern class syntax with inheritance
- **Abstract base class pattern**: Prevents direct instantiation
- **Duck typing**: "If it walks like a duck and quacks like a duck..."
- **Polymorphism**: Same interface, different implementations

**Benefits:**
```javascript
// Adding new transistor types requires no changes to existing code
class JFETTransistor extends Transistor {
    base(signal) { /* JFET-specific logic */ }
    collector(input) { /* JFET-specific logic */ }
    output() { /* JFET-specific logic */ }
}

// Amplifier works with any Transistor implementation
amplifySignal(signal, input) {
    for (const transistor of this.transistors) {
        transistor.base(signal);
        transistor.collector(input);
        totalOutput += transistor.output();
    }
}
```

### ❌ Violation Example

- **`transistorOCPViolation.js`**: Shows how NOT to implement OCP

**Problems:**
```javascript
// This switch statement violates OCP
switch (transistor.transistorType) {
    case TransistorTypes.BJT:
        // BJT logic
        break;
    case TransistorTypes.FET:
        // FET logic - requires modifying this method
        break;
    // Adding new types requires more cases here
}
```

## JavaScript-Specific OCP Features

### 1. Dynamic Nature
```javascript
// Can add methods dynamically
Transistor.prototype.newMethod = function() {
    // Extension without modification
};
```

### 2. Mixins
```javascript
// Extend functionality through mixins
const AmplifierMixin = {
    amplify() {
        return this.output() * this.gain;
    }
};

Object.assign(BJTTransistor.prototype, AmplifierMixin);
```

### 3. Higher-Order Functions
```javascript
// Extend behavior through function composition
const withLogging = (transistorClass) => {
    return class extends transistorClass {
        output() {
            const result = super.output();
            console.log(`Output: ${result}`);
            return result;
        }
    };
};

const LoggingBJT = withLogging(BJTTransistor);
```

## Running the Examples

```bash
# Run correct implementation
node transistor.js

# Run violation example
node transistorOCPViolation.js

# In browser
# Include scripts and call demonstration functions
```

## Module System Integration

### CommonJS
```javascript
module.exports = {
    Transistor,
    BJTTransistor,
    AmplifierCircuit
};
```

### ES6 Modules
```javascript
export { Transistor, BJTTransistor, AmplifierCircuit };
```

### Browser Usage
```html
<script src="transistor.js"></script>
<script>
    const amplifier = new AmplifierCircuit();
    amplifier.addTransistor(new BJTTransistor());
</script>
```

## Modern JavaScript Patterns

### 1. Factory Functions
```javascript
const createTransistor = (type) => {
    const transistorMap = {
        'BJT': () => new BJTTransistor(),
        'FET': () => new FETTransistor(),
        'MOSFET': () => new MOSFETTransistor()
    };
    
    return transistorMap[type]?.() || null;
};
```

### 2. Strategy Pattern with Objects
```javascript
const transistorStrategies = {
    BJT: {
        base: (signal) => { /* BJT logic */ },
        output: () => { /* BJT output */ }
    },
    FET: {
        base: (signal) => { /* FET logic */ },
        output: () => { /* FET output */ }
    }
};
```

### 3. Plugin Architecture
```javascript
class AmplifierCircuit {
    constructor() {
        this.plugins = new Map();
    }
    
    use(name, plugin) {
        this.plugins.set(name, plugin);
    }
    
    executePlugin(name, ...args) {
        return this.plugins.get(name)?.(args);
    }
}
```

## Real-world JavaScript Applications

- **Express.js middleware**: Plugins that extend functionality
- **Webpack loaders**: Transform files without modifying core
- **React components**: Composable UI elements
- **Node.js streams**: Transformable data pipelines
- **Vue.js plugins**: Extend framework capabilities

JavaScript's dynamic nature makes it particularly well-suited for implementing OCP through various patterns and paradigms.