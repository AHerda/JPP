#include <cstdint>
#include <iostream>

#include "DHSetup.hpp"
#include "User.hpp"
//#include "../../lista2/zad1-cpp/gf.hpp"

int main() {
	GF a(11);
    GF m(2115);

    DHSetup<GF> setup;
    std::cout << "generator: " << setup.getGenerator().getValue() << std::endl;

    User<GF> user(setup);
    GF pubKey = user.getPublicKey();
    std::cout << "public key: " << pubKey.getValue() << std::endl;

    user.setKey(a);
    std::cout << "message: " << m.getValue() << std::endl;

    GF c = user.encrypt(m);
    std::cout << "encrypted: " << c.getValue() << std::endl;

    GF mDecrypted = user.decrypt(c);
    std::cout << "decrypted: " << mDecrypted.getValue() << std::endl;

    return 0;
}
