"""
Interface Segregation Principle (ISP) - VIOLATION Example (Python)

This example demonstrates how forcing a Robot to implement a Human interface
violates the Interface Segregation Principle.

Problem: Robot is forced to implement biological methods that make no sense
for a robot, leading to empty implementations or exceptions.
"""

from abc import ABC, abstractmethod
from typing import NoReturn


# VIOLATION: Monolithic interface forces all implementations to have methods they don't need
class Human(ABC):
    """Monolithic interface that forces all implementations to have methods they don't need"""
    
    @abstractmethod
    def work(self) -> None:
        """Perform work"""
        pass
    
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
    
    @abstractmethod
    def think(self) -> None:
        """Cognitive function"""
        pass
    
    @abstractmethod
    def reproduce(self) -> None:
        """Reproduction capability"""
        pass
    
    @abstractmethod
    def exercise(self) -> None:
        """Physical exercise"""
        pass
    
    @abstractmethod
    def socialize(self) -> None:
        """Social interaction"""
        pass
    
    @abstractmethod
    def feel_emotions(self) -> None:
        """Emotional experience"""
        pass
    
    @abstractmethod
    def dream(self) -> None:
        """Dream during sleep"""
        pass


class Person(Human):
    """Human implementation - naturally implements all interface methods"""
    
    def __init__(self, name: str):
        self.name = name
        self.energy = 100
        self.is_awake = True
    
    def work(self) -> None:
        if not self.is_awake:
            print(f"{self.name} cannot work while sleeping!")
            return
        print(f"{self.name} is working productively")
        self.energy -= 20
    
    def eat(self) -> None:
        print(f"{self.name} is eating delicious food")
        self.energy += 30
    
    def sleep(self) -> None:
        print(f"{self.name} is sleeping peacefully")
        self.is_awake = False
        self.energy = 100
    
    def breathe(self) -> None:
        print(f"{self.name} is breathing fresh air")
        # Essential biological function
    
    def think(self) -> None:
        print(f"{self.name} is thinking creatively")
        self.energy -= 5
    
    def reproduce(self) -> None:
        print(f"{self.name} can participate in reproduction")
    
    def exercise(self) -> None:
        print(f"{self.name} is doing physical exercise")
        self.energy -= 15
    
    def socialize(self) -> None:
        print(f"{self.name} is socializing with friends")
        self.energy -= 10
    
    def feel_emotions(self) -> None:
        print(f"{self.name} is experiencing complex emotions")
    
    def dream(self) -> None:
        if not self.is_awake:
            print(f"{self.name} is dreaming while sleeping")


class IndustrialRobot(Human):
    """
    VIOLATION: Robot forced to implement Human interface
    This violates ISP because Robot doesn't need most of these methods
    """
    
    def __init__(self, model: str):
        self.model = model
        self.battery_level = 100
        self.is_operational = True
    
    def work(self) -> None:
        if not self.is_operational:
            print(f"{self.model} is not operational!")
            return
        print(f"{self.model} robot is performing industrial work")
        self.battery_level -= 10
    
    # VIOLATION: Forced to implement irrelevant methods
    def eat(self) -> NoReturn:
        # Makes no sense for a robot!
        raise NotImplementedError("Robots don't eat food!")
    
    def sleep(self) -> NoReturn:
        # Robots don't sleep, they power down
        raise NotImplementedError("Robots don't sleep! Use power_down() instead")
    
    def breathe(self) -> NoReturn:
        # Robots don't breathe
        raise NotImplementedError("Robots don't breathe air!")
    
    def think(self) -> None:
        # Different kind of thinking (computational)
        print(f"{self.model} is processing data algorithmically")
        self.battery_level -= 2
    
    def reproduce(self) -> NoReturn:
        # Robots are manufactured, not reproduced
        raise NotImplementedError("Robots are manufactured, not reproduced!")
    
    def exercise(self) -> NoReturn:
        # Robots don't need exercise
        raise NotImplementedError("Robots don't need exercise!")
    
    def socialize(self) -> None:
        # Basic communication only
        print(f"{self.model} is communicating via network protocols")
    
    def feel_emotions(self) -> NoReturn:
        # Robots don't feel emotions
        raise NotImplementedError("Robots don't have emotions!")
    
    def dream(self) -> NoReturn:
        # Robots don't dream
        raise NotImplementedError("Robots don't dream!")
    
    # Robot-specific methods that don't fit in Human interface
    def power_down(self) -> None:
        print(f"{self.model} is powering down systems")
        self.is_operational = False
    
    def recharge(self) -> None:
        print(f"{self.model} is recharging battery")
        self.battery_level = 100
    
    def run_diagnostics(self) -> None:
        print(f"{self.model} is running system diagnostics")
        status = "Operational" if self.is_operational else "Offline"
        print(f"Battery: {self.battery_level}%, Status: {status}")
    
    def execute_program(self, program: str) -> None:
        print(f"{self.model} is executing program: {program}")


