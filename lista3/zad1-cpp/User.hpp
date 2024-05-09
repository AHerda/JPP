#ifndef USRE_HPP
#define USRE_HPP

#include <cstdint>

#include "DHSetup.hpp"

template <typename T>
class User {
private:
	uint64_t secret;
	DHSetup<T> setup;
	T key;
public:
	User(DHSetup<T>& setup_) : setup(setup_) {}

	T getPublicKey() {
		return setup.power(setup.getGenerator(), secret);
	}

	void setKey(T key_) {
		key = setup.power(key_, secret);
	}

	T encrypt(T message) {
		return message * key;
	}

	T decrypt(T cipher) {
		return cipher / key;
	}
};

#endif // USRE_HPP
