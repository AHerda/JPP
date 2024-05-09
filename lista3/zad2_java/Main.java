package JPP.lista3.zad2_java;

public class Main {
	public static void main(String[] args) {
		GF a = new GF(11);
		GF m = new GF(2115);

		DHSetup<GF> setup = new DHSetup<GF>(GF::new);
		System.out.println("generator: " + setup.getGenerator());

		User<GF> user = new User<>(setup);
		GF pubKey = user.getPublicKey();
		System.out.println("public key: " + pubKey);

		user.setKey(a);
		System.out.println("message: " + m);

		GF c = user.encrypt(m);
		System.out.println("encrypted: " + c);

		GF mDecrypted = user.decrypt(c);
		System.out.println("decrypted: " + mDecrypted);
	}
}
