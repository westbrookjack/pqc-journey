#include <iostream>

class Counter {
    int val;
    int incr;
    public:
        Counter(int n, int m): val{n}, incr{m} {}
        Counter(int n): val{n}, incr{1} {}
        Counter(): val{0}, incr{1} {}
        
        int get_value() const { return val; }
        int get_increment() const { return incr; }
        void set_value(int n) { val = n; }
        void set_value() { val = 0; }
        void set_increment(int n) { incr = n; }
        void set_increment() { incr = 1; }
        void reset() {set_value(); set_increment();}
        int tick() { val+=incr; return val; }
};

int main(){
    Counter c;
    c.tick();
    c.tick();
    
    std::cout << "Counter value is: " << c.get_value() << " and increment is: " << c.get_increment() <<".\n";
    c.set_increment(2);
    c.tick();
    std::cout << "Counter value is: " << c.get_value() << " and increment is: " << c.get_increment() <<".\n";
    c.set_value(20);
    std::cout << "Counter value is: " << c.get_value() << " and increment is: " << c.get_increment() <<".\n";
    c.reset();
    std::cout << "Counter value is: " << c.get_value() << " and increment is: " << c.get_increment() <<".\n";
    c.set_increment(2);
    c.tick();
    std::cout << "Counter value is: " << c.get_value() << " and increment is: " << c.get_increment() <<".\n";
    c.set_increment();
    c.tick();
    std::cout << "Counter value is: " << c.get_value() << " and increment is: " << c.get_increment() <<".\n";
}

