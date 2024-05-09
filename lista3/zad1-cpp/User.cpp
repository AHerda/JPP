#include "User.hpp"

template <typename T>
User<T>::User(DHSetup<T>& setup_) : setup(setup_) {}

template <typename T>
T User<T>::getPublicKey() {
	return setup.power(setup.getGenerator(), secret);
}

template <typename T>
void User<T>::setKey(T key_) {
	key = setup.power(key_, secret);
}

template <typename T>
T User<T>::encrypt(T message) {
	return message * key;
}

template <typename T>
T User<T>::decrypt(T cipher) {
	return cipher / key;
}
