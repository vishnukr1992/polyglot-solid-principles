# Polyglot SOLID Principles

This repository demonstrates the SOLID principles of object-oriented programming across multiple programming languages. Each language implementation shows both violations and correct applications of each principle.

## SOLID Principles

1. **Single Responsibility Principle (SRP)** - A class should have only one reason to change
2. **Open/Closed Principle (OCP)** - Software entities should be open for extension but closed for modification
3. **Liskov Substitution Principle (LSP)** - Objects of a superclass should be replaceable with objects of a subclass
4. **Interface Segregation Principle (ISP)** - Clients should not be forced to depend on interfaces they don't use
5. **Dependency Inversion Principle (DIP)** - High-level modules should not depend on low-level modules

## Supported Languages

- **Java** - Object-oriented implementation with enterprise patterns
- **JavaScript** - Modern ES6+ with functional programming concepts
- **Go** - Structural typing and interface-based design
- **Python** - Dynamic typing with clean, readable code
- **Rust** - Memory-safe systems programming with ownership

## Repository Structure

Each language directory contains subdirectories for each SOLID principle:

```
[language]/
├── single-responsibility/
├── open-closed/
├── liskov-substitution/
├── interface-segregation/
└── dependency-inversion/
```

Each principle directory contains:
- `violation_*` - Code that violates the principle
- `correct_*` - Code that follows the principle correctly
- `README.md` - Explanation and comparison

## How to Navigate

1. Choose your preferred programming language
2. Start with the Single Responsibility Principle
3. Compare the violation and correct implementations
4. Read the README for detailed explanations
5. Progress through each principle

## Getting Started

Clone this repository and explore the examples in your preferred language:

```bash
git clone <repository-url>
cd polyglot-solid-principles
```

Navigate to any language directory and start with the `single-responsibility` folder to begin learning!