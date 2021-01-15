mod lidar_scanner;
extern crate ctrlc;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn main() {
    let mut scanner = lidar_scanner::LidarScanner::new("/dev/ttyUSB0", 230400);

    let should_cont = Arc::new(AtomicBool::new(true));
    let s = should_cont.clone();
    ctrlc::set_handler( move || s.store(false, Ordering::SeqCst) );

    while should_cont.load(Ordering::SeqCst) {
        let data = scanner.poll();

        for i in 0..20 {
            for j in 0..18 {
                print!("{}: {}, ", i*18 + j, data[i*18 + j])
            }
            println!()
        }
    }
}
