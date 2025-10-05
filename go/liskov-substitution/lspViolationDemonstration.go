package main

import (
	"fmt"
	"math/rand"
	"time"
)

// VIOLATION OF LISKOV SUBSTITUTION PRINCIPLE
//
// This demonstrates how LSP can be violated in Go when types
// that implement the same interface behave in ways that break
// client expectations.

// DataStructureViolation interface defines the expected contract
type DataStructureViolation interface {
	Add(element int)
	Remove() (int, error)
	Peek() (int, error)
	Size() int
	IsEmpty() bool
}

// CorrectStack implementation (baseline)
type CorrectStack struct {
	items []int
}

func NewCorrectStack() *CorrectStack {
	return &CorrectStack{items: make([]int, 0)}
}

func (s *CorrectStack) Add(element int) {
	s.items = append(s.items, element)
}

func (s *CorrectStack) Remove() (int, error) {
	if len(s.items) == 0 {
		return 0, fmt.Errorf("stack is empty")
	}

	index := len(s.items) - 1
	element := s.items[index]
	s.items = s.items[:index]
	return element, nil
}

func (s *CorrectStack) Peek() (int, error) {
	if len(s.items) == 0 {
		return 0, fmt.Errorf("stack is empty")
	}
	return s.items[len(s.items)-1], nil
}

func (s *CorrectStack) Size() int {
	return len(s.items)
}

func (s *CorrectStack) IsEmpty() bool {
	return len(s.items) == 0
}

func (s *CorrectStack) String() string {
	return fmt.Sprintf("CorrectStack%v", s.items)
}

// VIOLATION 1: Queue that breaks LIFO expectation
type MislabeledQueue struct {
	items []int
}

func NewMislabeledQueue() *MislabeledQueue {
	return &MislabeledQueue{items: make([]int, 0)}
}

func (q *MislabeledQueue) Add(element int) {
	q.items = append(q.items, element)
}

// LSP VIOLATION: Removes from front instead of back!
func (q *MislabeledQueue) Remove() (int, error) {
	if len(q.items) == 0 {
		return 0, fmt.Errorf("queue is empty")
	}

	element := q.items[0] // FIFO instead of LIFO!
	q.items = q.items[1:]
	return element, nil
}

// LSP VIOLATION: Peeks at wrong end!
func (q *MislabeledQueue) Peek() (int, error) {
	if len(q.items) == 0 {
		return 0, fmt.Errorf("queue is empty")
	}
	return q.items[0], nil // Should peek at end for stack behavior!
}

func (q *MislabeledQueue) Size() int {
	return len(q.items)
}

func (q *MislabeledQueue) IsEmpty() bool {
	return len(q.items) == 0
}

func (q *MislabeledQueue) String() string {
	return fmt.Sprintf("MislabeledQueue%v", q.items)
}

// VIOLATION 2: Random removal structure
type RandomStructure struct {
	items []int
	rand  *rand.Rand
}

func NewRandomStructure() *RandomStructure {
	return &RandomStructure{
		items: make([]int, 0),
		rand:  rand.New(rand.NewSource(time.Now().UnixNano())),
	}
}

func (r *RandomStructure) Add(element int) {
	r.items = append(r.items, element)
}

// LSP VIOLATION: Removes random element - completely unpredictable!
func (r *RandomStructure) Remove() (int, error) {
	if len(r.items) == 0 {
		return 0, fmt.Errorf("structure is empty")
	}

	index := r.rand.Intn(len(r.items))
	element := r.items[index]

	// Remove element at random index
	r.items = append(r.items[:index], r.items[index+1:]...)
	return element, nil
}

// LSP VIOLATION: Random peek too!
func (r *RandomStructure) Peek() (int, error) {
	if len(r.items) == 0 {
		return 0, fmt.Errorf("structure is empty")
	}
	index := r.rand.Intn(len(r.items))
	return r.items[index], nil // Random element!
}

func (r *RandomStructure) Size() int {
	return len(r.items)
}

func (r *RandomStructure) IsEmpty() bool {
	return len(r.items) == 0
}

func (r *RandomStructure) String() string {
	return fmt.Sprintf("RandomStructure%v", r.items)
}

// VIOLATION 3: Structure that changes behavior based on size
type InconsistentStructure struct {
	items []int
}

func NewInconsistentStructure() *InconsistentStructure {
	return &InconsistentStructure{items: make([]int, 0)}
}

func (i *InconsistentStructure) Add(element int) {
	i.items = append(i.items, element)
}

