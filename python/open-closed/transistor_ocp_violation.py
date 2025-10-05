"""
VIOLATION OF OPEN-CLOSED PRINCIPLE
This code violates OCP because:
1. Adding new transistor types requires modifying existing code
2. The AmplifierCircuit is tightly coupled to specific transistor implementations
3. No abstraction layer exists for different transistor types
"""

from enum import Enum
from typing import List, Dict, Any


class TransistorType(Enum):
    """Hard-coded transistor types - violates OCP"""
    BJT = "BJT"
    FET = "FET"
    MOSFET = "MOSFET"


class TransistorViolation:
    """Monolithic transistor class that handles all types - violates SRP too"""
    
    def __init__(self, transistor_type: TransistorType):
        self.transistor_type = transistor_type
        self.base_signal = 0.0
        self.collector_input = 0.0
        self.gate_voltage = 0.0  # for FET types
        self.drain_current = 0.0  # for FET types
    
    def process_signal(self, signal: float, input_value: float) -> float:
        """Violates OCP: Adding new transistor behavior requires modifying this method"""
        self.base_signal = signal
        self.collector_input = input_value
        
        # Hard-coded logic for different types - violates OCP
        if self.transistor_type == TransistorType.BJT:
            # BJT specific processing
            return self.collector_input * (self.base_signal * 0.1)
        elif self.transistor_type == TransistorType.FET:
            # FET specific processing
            self.gate_voltage = signal
            self.drain_current = input_value
            return self.drain_current * (self.gate_voltage * 0.15)
        elif self.transistor_type == TransistorType.MOSFET:
            # MOSFET specific processing
            self.gate_voltage = signal
            self.drain_current = input_value
            threshold = 0.7
            if self.gate_voltage > threshold:
                return self.drain_current * (self.gate_voltage - threshold) * 0.2
            return 0.0
        else:
            return 0.0


class AmplifierCircuitViolation:
    """Amplifier circuit that violates OCP"""
    
    def __init__(self):
        self.transistors: List[TransistorViolation] = []
    
    def add_transistor(self, transistor_type: TransistorType) -> None:
        transistor = TransistorViolation(transistor_type)
        self.transistors.append(transistor)
    
    def amplify_signal(self, signal: float, input_value: float) -> float:
        """Violates OCP: Adding new transistor types requires modifying this method"""
        total_output = 0.0
        
        for transistor in self.transistors:
            # This if-elif chain violates OCP - must be modified for each new type
            if transistor.transistor_type == TransistorType.BJT:
                output = transistor.process_signal(signal, input_value)
                total_output += output
            elif transistor.transistor_type == TransistorType.FET:
                output = transistor.process_signal(signal, input_value) * 1.1  # FET has higher gain
                total_output += output
            elif transistor.transistor_type == TransistorType.MOSFET:
                output = transistor.process_signal(signal, input_value) * 1.2  # MOSFET has highest gain
                total_output += output
        
        return total_output
    
    def get_transistor_info(self) -> List[Dict[str, Any]]:
        """Another method that would need modification for new types - violates OCP"""
        info = []
        
        for transistor in self.transistors:
            # Yet another if-elif chain that violates OCP
            if transistor.transistor_type == TransistorType.BJT:
                description = "Bipolar Junction Transistor - Current controlled"
            elif transistor.transistor_type == TransistorType.FET:
                description = "Field Effect Transistor - Voltage controlled"
            elif transistor.transistor_type == TransistorType.MOSFET:
                description = "Metal Oxide Semiconductor FET - Enhanced mode"
            else:
                description = "Unknown transistor type"
            
            info.append({
                "type": transistor.transistor_type.value,
                "description": description
            })
        
        return info
    
    def calculate_power_consumption(self) -> float:
        """Yet another method requiring modification for new types - violates OCP"""
        total_power = 0.0
        
        for transistor in self.transistors:
            # Another if-elif chain that violates OCP
            if transistor.transistor_type == TransistorType.BJT:
                # BJT power calculation
                power = transistor.base_signal * transistor.collector_input * 0.05
                total_power += power
            elif transistor.transistor_type == TransistorType.FET:
                # FET power calculation
                power = transistor.gate_voltage * transistor.drain_current * 0.03
                total_power += power
            elif transistor.transistor_type == TransistorType.MOSFET:
                # MOSFET power calculation
                if transistor.gate_voltage > 0.7:
                    power = transistor.gate_voltage * transistor.drain_current * 0.02
                else:
                    power = 0.0
                total_power += power
        
        return total_power


def demonstrate_ocp_violation():
    """Example usage demonstrating OCP violation"""
    print("=== OCP VIOLATION EXAMPLE ===")
    print("This code violates the Open-Closed Principle")
    print("Adding new transistor types requires modifying existing code")
    print()
    
    # Create amplifier circuit
    amplifier = AmplifierCircuitViolation()
    
    # Add different transistor types
    amplifier.add_transistor(TransistorType.BJT)
    amplifier.add_transistor(TransistorType.FET)
    amplifier.add_transistor(TransistorType.MOSFET)
    
    # Test the amplifier
    signal = 2.0
    input_value = 5.0
    
    output = amplifier.amplify_signal(signal, input_value)
    print(f"Input Signal: {signal:.2f}")
    print(f"Input Current: {input_value:.2f}")
    print(f"Amplified Output: {output:.2f}")
    
    # Test additional functionality
    power = amplifier.calculate_power_consumption()
    print(f"Power Consumption: {power:.2f}W")
    
    print()
    print("Transistor Information:")
    info = amplifier.get_transistor_info()
    for i, item in enumerate(info, 1):
        print(f"{i}. {item['type']}: {item['description']}")
    
    print()
    print("Problems with this approach:")
    print("1. Adding new transistor types requires modifying amplify_signal method")
    print("2. process_signal method must be extended for each new type")
    print("3. get_transistor_info method needs updates for each new type")
    print("4. calculate_power_consumption method needs updates for each new type")
    print("5. No abstraction - tightly coupled to specific implementations")
    print("6. Violates Single Responsibility Principle as well")
    print("7. Multiple if-elif chains scattered throughout the code")


if __name__ == "__main__":
    demonstrate_ocp_violation()