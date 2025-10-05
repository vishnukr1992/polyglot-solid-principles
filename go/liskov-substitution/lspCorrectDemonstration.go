package main

import "fmt"

// CORRECT IMPLEMENTATION - FOLLOWS LISKOV SUBSTITUTION PRINCIPLE
//
// This demonstrates proper interface design where all implementations
// can be substituted without breaking the program's correctness.
//
// In Go, LSP is naturally enforced through interface compliance.
// If a type implements an interface, it must honor the behavioral contract.

// DataStructure interface defines the contract
type DataStructure interface {
	Add(element int)
	Remove() (int, error)
	Peek() (int, error)
	Size() int
	IsEmpty() bool
}

// Stack implementation (LIFO)
type Stack struct {
	items []int
}

func NewStack() *Stack {
	return &Stack{items: make([]int, 0)}
}

func (s *Stack) Add(element int) {
	s.items = append(s.items, element)
}

func (s *Stack) Remove() (int, error) {
	if len(s.items) == 0 {
		return 0, fmt.Errorf("stack is empty")
	}

	index := len(s.items) - 1
	element := s.items[index]
	s.items = s.items[:index]
	return element, nil
}

func (s *Stack) Peek() (int, error) {
	if len(s.items) == 0 {
		return 0, fmt.Errorf("stack is empty")
	}
	return s.items[len(s.items)-1], nil
}

func (s *Stack) Size() int {
	return len(s.items)
}

func (s *Stack) IsEmpty() bool {
	return len(s.items) == 0
}

func (s *Stack) String() string {
	return fmt.Sprintf("Stack%v", s.items)
}

// Queue implementation (FIFO)
type Queue struct {
	items []int
}

func NewQueue() *Queue {
	return &Queue{items: make([]int, 0)}
}

func (q *Queue) Add(element int) {
	q.items = append(q.items, element)
}

func (q *Queue) Remove() (int, error) {
	if len(q.items) == 0 {
		return 0, fmt.Errorf("queue is empty")
	}

	element := q.items[0]
	q.items = q.items[1:]
	return element, nil
}

func (q *Queue) Peek() (int, error) {
	if len(q.items) == 0 {
		return 0, fmt.Errorf("queue is empty")
	}
	return q.items[0], nil
}

func (q *Queue) Size() int {
	return len(q.items)
}

func (q *Queue) IsEmpty() bool {
	return len(q.items) == 0
}

func (q *Queue) String() string {
	return fmt.Sprintf("Queue%v", q.items)
}

// PriorityQueue implementation (highest value first)
type PriorityQueue struct {
	items []int
}

func NewPriorityQueue() *PriorityQueue {
	return &PriorityQueue{items: make([]int, 0)}
}

func (pq *PriorityQueue) Add(element int) {
	pq.items = append(pq.items, element)
	// Keep sorted for priority (simple insertion sort)
	for i := len(pq.items) - 1; i > 0; i-- {
		if pq.items[i] > pq.items[i-1] {
			pq.items[i], pq.items[i-1] = pq.items[i-1], pq.items[i]
		} else {
			break
		}
	}
}

func (pq *PriorityQueue) Remove() (int, error) {
	if len(pq.items) == 0 {
		return 0, fmt.Errorf("priority queue is empty")
	}

	element := pq.items[0] // Highest priority (largest number)
	pq.items = pq.items[1:]
	return element, nil
}

func (pq *PriorityQueue) Peek() (int, error) {
	if len(pq.items) == 0 {
		return 0, fmt.Errorf("priority queue is empty")
	}
	return pq.items[0], nil
}

func (pq *PriorityQueue) Size() int {
	return len(pq.items)
}

func (pq *PriorityQueue) IsEmpty() bool {
	return len(pq.items) == 0
}

func (pq *PriorityQueue) String() string {
	return fmt.Sprintf("PriorityQueue%v", pq.items)
}

// Client function that works with any DataStructure implementation
func processDataStructure(ds DataStructure, name string) {
	fmt.Printf("--- Processing %s ---\n", name)

	// Add elements
	ds.Add(10)
	ds.Add(20)
	ds.Add(30)
	fmt.Printf("After adding 10, 20, 30 - Size: %d\n", ds.Size())

	// Peek at next element
	if next, err := ds.Peek(); err == nil {
		fmt.Printf("Next element (peek): %d\n", next)
	}

	// Remove elements
	if element, err := ds.Remove(); err == nil {
		fmt.Printf("Removed: %d\n", element)
	}
	fmt.Printf("Size after removal: %d\n", ds.Size())
	fmt.Printf("Is empty: %t\n", ds.IsEmpty())
	fmt.Println()
}

