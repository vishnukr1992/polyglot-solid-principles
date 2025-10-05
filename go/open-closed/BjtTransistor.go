package main

type BJT struct {
	baseSignal     float64
	collectorInput float64
}

func (b *BJT) Base(signal float64) {
	b.baseSignal = signal
}

func (b *BJT) Collector(input float64) {
	b.collectorInput = input
}

func (b *BJT) Output() float64 {
	// Simulated gain: collector output depends on base signal
	return b.collectorInput * (b.baseSignal * 0.1) // crude amplifier model
}
