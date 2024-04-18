#include <iostream>
#include <ostream>

#include "gf.hpp"


int main() {
    GF a(1234560);
    GF b(10);
    GF c = a + b;
    std::cout << c << std::endl;
    c = a - b;
    std::cout << c << std::endl;
    c = a * b;
    std::cout << c << std::endl;
    c = a / b;
    std::cout << c << std::endl;
    return 0;
}
