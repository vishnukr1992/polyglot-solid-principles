"""
Interface Segregation Principle (ISP) - CORRECT Example (Python)

This example demonstrates proper interface segregation where clients
only depend on the methods they actually need.

Solution: Break down the monolithic Human interface into smaller,
focused interfaces that can be implemented independently.
"""

from abc import ABC, abstractmethod
from typing import Optional, List


# CORRECT: Small, focused interfaces following ISP
class Workable(ABC):
    """Interface for entities that can perform work"""
    
    @abstractmethod
    def work(self) -> None:
        """Perform work"""
        pass


class Biological(ABC):
    """Interface for living organisms"""
    
    @abstractmethod
    def eat(self) -> None:
        """Consume food"""
        pass
    
    @abstractmethod
    def sleep(self) -> None:
        """Rest and recover"""
        pass
    
    @abstractmethod
    def breathe(self) -> None:
        """Breathing function"""
        pass


class Cognitive(ABC):
    """Interface for thinking entities"""
    
    @abstractmethod
    def think(self) -> None:
        """Cognitive function"""
        pass
    
    @abstractmethod
    def learn(self) -> None:
        """Learning capability"""
        pass


class Social(ABC):
    """Interface for entities that can interact socially"""
    
    @abstractmethod
    def socialize(self) -> None:
        """Social interaction"""
        pass
    
    @abstractmethod
    def communicate(self) -> None:
        """Communication capability"""
        pass


class Physical(ABC):
    """Interface for entities with physical capabilities"""
    
    @abstractmethod
    def exercise(self) -> None:
        """Physical exercise"""
        pass
    
    @abstractmethod
    def move(self) -> None:
        """Movement capability"""
        pass


class Reproductive(ABC):
    """Interface for entities that can reproduce"""
    
    @abstractmethod
    def reproduce(self) -> None:
        """Reproduction capability"""
        pass


# Robot-specific interfaces
class Mechanical(ABC):
    """Interface for mechanical devices"""
    
    @abstractmethod
    def power_down(self) -> None:
        """Power down the device"""
        pass
    
    @abstractmethod
    def power_up(self) -> None:
        """Power up the device"""
        pass
    
    @abstractmethod
    def run_diagnostics(self) -> None:
        """Run system diagnostics"""
        pass


class Rechargeable(ABC):
    """Interface for rechargeable devices"""
    
    @abstractmethod
    def recharge(self) -> None:
        """Recharge the device"""
        pass
    
    @abstractmethod
    def get_battery_level(self) -> int:
        """Get current battery level"""
        pass


class Programmable(ABC):
    """Interface for programmable devices"""
    
    @abstractmethod
    def load_program(self, program: str) -> None:
        """Load a program"""
        pass
    
    @abstractmethod
    def execute_program(self) -> None:
        """Execute the loaded program"""
        pass


class NetworkEnabled(ABC):
    """Interface for network-enabled devices"""
    
    @abstractmethod
    def connect_to_network(self) -> None:
        """Connect to network"""
        pass
    
    @abstractmethod
    def send_data(self, data: str) -> None:
        """Send data over network"""
        pass
    
    @abstractmethod
    def receive_data(self) -> str:
        """Receive data from network"""
        pass


