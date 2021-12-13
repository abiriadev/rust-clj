use std::{
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

fn main() {
    let sum: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));

    let _ = (0..=100)
        .into_iter()
        .map(|x| (x, sum.clone()))
        .map(|(i, sum)| {
            thread::spawn(move || match sum.lock() {
                Ok(mut v) => *v += i,
                Err(err) => panic!("can't lock mutex: {}", err),
            })
        })
        .map(JoinHandle::join)
        .collect::<Vec<_>>();

    println!("sum: {}", (*sum).lock().unwrap());
}