// LSP VIOLATION: Behavior changes based on state!
func (i *InconsistentStructure) Remove() (int, error) {
	if len(i.items) == 0 {
		return 0, fmt.Errorf("structure is empty")
	}

	// Inconsistent behavior based on size
	if len(i.items) <= 2 {
		// FIFO when small
		element := i.items[0]
		i.items = i.items[1:]
		return element, nil
	} else {
		// LIFO when large
		index := len(i.items) - 1
		element := i.items[index]
		i.items = i.items[:index]
		return element, nil
	}
}

// LSP VIOLATION: Peek behavior also changes!
func (i *InconsistentStructure) Peek() (int, error) {
	if len(i.items) == 0 {
		return 0, fmt.Errorf("structure is empty")
	}
	if len(i.items) <= 2 {
		return i.items[0], nil // FIFO peek
	} else {
		return i.items[len(i.items)-1], nil // LIFO peek
	}
}

func (i *InconsistentStructure) Size() int {
	return len(i.items)
}

func (i *InconsistentStructure) IsEmpty() bool {
	return len(i.items) == 0
}

func (i *InconsistentStructure) String() string {
	return fmt.Sprintf("InconsistentStructure%v", i.items)
}

// VIOLATION 4: Structure with strengthened preconditions
type RestrictiveStructure struct {
	items []int
}

func NewRestrictiveStructure() *RestrictiveStructure {
	return &RestrictiveStructure{items: make([]int, 0)}
}

// LSP VIOLATION: Strengthened precondition - rejects negative numbers!
func (r *RestrictiveStructure) Add(element int) {
	if element < 0 {
		panic("RestrictiveStructure does not accept negative numbers!")
	}
	r.items = append(r.items, element)
}

// LSP VIOLATION: Fails when there's only one element
func (r *RestrictiveStructure) Remove() (int, error) {
	if len(r.items) == 0 {
		return 0, fmt.Errorf("structure is empty")
	}

	if len(r.items) == 1 {
		return 0, fmt.Errorf("cannot remove last element") // Unexpected behavior!
	}

	index := len(r.items) - 1
	element := r.items[index]
	r.items = r.items[:index]
	return element, nil
}

// LSP VIOLATION: Peek also fails with one element
func (r *RestrictiveStructure) Peek() (int, error) {
	if len(r.items) == 0 {
		return 0, fmt.Errorf("structure is empty")
	}
	if len(r.items) == 1 {
		return 0, fmt.Errorf("cannot peek at last element")
	}
	return r.items[len(r.items)-1], nil
}

func (r *RestrictiveStructure) Size() int {
	return len(r.items)
}

func (r *RestrictiveStructure) IsEmpty() bool {
	return len(r.items) == 0
}

func (r *RestrictiveStructure) String() string {
	return fmt.Sprintf("RestrictiveStructure%v", r.items)
}

// VIOLATION 5: Structure that modifies during iteration
type ModifyingStructure struct {
	items       []int
	removeCount int
}

func NewModifyingStructure() *ModifyingStructure {
	return &ModifyingStructure{items: make([]int, 0)}
}

func (m *ModifyingStructure) Add(element int) {
	m.items = append(m.items, element)
}

// LSP VIOLATION: Has hidden side effects!
func (m *ModifyingStructure) Remove() (int, error) {
	if len(m.items) == 0 {
		return 0, fmt.Errorf("structure is empty")
	}

	index := len(m.items) - 1
	element := m.items[index]
	m.items = m.items[:index]

	m.removeCount++

	// Hidden side effect: removes extra element every 3rd removal!
	if m.removeCount%3 == 0 && !m.IsEmpty() {
		index = len(m.items) - 1
		m.items = m.items[:index] // Removes another element silently!
	}

	return element, nil
}

// LSP VIOLATION: Peek has side effects too!
func (m *ModifyingStructure) Peek() (int, error) {
	if len(m.items) == 0 {
		return 0, fmt.Errorf("structure is empty")
	}
	// Side effect: increments a counter even for peek!
	m.removeCount++
	return m.items[len(m.items)-1], nil
}

func (m *ModifyingStructure) Size() int {
	return len(m.items)
}

func (m *ModifyingStructure) IsEmpty() bool {
	return len(m.items) == 0
}

func (m *ModifyingStructure) String() string {
	return fmt.Sprintf("ModifyingStructure%v", m.items)
}

