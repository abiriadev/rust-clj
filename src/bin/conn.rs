fn work() {
    for i in 0..10 {
        println!("new thread: {}", i);

        std::thread::sleep(std::time::Duration::from_millis(300));
    }
}

fn main() {
    // println!("skllksd");
    // let (tx, rx) = std::sync::mpsc::channel();

    // let tx2 = std::sync::mpsc::Sender::clone(&tx);
    // let tx3 = tx2.clone();

    // let h = std::thread::spawn(move || {
    //     let s = String::from("success!");

    //     tx.send(s).unwrap();
    // });

    // let h2 = std::thread::spawn(move || {
    //     for i in 0..10 {
    //         tx2.send(format!("lol2: {:?}", i));
    //     }
    // });

    // for i in 0..5 {
    //     println!("main thread: {}", i);

    //     std::thread::sleep(std::time::Duration::from_millis(300));
    // }

    // let res = rx.recv().unwrap();

    // for res in rx {
    //     println!("rx: {:?}", res);
    // }

    // println!("res: {}", res);

    // let res = h.join();

    let m = std::sync::Mutex::new(5);
    {
        let mut n = m.lock().unwrap();

        *n += 10;
    }

    println!("mutex: {:?}", m.lock().unwrap());
}