# Person implementation - implements relevant interfaces
class Person(Workable, Biological, Cognitive, Social, Physical, Reproductive):
    """Human implementation - implements relevant interfaces"""
    
    def __init__(self, name: str):
        self.name = name
        self.energy = 100
        self.is_awake = True
        self.current_task: Optional[str] = None
    
    # Workable interface
    def work(self) -> None:
        if not self.is_awake:
            print(f"{self.name} cannot work while sleeping!")
            return
        task = self.current_task or "general tasks"
        print(f"{self.name} is working on: {task}")
        self.energy -= 20
    
    # Biological interface
    def eat(self) -> None:
        print(f"{self.name} is eating nutritious food")
        self.energy += 30
    
    def sleep(self) -> None:
        print(f"{self.name} is sleeping peacefully")
        self.is_awake = False
        self.energy = 100
    
    def breathe(self) -> None:
        print(f"{self.name} is breathing fresh air")
    
    # Cognitive interface
    def think(self) -> None:
        print(f"{self.name} is thinking creatively and analytically")
        self.energy -= 5
    
    def learn(self) -> None:
        print(f"{self.name} is learning new skills")
        self.energy -= 10
    
    # Social interface
    def socialize(self) -> None:
        print(f"{self.name} is socializing with friends")
        self.energy -= 10
    
    def communicate(self) -> None:
        print(f"{self.name} is communicating through speech")
    
    # Physical interface
    def exercise(self) -> None:
        print(f"{self.name} is doing physical exercise")
        self.energy -= 15
    
    def move(self) -> None:
        print(f"{self.name} is walking around")
        self.energy -= 5
    
    # Reproductive interface
    def reproduce(self) -> None:
        print(f"{self.name} can participate in human reproduction")
    
    def set_current_task(self, task: str) -> None:
        self.current_task = task


# IndustrialRobot - only implements interfaces it needs
class IndustrialRobot(Workable, Mechanical, Rechargeable, Programmable):
    """Basic Robot - only implements interfaces it needs"""
    
    def __init__(self, model: str):
        self.model = model
        self.battery_level = 100
        self.is_operational = True
        self.current_program: Optional[str] = None
    
    # Workable interface
    def work(self) -> None:
        if not self.is_operational:
            print(f"{self.model} is not operational!")
            return
        if not self.current_program:
            print(f"{self.model} has no program loaded!")
            return
        print(f"{self.model} robot is executing work program: {self.current_program}")
        self.battery_level -= 10
    
    # Mechanical interface
    def power_down(self) -> None:
        print(f"{self.model} is powering down all systems")
        self.is_operational = False
    
    def power_up(self) -> None:
        print(f"{self.model} is powering up systems")
        self.is_operational = True
    
    def run_diagnostics(self) -> None:
        print(f"{self.model} is running comprehensive diagnostics")
        status = "Operational" if self.is_operational else "Offline"
        print(f"Battery: {self.battery_level}%, Status: {status}")
    
    # Rechargeable interface
    def recharge(self) -> None:
        print(f"{self.model} is recharging battery to full capacity")
        self.battery_level = 100
    
    def get_battery_level(self) -> int:
        return self.battery_level
    
    # Programmable interface
    def load_program(self, program: str) -> None:
        print(f"{self.model} is loading program: {program}")
        self.current_program = program
    
    def execute_program(self) -> None:
        if not self.current_program:
            print(f"{self.model} has no program to execute!")
            return
        print(f"{self.model} is executing program: {self.current_program}")
        self.battery_level -= 5


