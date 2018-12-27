extern crate parking_lot;

use std::sync::Arc;

use parking_lot::{RwLock, RwLockUpgradableReadGuard};

fn main() {
    let data = Arc::new(RwLock::new(0u64));
    const N: usize = 1_000_000;
    let mut upgraders = Vec::new();
    for _ in 0..8 {
        let t = {
            let data = Arc::clone(&data);
            std::thread::spawn(move || {
                for i in 0..N {
                    let guard = data.upgradable_read();
                    if i % 2 == 0 {
                        let mut guard = RwLockUpgradableReadGuard::upgrade(guard);
                        *guard += 1;
                    } else {
                        drop(guard);
                        let mut guard = data.write();
                        *guard += 1;
                    }
                    if i % 10 == 9 {
                        assert!(*data.read() > 1)
                    }
                }
            })
        };
        upgraders.push(t);
    }

    eprintln!("waiting for threads to finish...");
    upgraders.into_iter().for_each(|it| drop(it.join()));
    eprintln!("done, final value: {}", *data.read());
}
