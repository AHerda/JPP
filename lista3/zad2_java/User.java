package JPP.lista3.zad2_java;

import java.util.Random;
import java.util.Optional;

import JPP.lista3.zad2_java.DHSetup;

public class User<T extends Arithmetics> {
	private long secret;
	private DHSetup<T> setup;
	private Optional<T> key;

	public User(DHSetup<T> setup) {
		Random rand = new Random();
		this.setup = setup;
		this.secret = 1 + Math.abs(rand.nextLong()) % (GF.P - 1);
		System.out.println("Secret: " + secret);
	}

	public T getPublicKey() {
		return setup.power(setup.getGenerator(), secret);
	}

	public void setKey(T key_) {
		key = Optional.of(setup.power(key_, secret));
	}

	public T encrypt(T message) {
		if (!key.isPresent()) {
			throw new IllegalStateException("Key not set");
		}
		return (T) message.mul(key.get());
	}

	public T decrypt(T cipher) {
		if (!key.isPresent()) {
			throw new IllegalStateException("Key not set");
		}
		return (T) cipher.div(key.get());
	}
}