# HumanoidRobot - implements more interfaces including some human-like ones
class HumanoidRobot(Workable, Cognitive, Social, Physical, Mechanical, 
                   Rechargeable, Programmable, NetworkEnabled):
    """Advanced Humanoid Robot - implements more interfaces including some human-like ones"""
    
    def __init__(self, name: str):
        self.name = name
        self.battery_level = 100
        self.is_operational = True
        self.current_program: Optional[str] = None
        self.network_connected = False
    
    # Workable interface
    def work(self) -> None:
        if not self.is_operational:
            print(f"{self.name} is not operational!")
            return
        print(f"{self.name} humanoid robot is performing complex work tasks")
        self.battery_level -= 8
    
    # Cognitive interface - robots can have AI cognition
    def think(self) -> None:
        print(f"{self.name} is processing information using AI algorithms")
        self.battery_level -= 3
    
    def learn(self) -> None:
        print(f"{self.name} is updating neural networks with new data")
        self.battery_level -= 5
    
    # Social interface - humanoid robots can be social
    def socialize(self) -> None:
        print(f"{self.name} is engaging in social interaction protocols")
        self.battery_level -= 4
    
    def communicate(self) -> None:
        print(f"{self.name} is communicating through speech synthesis")
    
    # Physical interface - humanoid robots can move
    def move(self) -> None:
        print(f"{self.name} is walking with bipedal locomotion")
        self.battery_level -= 6
    
    def exercise(self) -> None:
        print(f"{self.name} is performing calibration movements")
        self.battery_level -= 7
    
    # Mechanical interface
    def power_down(self) -> None:
        print(f"{self.name} humanoid is entering sleep mode")
        self.is_operational = False
    
    def power_up(self) -> None:
        print(f"{self.name} humanoid is activating all systems")
        self.is_operational = True
    
    def run_diagnostics(self) -> None:
        print(f"{self.name} is running humanoid system diagnostics")
        network = "Connected" if self.network_connected else "Disconnected"
        print(f"Battery: {self.battery_level}%, Network: {network}")
    
    # Rechargeable interface
    def recharge(self) -> None:
        print(f"{self.name} is recharging via induction pad")
        self.battery_level = 100
    
    def get_battery_level(self) -> int:
        return self.battery_level
    
    # Programmable interface
    def load_program(self, program: str) -> None:
        print(f"{self.name} is loading behavioral program: {program}")
        self.current_program = program
    
    def execute_program(self) -> None:
        if not self.current_program:
            print(f"{self.name} has no behavioral program loaded!")
            return
        print(f"{self.name} is executing behavioral program: {self.current_program}")
        self.battery_level -= 3
    
    # NetworkEnabled interface
    def connect_to_network(self) -> None:
        print(f"{self.name} is connecting to robot network")
        self.network_connected = True
    
    def send_data(self, data: str) -> None:
        if not self.network_connected:
            print(f"{self.name} is not connected to network!")
            return
        print(f"{self.name} is sending data: {data}")
    
    def receive_data(self) -> str:
        if not self.network_connected:
            print(f"{self.name} is not connected to network!")
            return ""
        print(f"{self.name} is receiving network data")
        return "network_data_received"


# Client classes that only depend on interfaces they need
class WorkManager:
    """Client that only depends on Workable interface"""
    
    @staticmethod
    def assign_task(worker: Workable, task: str) -> None:
        """Only depends on Workable interface - works with any worker type"""
        print(f"Assigning task: {task}")
        
        # Type-specific task assignment
        if isinstance(worker, Person):
            worker.set_current_task(task)
        elif isinstance(worker, Programmable):
            worker.load_program(task)
        
        worker.work()
    
    @staticmethod
    def manage_workforce(workers: List[Workable]) -> None:
        """Only needs workers, doesn't care about other capabilities"""
        print(f"Managing workforce of {len(workers)} workers")
        for worker in workers:
            worker.work()


class MaintenanceManager:
    """Client that only depends on Mechanical and Rechargeable interfaces"""
    
    @staticmethod
    def perform_maintenance(device: Mechanical) -> None:
        """Only depends on Mechanical interface"""
        print("Starting maintenance routine...")
        device.run_diagnostics()
        device.power_down()
        print("Maintenance completed")
        device.power_up()
    
    @staticmethod
    def manage_charge(device: Rechargeable) -> None:
        """Only depends on Rechargeable interface"""
        if device.get_battery_level() < 20:
            print("Battery low, initiating recharge")
            device.recharge()
        else:
            print(f"Battery sufficient: {device.get_battery_level()}%")


class EducationManager:
    """Client that only depends on Cognitive interface"""
    
    @staticmethod
    def conduct_training(learner: Cognitive) -> None:
        """Only depends on Cognitive interface"""
        print("Starting cognitive training session...")
        learner.think()
        learner.learn()


class SocialCoordinator:
    """Client that only depends on Social interface"""
    
    @staticmethod
    def organize_social_event(participants: List[Social]) -> None:
        """Only depends on Social interface"""
        print("Organizing social interaction...")
        for participant in participants:
            participant.socialize()
            participant.communicate()


class BiologicalCareProvider:
    """Client that only depends on Biological interface"""
    
    @staticmethod
    def provide_care(organism: Biological) -> None:
        """Only depends on Biological interface"""
        print("Providing biological care...")
        organism.eat()
        organism.sleep()
        organism.breathe()