class WorkManager:
    """Client code that expects Human interface"""
    
    @staticmethod
    def manage_worker(worker: Human) -> None:
        """This method expects all Human interface methods to work"""
        print("Managing worker...")
        
        # This works for both
        worker.work()
        
        # These fail for Robot!
        try:
            worker.eat()  # Robot will throw exception
        except NotImplementedError as e:
            print(f"Error: {e}")
        
        try:
            worker.sleep()  # Robot will throw exception
        except NotImplementedError as e:
            print(f"Error: {e}")
        
        worker.think()  # Different behavior for robot
        
        try:
            worker.exercise()  # Robot will throw exception
        except NotImplementedError as e:
            print(f"Error: {e}")
        
        try:
            worker.feel_emotions()  # Robot will throw exception
        except NotImplementedError as e:
            print(f"Error: {e}")
    
    @staticmethod
    def assign_work(worker: Human, task: str) -> None:
        """This method only needs work capability but must accept full Human interface"""
        print(f"Assigning task: {task}")
        # We only need work() method, but must accept entire Human interface
        worker.work()
    
    @staticmethod
    def provide_care(human: Human) -> None:
        """Expects Human biological needs"""
        print("Providing human care...")
        try:
            human.eat()
            human.sleep()
            human.breathe()
        except NotImplementedError as e:
            print(f"Failed to provide care: {e}")


def check_human_compliance(obj: object) -> bool:
    """Function that checks if object follows Human interface"""
    required_methods = [
        'work', 'eat', 'sleep', 'breathe', 'think', 
        'reproduce', 'exercise', 'socialize', 'feel_emotions', 'dream'
    ]
    
    return all(hasattr(obj, method) and callable(getattr(obj, method)) for method in required_methods)


def demonstrate_isp_violation() -> None:
    """Demonstrates the ISP violation in Python"""
    print("=== Interface Segregation Principle - VIOLATION (Python) ===")
    print()
    
    print("1. Creating human worker:")
    person = Person("Alice")
    
    print("\n2. Creating robot worker:")
    robot = IndustrialRobot("R2D2-Industrial")
    
    print("\n3. Checking Human interface compliance:")
    print(f"Person follows Human interface: {check_human_compliance(person)}")
    print(f"Robot follows Human interface: {check_human_compliance(robot)}")
    
    print("\n4. Managing human worker (works fine):")
    WorkManager.manage_worker(person)
    
    print("\n5. Managing robot worker (many failures!):")
    WorkManager.manage_worker(robot)
    
    print("\n6. Attempting to provide human care to robot:")
    WorkManager.provide_care(robot)  # This will fail!
    
    print("\n7. Problems with this design:")
    print("   - Robot forced to implement methods it doesn't need")
    print("   - Many methods raise NotImplementedError instead of working")
    print("   - Clients can't rely on interface methods working")
    print("   - Robot-specific methods don't fit Human interface")
    print("   - Violates ISP: clients forced to depend on methods they don't use")
    
    print("\n8. Robot-specific operations (not part of Human interface):")
    robot.power_down()
    robot.recharge()
    robot.run_diagnostics()
    robot.execute_program("Industrial Assembly v2.1")
    
    print("\n=== Conclusion ===")
    print("The monolithic Human interface forces Robot to implement")
    print("irrelevant methods, violating the Interface Segregation Principle.")
    print("Clients depending on Human interface cannot rely on all methods working.")


if __name__ == "__main__":
    demonstrate_isp_violation()