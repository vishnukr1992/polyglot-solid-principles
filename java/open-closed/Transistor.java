// CORRECT IMPLEMENTATION - FOLLOWS OPEN-CLOSED PRINCIPLE
// This interface allows extension without modification

public interface Transistor {
    void base(double signal);      // control input
    void collector(double input);  // main input  
    double output();               // measured output (Collector - Emitter)
}