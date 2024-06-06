use rand::{rngs::ThreadRng, Rng};
use std::sync::{Arc, Mutex, MutexGuard};
pub struct Filozof {
    pub id: usize,
    pub lewy: Arc<Mutex<()>>,
    pub prawy: Arc<Mutex<()>>,
    pub do_zjedzenia: usize,
}

impl Filozof {
    pub fn new(
        id: usize,
        lewy: Arc<Mutex<()>>,
        prawy: Arc<Mutex<()>>,
        do_zjedzenia: usize,
    ) -> Self {
        Self {
            id,
            lewy,
            prawy,
            do_zjedzenia,
        }
    }

    fn wez_lewy(&self) {//-> MutexGuard<()> {
        println!("{}[{}] bierze lewy widelec", " ".repeat(self.id), self.id);
        //self.lewy.lock().unwrap()
    }

    fn odloz_lewy(&self, lewy: MutexGuard<()>) {
        println!("{}[{}] odkłada lewy widelec", " ".repeat(self.id), self.id);
        drop(lewy);
    }

    fn wez_prawy(&self) {//-> MutexGuard<()> {
        println!("{}[{}] bierze prawy widelec", " ".repeat(self.id), self.id);
        //self.prawy.lock().unwrap()
    }

    fn odloz_prawy(&self, prawy: MutexGuard<()>) {
        println!("{}[{}] odkłada prawy widelec", " ".repeat(self.id), self.id);
        drop(prawy);
    }

    fn jedz(&self, rng: &mut rand::rngs::ThreadRng) {
        println!(
            "{}[{}] je (zostało {} obiadów)",
            " ".repeat(self.id),
            self.id,
            self.do_zjedzenia - 1
        );
        std::thread::sleep(std::time::Duration::from_millis(
            rng.gen_range(0..1000),
        ));
    }

    fn najedzony(&self) -> bool {
        self.do_zjedzenia == 0
    }

    fn mysl(&self, rng: &mut rand::rngs::ThreadRng) {
        println!("{}[{}] mysli", " ".repeat(self.id), self.id);
        std::thread::sleep(std::time::Duration::from_millis(
            rng.gen_range(0..1000),
        ));
    }

    pub fn zjedz_wszystko(&mut self, mut rng: ThreadRng) {
        while !self.najedzony() {
            self.mysl(&mut rng);

            self.wez_lewy();
            let _lewy = self.lewy.lock().unwrap();
            self.wez_prawy();
            let _prawy = self.prawy.lock().unwrap();

            self.jedz(&mut rng);
            self.do_zjedzenia -= 1;
            self.odloz_lewy(_lewy);
            self.odloz_prawy(_prawy);
        }
    }
}