// Client function that expects consistent DataStructure behavior
func processDataStructureViolation(ds DataStructureViolation, name string) {
	fmt.Printf("--- Processing %s ---\n", name)

	defer func() {
		if r := recover(); r != nil {
			fmt.Printf("PANIC: %v\n", r)
		}
	}()

	// Add elements
	ds.Add(10)
	ds.Add(20)
	ds.Add(30)

	fmt.Printf("Added 10, 20, 30. Size: %d\n", ds.Size())

	// Remove elements and expect consistent order
	first, err1 := ds.Remove()
	if err1 != nil {
		fmt.Printf("Error removing first: %v\n", err1)
	} else {
		fmt.Printf("First removed: %d\n", first)
	}

	second, err2 := ds.Remove()
	if err2 != nil {
		fmt.Printf("Error removing second: %v\n", err2)
	} else {
		fmt.Printf("Second removed: %d\n", second)
	}

	fmt.Printf("Size after removals: %d\n", ds.Size())
	fmt.Printf("Expected remaining: 10, Actual size: %d\n", ds.Size())
	fmt.Println()
}

// Function that tries to reverse elements (works with proper stack)
func reverseElements(ds DataStructureViolation) {
	fmt.Printf("Attempting to reverse %s\n", ds)

	defer func() {
		if r := recover(); r != nil {
			fmt.Printf("PANIC during reverse: %v\n", r)
		}
	}()

	temp := make([]int, 0)

	// Remove all elements
	for !ds.IsEmpty() {
		element, err := ds.Remove()
		if err != nil {
			fmt.Printf("Error during reverse: %v\n", err)
			return
		}
		temp = append(temp, element)
	}

	// Add them back (should reverse for stack)
	for _, element := range temp {
		ds.Add(element)
	}

	fmt.Printf("After reverse operation: %s\n", ds)
}

func demonstrateViolations() {
	fmt.Println("=== TESTING WITH PROBLEMATIC IMPLEMENTATIONS ===")

	structures := []DataStructureViolation{
		NewCorrectStack(),          // Correct baseline
		NewMislabeledQueue(),       // FIFO instead of LIFO
		NewRandomStructure(),       // Random removal
		NewInconsistentStructure(), // Behavior changes
		NewRestrictiveStructure(),  // Strengthened preconditions
		NewModifyingStructure(),    // Hidden side effects
	}

	names := []string{
		"CorrectStack (Correct)",
		"MislabeledQueue (FIFO Violation)",
		"RandomStructure (Unpredictable)",
		"InconsistentStructure (State-dependent)",
		"RestrictiveStructure (Precondition Violation)",
		"ModifyingStructure (Side Effects)",
	}

	for i, ds := range structures {
		processDataStructureViolation(ds, names[i])
	}
}

func demonstrateBrokenClientCode() {
	fmt.Println("=== DEMONSTRATING BROKEN CLIENT CODE ===")

	structures := []DataStructureViolation{
		NewCorrectStack(),
		NewMislabeledQueue(),
		NewInconsistentStructure(),
	}

	names := []string{
		"CorrectStack (works correctly)",
		"MislabeledQueue (broken)",
		"InconsistentStructure (unpredictable)",
	}

	for i, ds := range structures {
		// Add same elements
		ds.Add(1)
		ds.Add(2)
		ds.Add(3)

		fmt.Printf("\nReverse test with %s:\n", names[i])
		fmt.Printf("Before: %s\n", ds)
		reverseElements(ds)
	}
}

func demonstratePreConditionViolation() {
	fmt.Println("\n=== PRECONDITION VIOLATION DEMONSTRATION ===")

	restrictive := NewRestrictiveStructure()

	fmt.Println("Testing RestrictiveStructure with negative number:")
	fmt.Println("Adding 5, 10...")
	restrictive.Add(5)
	restrictive.Add(10)

	fmt.Println("Trying to add -3 (should panic):")
	restrictive.Add(-3) // This will panic!
}

func mainViolation() {
	fmt.Println("=== LSP VIOLATION DEMONSTRATION ===")
	fmt.Println("Objects implementing same interface but violating behavioral contracts")
	fmt.Println()

	demonstrateViolations()
	demonstrateBrokenClientCode()

	fmt.Println("\n=== WHY THESE VIOLATE LSP ===")
	fmt.Println("1. MislabeledQueue: Changes removal order (FIFO vs LIFO)")
	fmt.Println("2. RandomStructure: Unpredictable behavior breaks client expectations")
	fmt.Println("3. InconsistentStructure: Behavior changes based on internal state")
	fmt.Println("4. RestrictiveStructure: Strengthens preconditions and adds restrictions")
	fmt.Println("5. ModifyingStructure: Hidden side effects violate behavioral contract")
	fmt.Println("6. Client code written for base interface fails with these implementations")
	fmt.Println()

	// This will demonstrate precondition violation with panic
	fmt.Println("Final demonstration (will panic):")
	demonstratePreConditionViolation()
}
