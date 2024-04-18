#include <iostream>
#include <ostream>

#include "gf.hpp"


int main() {
    GF a(1234560);
    GF b(10);

    std::cout << "a = " << a << std::endl;
    std::cout << "b = " << b << std::endl;

    GF c = a + b;
    std::cout << "\na + b = " << c << std::endl;
    c = a - b;
    std::cout << "a - b = " << c << std::endl;
    c = a * b;
    std::cout << "a * b = " << c << std::endl;
    c = a / b;
    std::cout << "a / b = " << c << std::endl;

    std::cout << "\na == b <=> " << (a == b) << std::endl;
    std::cout << "a != b <=> " << (a != b) << std::endl;
    std::cout << "a < b <=> " << (a < b) << std::endl;
    std::cout << "a > b <=> " << (a > b) << std::endl;
    std::cout << "a <= b <=> " << (a <= b) << std::endl;
    std::cout << "a >= b <=> " << (a >= b) << std::endl;

    auto [prime, n] = a.characteristic();
    std::cout << "\nCharacteristic of a: " << prime << "^" << n << std::endl;
    return 0;
}