def get_implemented_interfaces(obj: object) -> List[str]:
    """Helper function to get interfaces implemented by an object"""
    interfaces = []
    interface_classes = [
        Workable, Biological, Cognitive, Social, Physical, Reproductive,
        Mechanical, Rechargeable, Programmable, NetworkEnabled
    ]
    
    for interface_class in interface_classes:
        if isinstance(obj, interface_class):
            interfaces.append(interface_class.__name__)
    
    return interfaces


def demonstrate_isp_correct() -> None:
    """Demonstrates proper Interface Segregation in Python"""
    print("=== Interface Segregation Principle - CORRECT (Python) ===")
    print()
    
    # Create different types of workers
    alice = Person("Alice")
    factory_bot = IndustrialRobot("FactoryBot-3000")
    android = HumanoidRobot("Android-Sara")
    
    print("1. Analyzing interface implementation:")
    print(f"Alice implements: {', '.join(get_implemented_interfaces(alice))}")
    print(f"FactoryBot implements: {', '.join(get_implemented_interfaces(factory_bot))}")
    print(f"Android implements: {', '.join(get_implemented_interfaces(android))}")
    
    print("\n2. Work Management (only needs Workable interface):")
    WorkManager.assign_task(alice, "Design new product")
    WorkManager.assign_task(factory_bot, "Assembly line task")
    WorkManager.assign_task(android, "Customer service")
    
    print("\n3. Managing workforce (polymorphic with Workable):")
    WorkManager.manage_workforce([alice, factory_bot, android])
    
    print("\n4. Maintenance (only for Mechanical devices):")
    # Note: Cannot pass alice here - she doesn't implement Mechanical!
    MaintenanceManager.perform_maintenance(factory_bot)
    MaintenanceManager.perform_maintenance(android)
    
    print("\n5. Battery Management (only for Rechargeable devices):")
    # Note: Cannot pass alice here - she doesn't implement Rechargeable!
    MaintenanceManager.manage_charge(factory_bot)
    MaintenanceManager.manage_charge(android)
    
    print("\n6. Cognitive Training (for thinking entities):")
    EducationManager.conduct_training(alice)
    EducationManager.conduct_training(android)  # Humanoid can think too!
    # Note: Cannot pass factory_bot - it doesn't implement Cognitive!
    
    print("\n7. Social Coordination (for social entities):")
    SocialCoordinator.organize_social_event([alice, android])
    # Note: Cannot pass factory_bot - it doesn't implement Social!
    
    print("\n8. Biological Care (only for living organisms):")
    BiologicalCareProvider.provide_care(alice)
    # Note: Cannot pass robots - they don't implement Biological!
    
    print("\n9. Network Operations (only for NetworkEnabled):")
    if isinstance(android, NetworkEnabled):
        android.connect_to_network()
        android.send_data("Status update")
        android.receive_data()
    
    print("\n=== Benefits of Interface Segregation ===")
    print("✓ Clients only depend on methods they actually use")
    print("✓ No forced implementation of irrelevant methods")
    print("✓ High cohesion within each interface")
    print("✓ Easy to extend with new capabilities")
    print("✓ Better testability and maintainability")
    print("✓ Follows Single Responsibility at interface level")
    print("✓ Python's multiple inheritance works naturally with segregated interfaces")
    
    print("\n=== Interface Composition Examples ===")
    print("Person implements: Workable + Biological + Cognitive + Social + Physical + Reproductive")
    print("IndustrialRobot implements: Workable + Mechanical + Rechargeable + Programmable")
    print("HumanoidRobot implements: Workable + Cognitive + Social + Physical + Mechanical + Rechargeable + Programmable + NetworkEnabled")
    
    print("\n10. Testing interface constraints:")
    print("✓ WorkManager.assign_task works with any Workable")
    print("✓ MaintenanceManager.perform_maintenance only accepts Mechanical")
    print("✓ EducationManager.conduct_training only accepts Cognitive")
    print("✓ BiologicalCareProvider.provide_care only accepts Biological")
    print("✓ Each client is decoupled from irrelevant functionality")


if __name__ == "__main__":
    demonstrate_isp_correct()