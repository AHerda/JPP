use zad3_rust2::gf::gf;
use zad3_rust3::libs::{dhsetup::DHSetup, user::User};

fn main() {
    let a = gf::GF::new(11);
    let m = gf::GF::new(2115);

    let setup = DHSetup::<gf::GF>::new();
    println!("Generator: {}", setup.getGenerator());

    let mut user = User::new(&setup);
    let public_key = user.getPublicKey();
    println!("Public key: {}", public_key);

    user.setKey(a);
    println!("Message: {}", m);

    let encrypted = user.encrypt(m);
    println!("Encrypted: {}", encrypted);

    let decrypted = user.decrypt(encrypted);
    println!("Decrypted: {}", decrypted);
}
