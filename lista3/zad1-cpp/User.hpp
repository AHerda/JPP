#ifndef USRE_HPP
#define USRE_HPP

#include <cstdint>
#include <optional>

#include "DHSetup.hpp"

template <typename T>
class User {
private:
	uint64_t secret;
	DHSetup<T> setup;
	std::optional<T> key;
public:
	User(DHSetup<T>& setup_) : setup(setup_), secret(rand()){
		std::cout << "Secret: " << this->secret << std::endl;
	}

	T getPublicKey() {
		return setup.power(setup.getGenerator(), secret);
	}

	void setKey(T key_) {
		key = setup.power(key_, secret);
	}

	T encrypt(T message) {
		if (!key) {
			throw std::runtime_error("Key not set");
		}
		return message * key.value();
	}

	T decrypt(T cipher) {
		if (!key) {
			throw std::runtime_error("Key not set");
		}
		return cipher / key.value();
	}
};

#endif // USRE_HPP
