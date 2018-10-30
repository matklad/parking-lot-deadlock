extern crate parking_lot;

use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;

use parking_lot::RwLock;

fn main() {
    let data = Arc::new(RwLock::new(()));
    let flag = Arc::new(AtomicBool::new(false));
    let N = 100_000;
    let mut upgraders = Vec::new();
    for _ in 0..10 {
        let t = {
            let data = Arc::clone(&data);
            let flag = flag.clone();
            std::thread::spawn(move || {
                for _ in 0..N {
                    let guard = data.upgradable_read();
                    while flag.load(SeqCst) {}
                    drop(guard);
                }
            })
        };
        upgraders.push(t);
    }

    for i in 0..N {
        eprintln!("i = {:?}", i);
        flag.store(true, SeqCst);
        let read = data.read();
        flag.store(false, SeqCst);
        drop(read);
    }
    println!("reader done");
    upgraders.into_iter().for_each(|it| drop(it.join()));
    println!("upgrader locked");
}
