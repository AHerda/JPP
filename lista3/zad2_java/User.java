package JPP.lista3.zad2_java;

import JPP.lista3.zad2_java.DHSetup;

public class User<T extends Arithmetics> {
	private long secret;
	private DHSetup<T> setup;
	private T key;

	public User(DHSetup<T> setup) {
		this.setup = setup;
	}

	public T getPublicKey() {
		return setup.power(setup.getGenerator(), secret);
	}

	public void setKey(T key_) {
		key = setup.power(key_, secret);
	}

	public T encrypt(T message) {
		return (T) key.mul(message);
	}

	public T decrypt(T cipher) {
		return (T) key.div(cipher);
	}
}
