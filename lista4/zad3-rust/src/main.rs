use std::sync::{Arc, Mutex};

mod lib;
use lib::filozof::Filozof;

const FILOZOFOWIE: usize = 5;
const DO_ZJEDZENIA: usize = 3;

fn main() {
    // let widelce = vec![Arc::new(Mutex::new(())); FILOZOFOWIE];
    let widelce: Vec<_> = (0..FILOZOFOWIE).map(|_| Arc::new(Mutex::new(()))).collect();
    let mut handles = vec![];

    let filozofowie: Vec<_> = (0..FILOZOFOWIE)
        .map(|i| {
            Filozof::new(
                i,
                widelce[i].clone(),
                widelce[(i + 1) % FILOZOFOWIE].clone(),
                DO_ZJEDZENIA,
            )
        })
        .collect();

    for mut filozof in filozofowie {
        handles.push(std::thread::spawn(move || {
            filozof.zjedz_wszystko(rand::thread_rng());
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