// Utility function that works with any DataStructure
func transferElements(source, target DataStructure, count int) error {
	for i := 0; i < count; i++ {
		if source.IsEmpty() {
			break
		}
		element, err := source.Remove()
		if err != nil {
			return err
		}
		target.Add(element)
	}
	return nil
}

// Generic processor interface
type Processor interface {
	Process(ds DataStructure) error
}

// Concrete processor that counts elements
type ElementCounter struct {
	count int
}

func (ec *ElementCounter) Process(ds DataStructure) error {
	ec.count = 0
	for !ds.IsEmpty() {
		_, err := ds.Remove()
		if err != nil {
			return err
		}
		ec.count++
	}
	return nil
}

func (ec *ElementCounter) GetCount() int {
	return ec.count
}

// Another processor that sums elements
type ElementSummer struct {
	sum int
}

func (es *ElementSummer) Process(ds DataStructure) error {
	es.sum = 0
	for !ds.IsEmpty() {
		element, err := ds.Remove()
		if err != nil {
			return err
		}
		es.sum += element
	}
	return nil
}

func (es *ElementSummer) GetSum() int {
	return es.sum
}

func demonstratePolymorphism() {
	fmt.Println("=== POLYMORPHIC BEHAVIOR DEMONSTRATION ===")

	// Array of different implementations
	structures := []DataStructure{
		NewStack(),
		NewQueue(),
		NewPriorityQueue(),
	}

	names := []string{"Stack", "Queue", "PriorityQueue"}

	// All can be treated the same way
	for i, ds := range structures {
		ds.Add(5)
		ds.Add(1)
		ds.Add(3)

		if next, err := ds.Peek(); err == nil {
			fmt.Printf("%s peek: %d\n", names[i], next)
		}
	}
	fmt.Println()
}

func demonstrateTransfer() {
	fmt.Println("=== TRANSFER OPERATION DEMONSTRATION ===")

	source := NewStack()
	target := NewQueue()

	// Populate source
	source.Add(1)
	source.Add(2)
	source.Add(3)

	fmt.Printf("Before transfer - Source: %s, Target: %s\n", source, target)

	// Transfer 2 elements
	err := transferElements(source, target, 2)
	if err != nil {
		fmt.Printf("Transfer error: %v\n", err)
		return
	}

	fmt.Printf("After transfer - Source: %s, Target: %s\n", source, target)
	fmt.Println()
}

func demonstrateProcessors() {
	fmt.Println("=== PROCESSOR DEMONSTRATION ===")

	// Create different data structures
	stack := NewStack()
	queue := NewQueue()

	// Populate them
	for i := 1; i <= 5; i++ {
		stack.Add(i * 10)
		queue.Add(i * 10)
	}

	// Process with different processors
	counter := &ElementCounter{}
	summer := &ElementSummer{}

	// Count elements in stack
	stackForCount := NewStack()
	for i := 1; i <= 5; i++ {
		stackForCount.Add(i * 10)
	}

	err := counter.Process(stackForCount)
	if err == nil {
		fmt.Printf("Stack element count: %d\n", counter.GetCount())
	}

	// Sum elements in queue
	err = summer.Process(queue)
	if err == nil {
		fmt.Printf("Queue element sum: %d\n", summer.GetSum())
	}
	fmt.Println()
}

func main() {
	fmt.Println("=== LSP CORRECT DEMONSTRATION ===")
	fmt.Println("All implementations can be substituted without breaking correctness")
	fmt.Println()

	// Test all implementations with the same client code
	stack := NewStack()
	queue := NewQueue()
	priorityQueue := NewPriorityQueue()

	// All these calls work correctly because LSP is followed
	processDataStructure(stack, "Stack (LIFO)")
	processDataStructure(queue, "Queue (FIFO)")
	processDataStructure(priorityQueue, "Priority Queue (Highest First)")

	// Demonstrate polymorphic behavior
	demonstratePolymorphism()

	// Demonstrate transfer between different types
	demonstrateTransfer()

	// Demonstrate processors working with any data structure
	demonstrateProcessors()

	fmt.Println("=== WHY THIS FOLLOWS LSP ===")
	fmt.Println("1. All implementations honor the DataStructure interface contract")
	fmt.Println("2. Client code works correctly with any implementation")
	fmt.Println("3. Behavioral substitutability is maintained")
	fmt.Println("4. Interface compliance is enforced at compile time")
	fmt.Println("5. Error handling is consistent across implementations")
	fmt.Println("6. No unexpected side effects or behavior changes")
}
