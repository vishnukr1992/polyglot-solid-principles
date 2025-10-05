"""
CORRECT IMPLEMENTATION - FOLLOWS OPEN-CLOSED PRINCIPLE
This approach uses abstract base classes to allow extension without modification
"""

from abc import ABC, abstractmethod
from typing import List


class Transistor(ABC):
    """Abstract base class for all transistor types"""
    
    @abstractmethod
    def base(self, signal: float) -> None:
        """Control input"""
        pass
    
    @abstractmethod
    def collector(self, input_value: float) -> None:
        """Main input"""
        pass
    
    @abstractmethod
    def output(self) -> float:
        """Measured output (Collector - Emitter)"""
        pass


class BJTTransistor(Transistor):
    """Bipolar Junction Transistor implementation"""
    
    def __init__(self):
        self.base_signal = 0.0
        self.collector_input = 0.0
    
    def base(self, signal: float) -> None:
        self.base_signal = signal
    
    def collector(self, input_value: float) -> None:
        self.collector_input = input_value
    
    def output(self) -> float:
        # Simulated gain: collector output depends on base signal
        return self.collector_input * (self.base_signal * 0.1)  # crude amplifier model


class FETTransistor(Transistor):
    """Field Effect Transistor implementation - extends without modifying existing code"""
    
    def __init__(self):
        self.gate_voltage = 0.0
        self.drain_current = 0.0
    
    def base(self, signal: float) -> None:
        self.gate_voltage = signal
    
    def collector(self, input_value: float) -> None:
        self.drain_current = input_value
    
    def output(self) -> float:
        # FET specific behavior - different from BJT
        return self.drain_current * (self.gate_voltage * 0.15)


class MOSFETTransistor(Transistor):
    """MOSFET implementation - another extension without modifying existing code"""
    
    def __init__(self):
        self.gate_voltage = 0.0
        self.drain_current = 0.0
        self.threshold = 0.7
    
    def base(self, signal: float) -> None:
        self.gate_voltage = signal
    
    def collector(self, input_value: float) -> None:
        self.drain_current = input_value
    
    def output(self) -> float:
        # MOSFET specific behavior with threshold
        if self.gate_voltage > self.threshold:
            return self.drain_current * (self.gate_voltage - self.threshold) * 0.2
        return 0.0


class AmplifierCircuit:
    """Amplifier circuit that follows OCP"""
    
    def __init__(self):
        self.transistors: List[Transistor] = []
    
    def add_transistor(self, transistor: Transistor) -> None:
        """This method doesn't need to change when new transistor types are added"""
        if not isinstance(transistor, Transistor):
            raise TypeError("Must be a Transistor instance")
        self.transistors.append(transistor)
    
    def amplify_signal(self, signal: float, input_value: float) -> float:
        """This method works with any Transistor implementation"""
        total_output = 0.0
        
        for transistor in self.transistors:
            transistor.base(signal)
            transistor.collector(input_value)
            total_output += transistor.output()
        
        return total_output


def demonstrate_correct_ocp():
    """Example usage demonstrating OCP compliance"""
    print("=== CORRECT OCP IMPLEMENTATION ===")
    print("This code follows the Open-Closed Principle")
    print("New transistor types can be added without modifying existing code")
    print()
    
    # Create amplifier circuit
    amplifier = AmplifierCircuit()
    
    # Add different transistor types
    amplifier.add_transistor(BJTTransistor())
    amplifier.add_transistor(FETTransistor())
    amplifier.add_transistor(MOSFETTransistor())
    
    # Test the amplifier
    signal = 2.0
    input_value = 5.0
    
    output = amplifier.amplify_signal(signal, input_value)
    print(f"Input Signal: {signal:.2f}")
    print(f"Input Current: {input_value:.2f}")
    print(f"Amplified Output: {output:.2f}")
    
    print()
    print("Benefits of this approach:")
    print("1. New transistor types can be added without modifying existing code")
    print("2. Each transistor type is responsible for its own behavior")
    print("3. Polymorphism enables flexible design")
    print("4. Follows Single Responsibility Principle")


if __name__ == "__main__":
    demonstrate_correct_ocp()