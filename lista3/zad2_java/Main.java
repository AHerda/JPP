package JPP.lista3.zad2_java;

public class Main {
	public static void main(String[] args) {
		GF a = new GF(11);
		GF m = new GF(2115);

		DHSetup<GF> setup = new DHSetup<GF>(GF::new);
		System.out.println("generator: " + setup.getGenerator().getValue());
		User<GF> user = new User<>(setup);
		GF pubKey = user.getPublicKey();
		System.out.println("public key: " + pubKey.getValue());
		user.setKey(a);
		System.out.println("message: " + m.getValue());
		GF c = user.encrypt(m);
		System.out.println("encrypted: " + c.getValue());
		GF mDecrypted = user.decrypt(c);
		System.out.println("decrypted: " + mDecrypted.getValue());
	}
}
