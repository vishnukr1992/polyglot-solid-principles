package main

// Transistor interface
type Transistor interface {
	Base(signal float64)     // control input
	Collector(input float64) // main input
	Output() float64         // measured output (Collector - Emitter)
}
