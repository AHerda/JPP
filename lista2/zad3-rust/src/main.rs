use zad3_rust::gf::gf::GF;

fn main() {
    let a = GF::new(1234560);
    let b = GF::new(10);

    println!("a = {}", a.value());
    println!("b = {}\n", b.value());

    let c = a + b;
    println!("a + b = {}", c.value());
    let c = a - b;
    println!("a - b = {}", c.value());
    let c = a * b;
    println!("a * b = {}", c.value());
    let c = a / b;
    println!("a / b = {}\n", c.value());

    println!("a == b <=> {}", a.eq(&b));
    println!("a != b <=> {}", a.ne(&b));
    println!("a < b <=> {}", a.lt(&b));
    println!("a > b <=> {}", a.gt(&b));
    println!("a <= b <=> {}", a.le(&b));
    println!("a >= b <=> {}", a.ge(&b));

    let (prime, n) = GF::characteristic();
    println!("\nCharacteristic of a: {}^{}", prime, n);
}
